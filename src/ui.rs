use std::num::NonZeroU8;

use crossterm::style::Color;

use crate::{
    containers::{bordered::Bordered, drop_down::DropDown}, context::Context, id::Id, math_util::{Rect, VecI2}, response::Response, style::{Style, StyledText}, widgets::{button::Button, lable::Label, seperator::Separator}
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layout {
    TopLeftVertical,
    TopLeftHorizontal,
    TopRightVertical,
    TopRightHorizontal,
    BottomLeftVertical,
    BottomLeftHorizontal,
    BottomRightVertical,
    BottomRightHorizontal,
}

impl Layout {
    pub fn is_primary_vertical(&self) -> bool {
        match self {
            Layout::TopLeftVertical => true,
            Layout::TopLeftHorizontal => false,
            Layout::TopRightVertical => true,
            Layout::TopRightHorizontal => false,
            Layout::BottomLeftVertical => true,
            Layout::BottomLeftHorizontal => false,
            Layout::BottomRightVertical => true,
            Layout::BottomRightHorizontal => false,
        }
    }

    pub fn is_primary_horizontal(&self) -> bool {
        !self.is_primary_vertical()
    }

    pub fn to_vertical(&self) -> Self {
        match self {
            Layout::TopLeftVertical | Layout::TopLeftHorizontal => Layout::TopLeftVertical,
            Layout::TopRightVertical | Layout::TopRightHorizontal => Layout::TopRightVertical,
            Layout::BottomLeftVertical | Layout::BottomLeftHorizontal => Layout::BottomLeftVertical,
            Layout::BottomRightVertical | Layout::BottomRightHorizontal => {
                Layout::BottomRightVertical
            }
        }
    }

    pub fn to_horizontal(&self) -> Self {
        match self {
            Layout::TopLeftVertical | Layout::TopLeftHorizontal => Layout::TopLeftHorizontal,
            Layout::TopRightVertical | Layout::TopRightHorizontal => Layout::TopRightHorizontal,
            Layout::BottomLeftVertical | Layout::BottomLeftHorizontal => {
                Layout::BottomLeftHorizontal
            }
            Layout::BottomRightVertical | Layout::BottomRightHorizontal => {
                Layout::BottomRightHorizontal
            }
        }
    }

    pub fn opposite_primary_direction(&self) -> Self {
        if self.is_primary_vertical() {
            self.to_horizontal()
        } else {
            self.to_vertical()
        }
    }
}

pub struct Gallery<'a> {
    pub bound: Rect,
    pub items: Vec<(Rect, StyledText<'a>)>,
}

pub struct Ui {
    id: Id,
    next_id_source: u64,

    context: Context,
    layout: Layout,
    clip: Rect,
    max_rect: Rect,
    cursor: VecI2,
    current: Rect,
    layer: NonZeroU8,
}

impl Ui {
    pub fn new(ctx: Context, layout: Layout, id: Id, clip: Rect, layer: NonZeroU8) -> Self {
        let cursor = match layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => clip.top_left(),
            Layout::TopRightHorizontal | Layout::TopRightVertical => clip.top_right(),
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => clip.bottom_left(),
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => clip.bottom_right(),
        };
        Self {
            id,
            next_id_source: id.with(":3").value(),
            context: ctx,
            layout,
            clip,
            max_rect: clip,
            cursor,
            current: Rect::new_pos_size(cursor, VecI2::new(0, 0)),
            layer,
        }
    }

    pub fn get_clip(&self) -> Rect {
        self.clip
    }

    pub fn get_max(&self) -> Rect {
        self.max_rect
    }

    pub fn set_max(&mut self, max: Rect) {
        self.max_rect = max;
    }

    pub fn get_cursor(&self) -> VecI2 {
        self.cursor
    }

    pub fn get_current(&self) -> Rect {
        self.current
    }

    pub fn ctx(&self) -> &Context {
        &self.context
    }

    pub fn layer(&self) -> NonZeroU8 {
        self.layer
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn next_id_source(&mut self) -> u64 {
        self.next_id_source = self.next_id_source.wrapping_add(1);
        self.next_id_source.wrapping_sub(1)
    }

    pub fn interact(&mut self, id: Id, area: Rect) -> Response {
        self.context.interact(self.clip, self.layer, id, area)
    }

    pub fn child_ui(&mut self, max_rect: Rect, layout: Layout) -> Self {
        Self::new(
            self.ctx().clone(),
            layout,
            Id::new(self.next_id_source()).with("child"),
            max_rect,
            self.layer,
        )
    }

    pub fn draw(&mut self, text: &str, style: Style, start: VecI2, clip: Rect) {
        self.context.draw(text, style, start, self.layer, clip)
    }
}

// gallery
impl Ui {
    pub fn draw_gallery(&mut self, gallery: Gallery) {
        for (bound, text) in gallery.items {
            self.context
                .draw(&text.text, text.style, bound.top_left(), self.layer, bound);
        }
    }

    pub fn create_gallery<'a>(&self, text: &'a StyledText<'a>) -> Gallery<'a> {
        self.create_gallery_at(self.cursor, text)
    }

    pub fn create_gallery_at<'a>(&self, pos: VecI2, text: &'a StyledText<'a>) -> Gallery<'a> {
        let mut rect = Rect::new_pos_size(pos, VecI2::new(0, 0));

        let mut gallery = Vec::new();

        gallery.push((rect, text.clone()));
        // text.text

        for (line_num, line) in text.text.split('\n').enumerate() {
            let mut line_width = 0;
            for char in line.chars() {
                line_width += unicode_width::UnicodeWidthChar::width(char).unwrap_or(0) as u16;
            }
            gallery.push((
                Rect {
                    x: rect.x,
                    y: rect.y + line_num as u16,
                    width: line_width,
                    height: 1,
                },
                StyledText {
                    text: std::borrow::Cow::Borrowed(line),
                    style: text.style,
                },
            ));
            // rect.in
            rect.height += 1;
            rect.width = rect.width.max(line_width);
        }

        match self.layout {
            Layout::TopLeftVertical | Layout::TopLeftHorizontal => {}
            Layout::TopRightVertical | Layout::TopRightHorizontal => {
                rect.x = rect.x.saturating_sub(rect.width);
                for (bound, _item) in &mut gallery {
                    bound.x = bound.x.saturating_sub(rect.width);
                }
            }
            Layout::BottomLeftVertical | Layout::BottomLeftHorizontal => {
                rect.y = rect.y.saturating_sub(rect.height);
                for (bound, _item) in &mut gallery {
                    bound.y = bound.y.saturating_sub(rect.height);
                }
            }
            Layout::BottomRightVertical | Layout::BottomRightHorizontal => {
                rect.y = rect.y.saturating_sub(rect.height);
                rect.x = rect.x.saturating_sub(rect.width);
                for (bound, _item) in &mut gallery {
                    bound.x = bound.x.saturating_sub(rect.width);
                    bound.y = bound.y.saturating_sub(rect.height);
                }
            }
        }

        Gallery {
            bound: rect,
            items: gallery,
        }
    }
}

// space allocation
impl Ui {
    pub fn allocate_area(&mut self, rect: Rect) -> Rect {
        let start = match self.layout {
            Layout::TopLeftVertical | Layout::TopLeftHorizontal => rect.top_left(),
            Layout::TopRightVertical | Layout::TopRightHorizontal => rect.top_right_inner(),
            Layout::BottomLeftVertical | Layout::BottomLeftHorizontal => rect.bottom_left_inner(),
            Layout::BottomRightVertical | Layout::BottomRightHorizontal => {
                rect.bottom_right_inner()
            }
        };
        if start == self.cursor {
            self.allocate_size(rect.size())
        } else if rect.contains(self.cursor) {
            // TODO this means that we are trying to layout things inside eachother
            // maybe stop this from happening from drawing off screen?
            {}
            self.allocate_size(rect.size())
        } else {
            let mut rect = rect;
            rect.expand_to_include(&Rect::new_pos_size(self.cursor, VecI2::new(0, 0)));
            self.allocate_size(rect.size())
        }
    }

    pub fn allocate_size(&mut self, desired: VecI2) -> Rect {
        let old_cursor = self.cursor;
        let old_max = self.max_rect;
        self.add_space(desired);
        let new_cursor = self.cursor;

        if self.layout.is_primary_vertical() {
            self.cursor.x = old_cursor.x;
            self.max_rect.x = old_max.x;
            self.max_rect.width = old_max.width;
        } else {
            self.cursor.y = old_cursor.y;
            self.max_rect.y = old_max.y;
            self.max_rect.height = old_max.height;
        }
        let x = old_cursor.x.min(new_cursor.x);
        let y = old_cursor.y.min(new_cursor.y);
        let width = old_cursor.x.abs_diff(new_cursor.x);
        let height = old_cursor.y.abs_diff(new_cursor.y);
        Rect::new_pos_size(VecI2::new(x, y), VecI2::new(width, height))
    }

    pub fn add_horizontal_space(&mut self, space: u16) {
        self.add_space(VecI2::new(space, 0))
    }

    pub fn add_vertical_space(&mut self, space: u16) {
        self.add_space(VecI2::new(0, space))
    }

    pub fn add_space_primary_direction(&mut self, space: u16) {
        if self.layout.is_primary_horizontal() {
            self.add_space(VecI2::new(space, 0));
        } else {
            self.add_space(VecI2::new(0, space));
        }
    }

    pub fn add_space(&mut self, space: VecI2) {
        match self.layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                self.cursor += space;
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                self.cursor += VecI2::new(0, space.y);
                self.cursor -= VecI2::new(space.x, 0);
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                self.cursor -= VecI2::new(0, space.y);
                self.cursor += VecI2::new(space.x, 0);
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                self.cursor -= VecI2::new(space.x, space.y);
            }
        }

        if self.cursor.x < self.max_rect.x {
            self.cursor.x = self.max_rect.x;
        }
        if self.cursor.x > self.max_rect.x.saturating_add(self.max_rect.width) {
            self.cursor.x = self.max_rect.x.saturating_add(self.max_rect.width);
        }
        if self.cursor.y < self.max_rect.y {
            self.cursor.y = self.max_rect.y;
        }
        if self.cursor.y > self.max_rect.y.saturating_add(self.max_rect.height) {
            self.cursor.y = self.max_rect.y.saturating_add(self.max_rect.height);
        }

        match self.layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                self.clip.move_top_left_to(self.cursor);
                self.max_rect.move_top_left_to(self.cursor);
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                self.clip.move_top_right_to(self.cursor);
                self.max_rect.move_top_right_to(self.cursor);
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                self.clip.move_bottom_left_to(self.cursor);
                self.max_rect.move_bottom_left_to(self.cursor);
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                self.clip.move_bottom_right_to(self.cursor);
                self.max_rect.move_bottom_right_to(self.cursor);
            }
        }

        self.current
            .expand_to_include(&Rect::new_pos_size(self.cursor, VecI2::new(0, 0)));
    }

    pub fn expand(&mut self, translation: VecI2) {
        match self.layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                self.current.add_bottom_right(translation)
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                self.current.add_bottom_right(VecI2::new(0, translation.y));
                self.current.add_top_left(VecI2::new(translation.x, 0))
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                self.current.add_bottom_right(VecI2::new(translation.x, 0));
                self.current.add_top_left(VecI2::new(0, translation.y))
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                self.current.add_top_left(translation)
            }
        }
    }

    pub fn set_minimum_size(&mut self, mut min: VecI2) {
        min.x = min.x.min(self.max_rect.width);
        min.y = min.y.min(self.max_rect.height);
        match self.layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                self.current.width = self.current.width.max(min.x);
                self.current.height = self.current.height.max(min.y);
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                todo!();
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                todo!();
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                todo!();
            }
        }
    }
}

