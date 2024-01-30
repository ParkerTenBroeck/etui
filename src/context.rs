use std::{
    cell::RefCell, collections::HashMap, marker::PhantomData, num::NonZeroU8, ops::Deref, time::Duration
};

use crossterm::event::Event;

use crate::{
    id::Id,
    input::{InputState, MoreInput},
    math_util::{Rect, VecI2},
    memory::Memory,
    response::Response,
    screen::{Screen, ScreenDrain, ScreenIter},
    style::{Color, DefaultStyle, Style},
    ui::{Layout, Ui},
};

#[derive(Debug, Default)]
struct Focus {
    focused: Option<(Id, Rect)>,
    ids: HashMap<Id, Rect>,
    ordered: Vec<Id>,
}

#[derive(Debug)]
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

    pub(crate) min_tick_rate: Duration,
    pub(crate) max_tick_rate: Duration,
    pub(crate) request_redraw: bool,

    pontees: usize,
    borrowed: bool,
    _phantom: PhantomData<*mut ()>,

    style: RefCell<DefaultStyle>,

    focus: RefCell<Focus>,

    pub(crate) current_cursor: Option<Cursor>,
    last_cursor: Option<Cursor>,
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        if self.pontees != 0 {
            panic!("Outstanding references to ContextInner while being dropped.")
        }
    }
}

impl ContextInner {
    pub(super) fn new(size: VecI2) -> ContextInner {
        let screen = Rect::new_pos_size(VecI2::new(0, 0), size);
        let mut myself = Self {
            max_rect: screen,
            last_reported_screen: screen,
            memory: Default::default(),
            input: Default::default(),
            current: Default::default(),
            last: Default::default(),
            previous_frame_report: Default::default(),
            resized: Default::default(),
            frame: Default::default(),
            used_ids: Default::default(),
            min_tick_rate: Default::default(),
            max_tick_rate: Default::default(),
            request_redraw: Default::default(),
            pontees: 0,
            borrowed: false,
            _phantom: PhantomData,
            style: RefCell::new(DefaultStyle::new_unicode()),
            current_cursor: None,
            last_cursor: None,
            focus: RefCell::default(),
        };
        myself.current.resize(size);
        myself.last.resize(size);
        myself
    }

    pub fn start_frame(&mut self) {
        if self.max_rect != self.last_reported_screen {
            self.max_rect = self.last_reported_screen;
            self.current.resize(self.max_rect.size());
            self.last.resize(self.max_rect.size());
            self.resized = true;
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            None,
            Forward,
            Backward,
            Up,
            Down,
            Left,
            Right,
        }

        let mut direction = Direction::None;
        {
            use crossterm::event::KeyCode;
            let pressed = &self.input.keyboard.pressed;
            if pressed.get(&KeyCode::Tab).is_some() {
                direction = Direction::Forward;
            }
            if pressed.get(&KeyCode::BackTab).is_some() {
                direction = Direction::Backward;
            }

            if pressed.get(&KeyCode::Up).is_some() {
                direction = Direction::Up;
            }
            if pressed.get(&KeyCode::Down).is_some() {
                direction = Direction::Down;
            }
            if pressed.get(&KeyCode::Left).is_some() {
                direction = Direction::Left;
            }
            if pressed.get(&KeyCode::Right).is_some() {
                direction = Direction::Right;
            }
        }

        let id = if let Some((focused_id, focused_rect)) = self.focus.get_mut().focused {
            match direction {
                Direction::None => None,
                Direction::Forward | Direction::Backward => {
                    let position = self
                        .focus
                        .get_mut()
                        .ordered
                        .iter()
                        .position(|v| v == &focused_id);
                    if let Some(position) = position {
                        let position = position
                            .checked_add_signed(if direction == Direction::Forward {
                                1
                            } else {
                                -1
                            })
                            .unwrap_or(self.focus.get_mut().ordered.len() - 1);
                        let id = if position >= self.focus.get_mut().ordered.len() {
                            self.focus.get_mut().ordered[0]
                        } else {
                            self.focus.get_mut().ordered[position]
                        };
                        Some(id)
                    } else {
                        None
                    }
                }
                Direction::Up | Direction::Down | Direction::Left | Direction::Right => {
                    let vector = match direction {
                        Direction::Up => VecI2::new(0, 1),
                        Direction::Down => VecI2::new(0, u16::MAX),
                        Direction::Left => VecI2::new(u16::MAX, 0),
                        Direction::Right => VecI2::new(1, 0),
                        _ => unreachable!(),
                    };

                    

                    for (id, rect) in self.focus.borrow_mut().ids.iter(){

                    }

                    None
                }
            }
        } else if direction != Direction::None {
            self.focus.get_mut().ordered.first().copied()
        } else {
            None
        };

        if let Some(id) = id {
            self.focus.get_mut().focused = self.focus.get_mut().ids.get(&id).map(|v| (id, *v));
        }

        // eprintln!("{:?}", self.focus.get_mut().ids);
        self.focus.get_mut().ids.clear();
        self.focus.get_mut().ordered.clear();
    }

