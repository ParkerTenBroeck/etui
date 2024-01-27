use std::{
    cell::RefCell, collections::HashMap, marker::PhantomData, num::NonZeroU8, time::Duration,
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
    _phantom: PhantomData<*mut ()>,

    style: RefCell<DefaultStyle>,

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
            pontees: Default::default(),
            _phantom: PhantomData,
            style: RefCell::new(DefaultStyle::new_unicode()),
            current_cursor: None,
            last_cursor: None,
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
        unsafe { (*self.inner).pontees += 1 }
        Self { inner: self.inner }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { (*self.inner).pontees -= 1 }
    }
}

impl Context {
    pub(crate) fn inner_mut(&self) -> Option<&mut ContextInner> {
        if unsafe { (*self.inner).pontees } == 1 {
            Some(unsafe { &mut *self.inner })
        } else {
            None
        }
    }

    pub fn frame(&self, func: impl FnOnce(&mut Ui)) {
        let clip = unsafe { (*self.inner).max_rect };
        func(&mut Ui::new(
            self.clone(),
            Layout::TopLeftVertical,
            clip,
            NonZeroU8::new(128).unwrap(),
        ));
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

    pub fn interact(&self, _clip: Rect, id: Id, area: Rect) -> Response {
        if let Some(position) = &self.input().mouse.position {
            if area.contains(*position) {
                let mut response = Response::new(area, id, Some(*position));
                response.buttons = self.input().mouse.buttons;
                response
            } else {
                Response::new(area, id, None)
            }
        } else {
            Response::new(area, id, None)
        }
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
        self.inner_mut().map(|v| func(&mut v.input))
    }
}
