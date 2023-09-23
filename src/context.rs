use std::{
    collections::HashMap,
    num::NonZeroU8,
    sync::{Arc, RwLock},
};

use crossterm::event::{Event, MouseButton, MouseEventKind};

use crate::{
    id::Id,
    input::InputState,
    math_util::{Rect, VecI2},
    memory::Memory,
    response::Response,
    screen::{Screen, ScreenDrain, ScreenIter},
    style::{Color, Style},
    ui::{Layout, Ui},
};

#[derive(Debug, Default)]
pub struct ContextInner {
    memory: Memory,

    input: InputState,

    current: Screen,
    last: Screen,

    previous_frame_report: PreviousFrameReport,

    max_rect: Rect,
    last_reported_screen: Rect,
    resized: bool,

    frame: usize,

    last_event: Option<Event>,

    used_ids: HashMap<Id, Rect>,
}
impl ContextInner {
    fn new(size: VecI2) -> ContextInner {
        let screen = Rect::new_pos_size(VecI2::new(0, 0), size);
        let mut myself = Self {
            max_rect: screen,
            last_reported_screen: screen,
            ..Default::default()
        };
        myself.current.resize(size);
        myself.last.resize(size);
        myself
    }

    pub fn draw(&mut self, str: &str, style: Style, start: VecI2, layer: NonZeroU8, clip: Rect) {
        self.current.push_text(str, style, start, layer, clip)
    }

    pub fn start_frame(&mut self) {
        if self.max_rect != self.last_reported_screen {
            self.max_rect = self.last_reported_screen;
            self.current.resize(self.max_rect.size());
            self.last.resize(self.max_rect.size());
            self.resized = true;
        }
    }

    pub fn get_finished_frame(&mut self) -> FinishedFrame<'_> {
        FinishedFrame {
            resized: self.resized,
            // we want to preserve this frame to allow us to diff it next frame
            current_frame: self.current.iter(),
            // but the last frame needs to be cleared before the next frame starts so we can drain it
            last_frame: self.last.drain(),
        }
    }

    pub fn finish_frame(&mut self, written: usize) {
        let ContextInner { current, last, .. } = self;
        std::mem::swap(last, current);
        self.resized = false;

        self.input.next_state();

        self.used_ids.clear();

        self.previous_frame_report.bytes_written = written;
        self.previous_frame_report.total_styles = self.last.num_styles();
        self.previous_frame_report.total_text_len = self.last.text_len();

        self.frame += 1;
    }
}

pub struct FinishedFrame<'a> {
    pub resized: bool,
    pub current_frame: ScreenIter<'a>,
    pub last_frame: ScreenDrain<'a>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PreviousFrameReport {
    pub bytes_written: usize,
    pub total_text_len: usize,
    pub total_styles: usize,
}

#[derive(Clone, Default)]
pub struct Context {
    inner: Arc<RwLock<ContextInner>>,
}

impl Context {
    pub fn frame(&self, func: impl FnOnce(&mut Ui)) {
        let lock = self.inner.read().unwrap();
        let clip = lock.max_rect;
        drop(lock);
        func(&mut Ui::new(
            self.clone(),
            Layout::TopLeftVertical,
            clip,
            NonZeroU8::new(128).unwrap(),
        ));
    }

    pub fn inner(&mut self) -> &mut Arc<RwLock<ContextInner>> {
        &mut self.inner
    }

    pub fn previous_frame_report(&self) -> PreviousFrameReport {
        self.inner.read().unwrap().previous_frame_report
    }