// widget helpers
impl Ui {
    pub fn label<'a>(&mut self, text: impl Into<StyledText<'a>>) {
        Label::new(text).show(self);
    }

    pub fn button<'a>(&mut self, text: impl Into<StyledText<'a>>) -> Response {
        Button::new(text).show(self)
    }

    pub fn seperator(&mut self) {
        Separator::new().show(self);
    }
}

// container/layout helpers
impl Ui {
    pub fn vertical<R, F: FnOnce(&mut Ui) -> R>(&mut self, func: F) -> R {
        self.with_layout(self.layout.to_vertical(), func)
    }

    pub fn horizontal<R, F: FnOnce(&mut Ui) -> R>(&mut self, func: F) -> R {
        self.with_layout(self.layout.to_horizontal(), func)
    }

    pub fn with_layout<R, F: FnOnce(&mut Ui) -> R>(&mut self, layout: Layout, func: F) -> R {
        let mut ui = self.child_ui(self.max_rect, layout);
        let res = func(&mut ui);
        self.allocate_area(ui.current);
        res
    }

    pub fn with_size(&mut self, size: VecI2, func: impl FnOnce(&mut Ui)) {
        let size = self.allocate_size(size);
        let mut child = self.child_ui(size, self.layout);
        func(&mut child)
    }

