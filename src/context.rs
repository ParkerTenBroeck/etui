use std::{
    collections::HashMap, num::NonZeroU8, sync::{Arc, RwLock}, time::Duration
};

use crossterm::event::Event;

use crate::{
    id::Id,
    input::{InputState, MoreInput},
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

    used_ids: HashMap<Id, Rect>,

    min_tick_rate: Duration,
    max_tick_rate: Duration,
    request_redraw: bool,
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

    pub fn finish_frame(&mut self, written: usize) -> MoreInput{
        let ContextInner { current, last, .. } = self;
        std::mem::swap(last, current);
        self.resized = false;

        let more_input = self.input.next_state();

        self.used_ids.clear();

        self.previous_frame_report.bytes_written = written;
        self.previous_frame_report.total_styles = self.last.num_styles();
        self.previous_frame_report.total_text_len = self.last.text_len();

        self.frame += 1;

        more_input
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

    pub fn request_redraw(&self){
        self.inner.write().unwrap().request_redraw = true;
    }

    pub fn should_redraw(&self) -> bool{
        self.inner.write().unwrap().request_redraw
    }

    pub fn get_min_tick(&self) -> Duration{
        self.inner.read().unwrap().min_tick_rate
    }

    pub fn get_max_tick(&self) -> Duration{
        self.inner.read().unwrap().max_tick_rate
    }

    pub fn set_min_tick(&self, duration: Duration) {
        self.inner.write().unwrap().min_tick_rate = duration;
    }

    pub fn set_max_tick(&self, duration: Duration) {
        self.inner.write().unwrap().max_tick_rate = duration;
    }

    pub fn inner(&mut self) -> &mut Arc<RwLock<ContextInner>> {
        &mut self.inner
    }

    pub fn previous_frame_report(&self) -> PreviousFrameReport {
        self.inner.read().unwrap().previous_frame_report
    }

    pub fn handle_event(&self, event: Event) -> MoreInput {
        let mut lock = self.inner.write().unwrap();
        match event{
            Event::Resize(x, y) => {
                lock.last_reported_screen = Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(x, y));
                MoreInput::Yes
            }
            event @ _ => {
                lock.input.handle_event(event)
            }
        }
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