    pub fn handle_event(&self, event: Event) -> bool {
        let mut lock = self.inner.write().unwrap();
        match event {
            Event::Resize(x, y) => {
                lock.last_reported_screen = Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(x, y));
                false
            }
            Event::Mouse(event) => {
                let mouse = &mut lock.input.mouse;
                let event_pos = VecI2::new(event.column, event.row);
                mouse.position = Some(event_pos);
                match event.kind {
                    MouseEventKind::Down(button)
                    | MouseEventKind::Up(button)
                    | MouseEventKind::Drag(button) => {
                        let button = match button {
                            MouseButton::Left => &mut mouse.buttons[0],
                            MouseButton::Right => &mut mouse.buttons[2],
                            MouseButton::Middle => &mut mouse.buttons[1],
                        };
                        match event.kind {
                            MouseEventKind::Down(_) => {
                                button.button_down(event_pos);
                            }
                            MouseEventKind::Up(_) => {
                                button.button_up(event_pos);
                            }
                            MouseEventKind::Drag(_) => {
                                button.button_dragged(event_pos);
                            }
                            _ => {}
                        }

                        true
                    }
                    MouseEventKind::Moved => false,
                    MouseEventKind::ScrollDown => {
                        mouse.delta_scroll -= 1;
                        false
                    }
                    MouseEventKind::ScrollUp => {
                        mouse.delta_scroll += 1;
                        false
                    }
                }
            }
            _ => false,
        };
        lock.last_event = Some(event);
        true
    }

    pub fn last_event(&self) -> Option<Event> {
        self.inner.read().unwrap().last_event.clone()
    }

    pub fn new(size: VecI2) -> Context {
        Self {
            inner: Arc::new(RwLock::new(ContextInner::new(size))),
        }
    }

    pub fn draw(&self, str: &str, style: Style, start: VecI2, layer: NonZeroU8, clip: Rect) {
        let mut lock = self.inner.write().unwrap();
        lock.current.push_text(str, style, start, layer, clip)
    }

    pub fn interact(&self, _clip: Rect, id: Id, area: Rect) -> Response {
        let lock = self.inner.read().unwrap();
        if let Some(position) = &lock.input.mouse.position {
            if area.contains(*position) {
                let mut response = Response::new(area, id, Some(*position));
                response.buttons = lock.input.mouse.buttons;
                response
            } else {
                Response::new(area, id, None)
            }
        } else {
            Response::new(area, id, None)
        }
    }

    pub fn insert_into_memory<T: Clone + 'static>(&self, id: Id, val: T) {
        self.inner.write().unwrap().memory.insert(id, val);
    }

    pub fn get_memory_or<T: Clone + 'static>(&self, id: Id, default: T) -> T {
        let mut lock = self.inner.write().unwrap();
        lock.memory.get_or(id, default)
    }

    pub fn get_frame(&self) -> usize {
        self.inner.read().unwrap().frame
    }

    pub fn get_memory_or_create<T: Clone + 'static>(
        &self,
        id: Id,
        default: impl FnOnce() -> T,
    ) -> T {
        let mut lock = self.inner.write().unwrap();
        lock.memory.get_or_create(id, default)
    }

    pub fn check_for_id_clash(&self, id: Id, new_rect: Rect) {
        let prev_rect = self.inner.write().unwrap().used_ids.insert(id, new_rect);
        if let Some(prev_rect) = prev_rect {
            if prev_rect == new_rect {
                self.draw(
                    &format!("Double use of {}", id.value() % 1000),
                    Style::new()
                        .background(Color::Red)
                        .set_underlined()
                        .set_rapid_blink(),
                    new_rect.top_left(),
                    NonZeroU8::new(255).unwrap(),
                    Rect::MAX_SIZE,
                )
            } else {
                self.draw(
                    &format!("First use of {}", id.value() % 1000),
                    Style::new()
                        .background(Color::Red)
                        .set_underlined()
                        .set_rapid_blink(),
                    prev_rect.top_left(),
                    NonZeroU8::new(255).unwrap(),
                    Rect::MAX_SIZE,
                );
                self.draw(
                    &format!("Second use of {}", id.value() % 1000),
                    Style::new()
                        .background(Color::Red)
                        .set_underlined()
                        .set_rapid_blink(),
                    new_rect.top_left(),
                    NonZeroU8::new(255).unwrap(),
                    Rect::MAX_SIZE,
                )
            }
        }
    }

    pub fn screen_rect(&self) -> Rect {
        self.inner.read().unwrap().max_rect
    }

    pub fn read<R>(&self, func: impl FnOnce(&ContextInner) -> R) -> R {
        func(&self.inner.read().unwrap())
    }

    pub fn write<R>(&mut self, func: impl FnOnce(&mut ContextInner) -> R) -> R {
        func(&mut self.inner.write().unwrap())
    }

    pub fn input<R>(&self, func: impl FnOnce(&InputState) -> R) -> R {
        func(&self.inner.read().unwrap().input)
    }
}