    pub fn get_finished_frame(&mut self) -> FinishedFrame<'_> {
        FinishedFrame {
            resized: self.resized,
            // we want to preserve this frame to allow us to diff it next frame
            current_frame: self.current.iter(),
            // but the last frame needs to be cleared before the next frame starts so we can drain it
            last_frame: self.last.drain(),
            current_cursor: self.current_cursor,
            last_cursor: self.last_cursor,
        }
    }

    pub fn finish_frame(&mut self, written: usize) -> MoreInput {
        let ContextInner {
            current,
            last,
            current_cursor,
            last_cursor,
            ..
        } = self;
        std::mem::swap(current_cursor, last_cursor);
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

    pub fn handle_event(&mut self, event: Event) -> MoreInput {
        match event {
            Event::Resize(x, y) => {
                self.last_reported_screen = Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(x, y));
                MoreInput::Yes
            }
            _ => self.input.handle_event(event),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub struct FinishedFrame<'a> {
    pub resized: bool,
    pub current_frame: ScreenIter<'a>,
    pub last_frame: ScreenDrain<'a>,
    pub current_cursor: Option<Cursor>,
    pub last_cursor: Option<Cursor>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PreviousFrameReport {
    pub bytes_written: usize,
    pub total_text_len: usize,
    pub total_styles: usize,
}

pub struct Context {
    inner: *mut ContextInner,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        if unsafe { (*self.inner).borrowed } {
            panic!("Tried to clone when borrowed")
        }
        unsafe { (*self.inner).pontees += 1 }
        Self { inner: self.inner }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { (*self.inner).pontees -= 1 }
    }
}

pub struct ContextGuard<'a> {
    context: &'a mut ContextInner,
}

impl<'a> ContextGuard<'a> {
    unsafe fn new(inner: *mut ContextInner) -> Self {
        (*inner).borrowed = true;
        Self {
            context: &mut *inner,
        }
    }
}

impl<'a> std::ops::Deref for ContextGuard<'a> {
    type Target = ContextInner;

    fn deref(&self) -> &Self::Target {
        self.context
    }
}

impl<'a> std::ops::DerefMut for ContextGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.context
    }
}

impl<'a> Drop for ContextGuard<'a> {
    fn drop(&mut self) {
        self.context.borrowed = false;
    }
}

