use std::{
    num::NonZeroU8,
    sync::{Arc, RwLock},
};

use crossterm::event::{Event, MouseButton, MouseEventKind};

use crate::{
    id::Id,
    input::mouse::{MouseState},
    math_util::{Rect, VecI2},
    memory::{IdCollision, Memory},
    response::Response,
    screen::{Screen, ScreenDrain, ScreenIter},
    style::Style,
    ui::{Layout, Ui},
};

struct InputState {}

#[derive(Debug, Default)]
pub struct ContextInner {
    mouse: Option<MouseState>,
    memory: Memory,

    current: Screen,
    last: Screen,


    max_rect: Rect,
    last_reported_screen: Rect,
    resized: bool,
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

    pub fn start_frame(&mut self){
        if self.max_rect != self.last_reported_screen{
            self.max_rect = self.last_reported_screen;
            self.current.resize(self.max_rect.size());
            self.last.resize(self.max_rect.size());
            self.resized = true;
        }
    }

    pub fn finish_frame(&mut self) -> FrameReport<'_> {
        if let Some(mouse) = &mut self.mouse {
            for button in &mut mouse.buttons {
                button.next_state();
            }
        }

        self.memory.clear_seen();
        let ContextInner { current, last, .. } = self;
        std::mem::swap(last, current);
        FrameReport {
            resized: {
                let tmp = self.resized;
                self.resized = false;
                tmp
            },
            current_frame: last.iter(),
            last_frame: current.drain(),
        }
    }
}

pub struct FrameReport<'a> {
    pub resized: bool,
    pub current_frame: ScreenIter<'a>,
    pub last_frame: ScreenDrain<'a>,
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

    pub fn handle_event(&self, event: Event) -> bool {
        let mut lock = self.inner.write().unwrap();
        match event {
            Event::Resize(x, y) => {
                lock.last_reported_screen = Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(x, y));
                false
            }
            Event::Mouse(event) => {
                let mouse = lock.mouse.get_or_insert(MouseState::default());
                mouse.position.x = event.column;
                mouse.position.y = event.row;
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
                                button.button_down(mouse.position);
                            }
                            MouseEventKind::Up(_) => {
                                button.button_up(mouse.position);
                            }
                            MouseEventKind::Drag(_) => {
                                button.button_dragged(mouse.position);
                            }
                            _ => {}
                        }
                    }
                    MouseEventKind::Moved => {}
                    MouseEventKind::ScrollDown => mouse.scroll -= 1,
                    MouseEventKind::ScrollUp => mouse.scroll += 1,
                }
                // mouse.kind
                // mouse.modifiers
                // lock.last_observed_mouse_pos = Some(VecI2::new(mouse.row, mouse.column));
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn new(size: VecI2) -> Context {
        Self {
            inner: Arc::new(RwLock::new(ContextInner::new(size))),
        }
    }

    pub fn draw(&mut self, str: &str, style: Style, start: VecI2, layer: NonZeroU8, clip: Rect) {
        let mut lock = self.inner.write().unwrap();
        lock.current.push_text(str, style, start, layer, clip)
    }

    pub fn interact(&self, clip: Rect, id: Id, area: Rect) -> Response {
        let lock = self.inner.read().unwrap();
        if let Some(mouse) = &lock.mouse {
            if area.contains(mouse.position) {
                let mut response = Response::new(area, id, Some(mouse.position));
                response.buttons = mouse.buttons;
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

    pub fn get_memory_or<T: Clone + 'static>(&self, id: Id, default: T) -> Result<T, IdCollision> {
        let mut lock = self.inner.write().unwrap();
        lock.memory.get_or(id, default)
    }

    pub fn get_memory_or_create<T: Clone + 'static>(
        &self,
        id: Id,
        default: impl FnOnce() -> T,
    ) -> Result<T, IdCollision> {
        let mut lock = self.inner.write().unwrap();
        lock.memory.get_or_create(id, default)
    }

    pub fn screen_rect(&self) -> Rect {
        self.inner.read().unwrap().max_rect
    }
}
