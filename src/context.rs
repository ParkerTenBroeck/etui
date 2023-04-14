use std::{
    num::NonZeroU8,
    sync::{Arc, RwLock},
};

use crossterm::event::{Event, MouseButton, MouseEventKind};

use crate::{
    id::Id,
    input::mouse::{MouseButtonState, MouseState},
    math_util::{Rect, VecI2},
    memory::{IdCollision, Memory},
    response::Response,
    screen::{Screen, ScreenDrain, ScreenIter},
    style::Style,
    ui::{Layout, Ui},
};

#[derive(Debug, Default)]
pub struct ContextInner {
    event: Option<Event>,
    mouse: Option<MouseState>,
    max_rect: Rect,
    memory: Memory,

    current: Screen,
    last: Screen,
}
impl ContextInner {
    fn new(size: VecI2) -> ContextInner {
        let mut myself = Self {
            max_rect: Rect::new_pos_size(VecI2::new(0, 0), size),
            ..Default::default()
        };
        myself.current.resize(size);
        myself.last.resize(size);
        myself
    }

    pub fn draw(&mut self, str: &str, style: Style, start: VecI2, layer: NonZeroU8, clip: Rect) {
        self.current.push_text(str, style, start, layer, clip)
    }

    pub fn finish_frame(&mut self) -> (ScreenIter<'_>, ScreenDrain<'_>) {
        if let Some(mouse) = &mut self.mouse {
            for button in &mut mouse.buttons {
                button.next_state();
            }
        }

        self.memory.clear_seen();
        let ContextInner { current, last, .. } = self;
        std::mem::swap(last, current);
        (last.iter(), current.drain())
    }
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

    pub fn handle_event(&self, event: Event) {
        let mut lock = self.inner.write().unwrap();
        match event {
            Event::Resize(x, y) => {
                lock.max_rect = Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(x, y))
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
                                *button = MouseButtonState::Down;
                            }
                            MouseEventKind::Up(_) => {
                                *button = MouseButtonState::Up;
                            }
                            MouseEventKind::Drag(_) => {
                                *button = MouseButtonState::Drag;
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
            }
            _ => {}
        }
        lock.event = Some(event)
    }

    pub fn get_event(&self) -> Option<Event> {
        self.inner.read().unwrap().event.clone()
    }

    pub fn new(size: VecI2) -> Context {
        Self {
            inner: Arc::new(RwLock::new(ContextInner::new(size))),
        }
    }

    pub fn clear_event(&self) {
        self.inner.write().unwrap().event = None
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

    pub fn set_size(&self, last_observed_size: VecI2) -> bool {
        let mut lock = self.inner.write().unwrap();
        if lock.max_rect.size() != last_observed_size {
            lock.max_rect = Rect::new_pos_size(VecI2::new(0, 0), last_observed_size);
            lock.current.resize(last_observed_size);
            lock.last.resize(last_observed_size);
            true
        } else {
            false
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