    pub fn drop_down<'a>(&mut self, title: impl Into<StyledText<'a>>, func: impl FnOnce(&mut Ui)) {
        DropDown::new(title).show(self, |ui, _| func(ui));
    }   

    pub fn bordered<R>(&mut self, func: impl FnOnce(&mut Ui) -> R) -> R {
        Bordered::new().show(self, func)
    }

    pub fn tabbed_area<'a, F: FnOnce(usize, &mut Self) -> R, R, const L: usize>(
        &mut self,
        id: Id,
        titles: [impl Into<StyledText<'a>>; L],
        func: F,
    ) -> R {
        let last_index = titles.len() - 1;
        let mut val = self.ctx().get_memory_or(id, 0usize);
        // let start = ui.cursor;
        self.with_layout(self.layout, |ui| {
            ui.add_space_primary_direction(1);
            ui.with_layout(ui.layout.opposite_primary_direction(), |ui| {
                ui.add_space_primary_direction(1);
                for (i, title) in titles.into_iter().enumerate() {
                    let mut title: StyledText = title.into();
                    if i == val {
                        title.bg(Color::DarkGrey)
                    }
                    if ui.button(title).clicked() {
                        val = i;
                        ui.ctx().insert_into_memory(id, i);
                    }
                    if i != last_index {
                        ui.seperator();
                    } else {
                        ui.add_space_primary_direction(1);
                    }
                }
            });
            ui.add_space_primary_direction(1);

            let tab_box = ui.current;

            ui.ctx().check_for_id_clash(id, tab_box);

            let res = func(val, ui);

            let mut bruh = BoxedArea::default();
            bruh.add_line(tab_box.top_left(), tab_box.top_right_inner());
            bruh.add_line(tab_box.top_right_inner(), tab_box.bottom_right_inner());
            bruh.add_line(tab_box.bottom_right_inner(), tab_box.bottom_left_inner());
            bruh.add_line(tab_box.bottom_left_inner(), tab_box.top_left());
            let lines = ui.ctx().style().borrow().lines;
            bruh.draw(&mut ui.context, Style::default(), lines);

            res
        })
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct NodeAttachements {
    up: bool,
    left: bool,
    right: bool,
    down: bool,
}

#[derive(Debug, Default, Clone)]
struct BoxedArea {
    vertices: std::collections::HashMap<VecI2, NodeAttachements>,
    lines: Vec<(VecI2, VecI2, bool)>,
}

impl BoxedArea {
    pub fn add_line(&mut self, p1: VecI2, p2: VecI2) {
        // assert!(p1 != p2);
        if p1 == p2 {
            return;
        }
        if p1.x == p2.x {
            let p1_node = self.vertices.entry(p1).or_default();
            if p1.y > p2.y {
                p1_node.down = true;
            } else {
                p1_node.up = true;
            }

            let p2_node = self.vertices.entry(p2).or_default();
            if p1.y > p2.y {
                p2_node.up = true;
            } else {
                p2_node.down = true;
            }
            self.lines.push((p1, p2, false))
        } else if p1.y == p2.y {
            let p1_node = self.vertices.entry(p1).or_default();
            if p1.x > p2.x {
                p1_node.right = true;
            } else {
                p1_node.left = true;
            }

            let p2_node = self.vertices.entry(p2).or_default();
            if p1.x > p2.x {
                p2_node.left = true;
            } else {
                p2_node.right = true;
            }
            self.lines.push((p1, p2, true))
        } else {
            panic!("Not stright line");
        }
    }

    pub fn draw(&self, ctx: &mut Context, style: Style, set: &crate::symbols::line::Set) {
        for (pos, node) in &self.vertices {
            let val = match (node.up, node.right, node.down, node.left) {
                (true, false, true, false) => set.vertical,
                (true, true, true, false) => set.vertical_right,
                (true, false, true, true) => set.vertical_left,

                (false, true, false, true) => set.horizontal,
                (true, true, false, true) => set.horizontal_down,
                (false, true, true, true) => set.horizontal_up,

                (true, true, false, false) => set.top_right,
                (false, true, true, false) => set.bottom_right,
                (false, false, true, true) => set.bottom_left,
                (true, false, false, true) => set.top_left,

                (true, true, true, true) => set.cross,
                _ => "*",
            };
            // let clip = c.max_rect;
            ctx.draw(
                val,
                style,
                *pos,
                NonZeroU8::new(1).unwrap(),
                Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(u16::MAX, u16::MAX)),
            );
            // lock.draws.push(Draw::Text(
            //     StyledText {
            //         text: val.to_owned(),
            //         style,
            //     },
            //     *pos,
            // ))
        }
    }
}