impl Context {
    pub(crate) fn inner_mut(&self) -> Option<ContextGuard<'_>> {
        if unsafe { (*self.inner).pontees } == 1 {
            Some(unsafe { ContextGuard::new(self.inner) })
        } else {
            None
        }
    }

    pub fn frame(&self, func: impl FnOnce(&mut Ui)) {
        let clip = unsafe { (*self.inner).max_rect };
        func(&mut Ui::new(
            self.clone(),
            Layout::TopLeftVertical,
            Id::new("frame"),
            clip,
            NonZeroU8::new(128).unwrap(),
        ));
    }

    pub fn push_id(&self, id: Id, rect: Rect) {
        self.focus().borrow_mut().ids.insert(id, rect);
        self.focus().borrow_mut().ordered.push(id);
    }

    pub fn focus(&self) -> &RefCell<Focus> {
        unsafe { &(*self.inner).focus }
    }

    pub fn request_redraw(&self) {
        unsafe { (*self.inner).request_redraw = true }
    }

    pub fn should_redraw(&self) -> bool {
        unsafe { (*self.inner).request_redraw }
    }

    pub fn get_min_tick(&self) -> Duration {
        unsafe { (*self.inner).min_tick_rate }
    }

    pub fn get_max_tick(&self) -> Duration {
        unsafe { (*self.inner).max_tick_rate }
    }

    pub fn set_min_tick(&self, duration: Duration) {
        unsafe { (*self.inner).min_tick_rate = duration }
    }

    pub fn set_max_tick(&self, duration: Duration) {
        unsafe { (*self.inner).max_tick_rate = duration }
    }

    pub fn previous_frame_report(&self) -> PreviousFrameReport {
        unsafe { (*self.inner).previous_frame_report }
    }

    pub fn style(&self) -> &RefCell<DefaultStyle> {
        unsafe { &(*self.inner).style }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        unsafe { (*self.inner).current_cursor }
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        unsafe { (*self.inner).current_cursor = Some(cursor) }
    }

    /// Creates a new [`Context`].
    ///
    /// # Safety
    /// The memory behind *mut ContextInner must not outlive this container
    /// and must always be valid. It must not be shared across threads nor accessed without
    /// checking that no outstanding pointees to that data exist
    /// .
    pub unsafe fn new(context: *mut ContextInner) -> Self {
        unsafe {
            (*context).pontees += 1;
        }
        Self { inner: context }
    }

    pub fn draw(&self, str: &str, style: Style, start: VecI2, layer: NonZeroU8, clip: Rect) {
        unsafe {
            (*self.inner)
                .current
                .push_text(str, style, start, layer, clip)
        }
    }

    pub fn interact(&self, _clip: Rect, _layer: NonZeroU8, id: Id, area: Rect) -> Response {
        self.check_for_id_clash(id, area);
        self.push_id(id, area);

        let mut focused = false;
        if let Some((cid, crect)) = &mut self.focus().borrow_mut().focused {
            if id == *cid {
                *crect = area;
                focused = true;
            }
        }

        let mut response = if let Some(position) = &self.input().mouse.position {
            if area.contains(*position) {
                let mut response = Response::new(area, id, Some(*position));
                response.buttons = self.input().mouse.buttons;
                response
            } else {
                Response::new(area, id, None)
            }
        } else {
            Response::new(area, id, None)
        };
        response.hovered |= focused;
        response
    }

    pub fn insert_into_memory<T: Clone + 'static>(&self, id: Id, val: T) {
        unsafe { (*self.inner).memory.insert(id, val) };
    }

    pub fn get_memory_or<T: Clone + 'static>(&self, id: Id, default: T) -> T {
        unsafe { (*self.inner).memory.get_or(id, default) }
    }

    pub fn get_frame(&self) -> usize {
        unsafe { (*self.inner).frame }
    }

    pub fn get_memory_or_create<T: Clone + 'static>(
        &self,
        id: Id,
        default: impl FnOnce() -> T,
    ) -> T {
        unsafe { (*self.inner).memory.get_or_create(id, default) }
    }

    pub fn check_for_id_clash(&self, id: Id, new_rect: Rect) {
        let prev_rect = unsafe { (*self.inner).used_ids.insert(id, new_rect) };
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
        unsafe { (*self.inner).max_rect }
    }

    pub fn input(&self) -> &InputState {
        unsafe { &(*self.inner).input }
    }

    pub fn try_input_mut<R>(&self, func: impl FnOnce(&mut InputState) -> R) -> Option<R> {
        self.inner_mut().map(|mut v| func(&mut v.input))
    }
}
