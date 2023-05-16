use std::num::NonZeroU8;

use crossterm::style::{Attribute, Color};

use crate::{
    containers::bordered::Bordered,
    containers::textfield::TextField,
    context::Context,
    id::Id,
    math_util::{Rect, VecI2},
    response::Response,
    style::{Style, StyledText},
    symbols::{self, line::*},
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

#[derive(Clone)]
pub struct Ui {
    context: Context,
    layout: Layout,
    clip: Rect,
    max_rect: Rect,
    cursor: VecI2,
    current: Rect,
    layer: NonZeroU8,
}

impl Ui {
    pub fn new(ctx: Context, layout: Layout, clip: Rect, layer: NonZeroU8) -> Self {
        let cursor = match layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => clip.top_left(),
            Layout::TopRightHorizontal | Layout::TopRightVertical => clip.top_right(),
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => clip.bottom_left(),
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => clip.bottom_right(),
        };
        Self {
            context: ctx,
            layout,
            clip,
            max_rect: clip,
            cursor,
            current: Rect::new_pos_size(cursor, VecI2::new(0, 0)),
            layer,
        }
    }

    pub fn label<'a>(&mut self, text: impl Into<StyledText<'a>>) {
        let text = text.into();
        let gallery = self.create_gallery(&text);
        self.allocate_area(gallery.bound);
        self.draw_gallery(gallery);
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

    // pub fn with_memory_or<T: Clone + 'static, F: FnOnce(T, &mut Self) -> R, R>(
    //     &mut self,
    //     id: Id,
    //     default: T,
    //     func: F,
    // ) -> Option<R> {
    //     let res = self.context.get_memory_or(id, default);

    //     if let Ok(val) = res {
    //         return Some(func(val, self));
    //     }

    //     let mut style = Style {
    //         bg: Color::Red,
    //         fg: Color::White,
    //         ..Default::default()
    //     };
    //     style.attributes.set(Attribute::RapidBlink);
    //     style.attributes.set(Attribute::Underlined);
    //     self.label(StyledText::styled(
    //         &format!("IDCOLLISION: {:?}", id.value()),
    //         style,
    //     ));
    //     None
    // }

    // pub fn with_memory_or_make<T: Clone + 'static, F: FnOnce(T, &mut Self) -> R, R>(
    //     &mut self,
    //     id: Id,
    //     default: impl FnOnce() -> T,
    //     func: F,
    // ) -> Option<R> {
    //     let res = self.context.get_memory_or_create(id, default);
    //     // drop(lock);
    //     if let Ok(val) = res {
    //         return Some(func(val, self));
    //     }

    //     let mut style = Style {
    //         bg: Color::Red,
    //         fg: Color::White,
    //         ..Default::default()
    //     };
    //     style.attributes.set(Attribute::RapidBlink);
    //     style.attributes.set(Attribute::Underlined);
    //     self.label(StyledText::styled(
    //         &format!("IDCOLLISION: {:?}", id.value()),
    //         style,
    //     ));
    //     None
    // }

    fn child(&self) -> Ui {
        let mut ui = self.clone();
        ui.current = Rect::new_pos_size(ui.cursor, VecI2::new(0, 0));
        ui.clip.move_top_left_to(ui.cursor);
        ui
    }

    pub fn child_ui(&self, max_rect: Rect, layout: Layout) -> Self {
        Self::new(self.ctx().clone(), layout, max_rect, self.layer)
    }

    pub fn with_size(&mut self, size: VecI2, func: impl FnOnce(&mut Ui)) {
        let size = self.allocate_size(size);
        let mut child = self.child();
        child.clip = size;
        child.max_rect = size;
        child.current = size;
        child.cursor = size.top_left();
        func(&mut child)
    }

    pub fn tabbed_area<'a, F: FnOnce(usize, &mut Self) -> R, R, const L: usize>(
        &mut self,
        id: Id,
        titles: [impl Into<StyledText<'a>>; L],
        func: F,
    ) -> R {
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
                    ui.add_space_primary_direction(1);
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
            bruh.draw(&mut ui.context, Style::default(), &symbols::line::NORMAL);

            res
        })
    }

    pub fn progress_bar(
        &mut self,
        mut style: Style,
        min_size: u16,
        max_size: u16,
        width: u16,
        layout: Layout,
        progress: f32,
    ) -> Response {
        let mut string = String::new();

        let cursor = self.cursor;

        let (len, area) = if layout.is_primary_horizontal() {
            let size = self.current.width.clamp(min_size, max_size);
            let rect = self.allocate_size(VecI2::new(size, width));
            (rect.width, rect)
        } else {
            let size = self.current.height.clamp(min_size, max_size);
            let rect = self.allocate_size(VecI2::new(width, size));
            (rect.height, rect)
        };

        let complete = (len as f32 * progress.clamp(0.0, 1.0) * 8.0) as u32;
        let whole = complete / 8;
        let remaining = ((len as u32 * 8) - complete) / 8;

        for _ in 0..whole {
            for _ in 0..width {
                string.push('█');
            }
            if layout.is_primary_vertical() {
                string.push('\n');
            }
        }
        match layout {
            Layout::TopLeftVertical => style.attributes.set(Attribute::Reverse),
            Layout::TopLeftHorizontal => style.attributes.set(Attribute::NoReverse),
            Layout::TopRightVertical => style.attributes.set(Attribute::Reverse),
            Layout::TopRightHorizontal => style.attributes.set(Attribute::Reverse),
            Layout::BottomLeftVertical => style.attributes.set(Attribute::NoReverse),
            Layout::BottomLeftHorizontal => style.attributes.set(Attribute::NoReverse),
            Layout::BottomRightVertical => style.attributes.set(Attribute::NoReverse),
            Layout::BottomRightHorizontal => style.attributes.set(Attribute::Reverse),
        }

        if whole + remaining != len as u32 {
            let t = if layout.is_primary_horizontal() {
                match complete % 8 {
                    0 => ' ',
                    1 => '▏',
                    2 => '▎',
                    3 => '▍',
                    4 => '▌',
                    5 => '▋',
                    6 => '▊',
                    7 => '▉',
                    // not gonna happen
                    _ => ' ',
                }
            } else {
                match complete % 8 {
                    0 => ' ',
                    1 => '▁',
                    2 => '▂',
                    3 => '▃',
                    4 => '▄',
                    5 => '▅',
                    6 => '▆',
                    7 => '▇',
                    // not gonna happen
                    _ => ' ',
                }
            };

            string.push(t);
            if layout.is_primary_vertical() {
                for _ in 0..(width-1) {
                    string.push(t);
                }
                string.push('\n');
            }
        }
        for _ in 0..remaining {
            for _ in 0..width {
                string.push(' ');
            }
            if layout.is_primary_vertical() {
                string.push('\n');
            }
        }
        if layout.is_primary_vertical() {
            string = string.chars().rev().collect();
        } else if width > 1 {
            let initial = string.clone();
            for _ in 0..(width - 1) {
                string.push('\n');
                string.push_str(&initial);
            }
        }
        string = string.trim_matches('\n').to_owned();
        let text = StyledText::styled(&string, style);
        let gallery = self.create_gallery_at(cursor, &text);
        // assert_eq!(gallery.bound, area, "{:#?}", gallery.items);
        self.draw_gallery(gallery);

        self.interact(Id::new("Bruh"), area)
    }

    pub fn bordered<R>(&mut self, func: impl FnOnce(&mut Ui) -> R) -> R {
        Bordered::new().show(self, func)
    }

    pub fn textfield<R>(&mut self, func: impl FnOnce(&mut Ui) -> R) -> R {
        TextField::new().show(self, func)
    }

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
            panic!("Cannot allocate before cursor")
        } else {
            let mut rect = rect;
            rect.expand_to_include(&Rect::new_pos_size(self.cursor, VecI2::new(0, 0)));
            self.allocate_size(rect.size())
        }
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn vertical<R, F: FnOnce(&mut Ui) -> R>(&mut self, func: F) -> R {
        self.with_layout(self.layout.to_vertical(), func)
    }
    pub fn horizontal<R, F: FnOnce(&mut Ui) -> R>(&mut self, func: F) -> R {
        self.with_layout(self.layout.to_horizontal(), func)
    }

    pub fn with_layout<R, F: FnOnce(&mut Ui) -> R>(&mut self, layout: Layout, func: F) -> R {
        let mut ui = self.clone();

        match layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                ui.cursor = ui.max_rect.top_left();
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                ui.cursor = ui.max_rect.top_right();
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                ui.cursor = ui.max_rect.bottom_left();
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                ui.cursor = ui.max_rect.bottom_right();
            }
        }
        ui.current = Rect::new_pos_size(ui.cursor, VecI2::new(0, 0));
        ui.layout = layout;
        let res = func(&mut ui);

        self.allocate_area(ui.current);

        res
    }

    pub fn seperator(&mut self) {
        if self.layout.is_primary_horizontal() {
            let area = self.allocate_size(VecI2::new(1, self.current.height));

            for i in 0..area.height {
                self.context.draw(
                    VERTICAL,
                    Style::default(),
                    VecI2 {
                        x: area.x,
                        y: self.current.y + i,
                    },
                    self.layer,
                    area,
                );
            }
        } else {
            let area = self.allocate_size(VecI2::new(self.current.width, 1));
            for i in 0..area.width {
                self.context.draw(
                    HORIZONTAL,
                    Style::default(),
                    VecI2 {
                        x: self.current.x + i,
                        y: area.y,
                    },
                    self.layer,
                    area,
                );
            }
        }
    }

    fn draw_gallery(&mut self, gallery: Gallery) {
        for (bound, text) in gallery.items {
            self.context
                .draw(&text.text, text.style, bound.top_left(), self.layer, bound);
        }
    }

    pub fn interact(&mut self, id: Id, area: Rect) -> Response {
        self.context.interact(self.clip, id, area)
    }

    pub fn button<'a>(&mut self, text: impl Into<StyledText<'a>>) -> Response {
        let text = text.into();
        let mut gallery = self.create_gallery(&text);
        let area = self.allocate_area(gallery.bound);
        // assert_eq!(area, gallery.bound);
        gallery.bound = area;
        let response = self.interact(Id::new("As"), gallery.bound);

        if response.pressed() {
            for item in &mut gallery.items {
                item.1.bg(Color::Blue);
            }
        }

        if response.hovered() {
            for item in &mut gallery.items {
                item.1.underline(true);
            }
        }

        self.draw_gallery(gallery);
        response
    }

    pub fn drop_down<'a>(&mut self, title: impl Into<StyledText<'a>>, func: impl FnOnce(&mut Ui)) {
        let mut text: StyledText = title.into();
        let id = Id::new(&text.text);
        let currently_down = self.ctx().get_memory_or(id, false);
        let val = if currently_down {
            match self.layout {
                Layout::TopLeftVertical
                | Layout::TopLeftHorizontal
                | Layout::TopRightVertical
                | Layout::TopRightHorizontal => symbols::pointers::TRIANGLE_DOWN,
                Layout::BottomLeftVertical
                | Layout::BottomLeftHorizontal
                | Layout::BottomRightVertical
                | Layout::BottomRightHorizontal => symbols::pointers::TRIANGLE_UP,
            }
        } else {
            symbols::pointers::TRIANGLE_RIGHT
        };

        match text.to_owned().text {
            std::borrow::Cow::Owned(mut owned_text) => {
                owned_text.push_str(val);
                text.text = std::borrow::Cow::Owned(owned_text);
            }
            std::borrow::Cow::Borrowed(str) => {
                let mut owned_text = str.to_owned();
                owned_text.push_str(val);
                text.text = std::borrow::Cow::Owned(owned_text);
            }
        }
        let button_res = self.button(text);
        if button_res.clicked() {
            self.ctx().insert_into_memory(id, !currently_down);
        }
        self.ctx().check_for_id_clash(id, button_res.rect);

        let layout = self.layout;
        let used = self.horizontal(|ui| {
            ui.add_horizontal_space(1);
            ui.with_layout(layout, |ui| {
                if currently_down {
                    func(ui)
                }
                ui.current
            })
        });

        for i in 0..used.height {
            let x = match self.layout {
                Layout::TopLeftVertical
                | Layout::TopLeftHorizontal
                | Layout::BottomLeftVertical
                | Layout::BottomLeftHorizontal => used.x - 1,
                Layout::TopRightVertical
                | Layout::TopRightHorizontal
                | Layout::BottomRightVertical
                | Layout::BottomRightHorizontal => used.x + used.width,
            };

            self.context.draw(
                VERTICAL,
                Style::default(),
                VecI2 { x, y: used.y + i },
                self.layer,
                //TODO: actaully calculate what our clip should be
                Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(u16::MAX, u16::MAX)),
            );
        }
    }

    fn create_gallery<'a>(&self, text: &'a StyledText<'a>) -> Gallery<'a> {
        self.create_gallery_at(self.cursor, text)
    }

    fn create_gallery_at<'a>(&self, pos: VecI2, text: &'a StyledText<'a>) -> Gallery<'a> {
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

    fn allocate_size(&mut self, desired: VecI2) -> Rect {
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

    pub fn add_space(&mut self, space: VecI2) {
        match self.layout {
            Layout::TopLeftHorizontal | Layout::TopLeftVertical => {
                self.cursor += space;

                self.clip.move_top_left_to(self.cursor);
                self.max_rect.move_top_left_to(self.cursor);
            }
            Layout::TopRightHorizontal | Layout::TopRightVertical => {
                self.cursor += VecI2::new(0, space.y);
                self.cursor -= VecI2::new(space.x, 0);

                self.clip.move_top_right_to(self.cursor);
                self.max_rect.move_top_right_to(self.cursor);
            }
            Layout::BottomLeftHorizontal | Layout::BottomLeftVertical => {
                self.cursor -= VecI2::new(0, space.y);
                self.cursor += VecI2::new(space.x, 0);

                self.clip.move_bottom_left_to(self.cursor);
                self.max_rect.move_bottom_left_to(self.cursor);
            }
            Layout::BottomRightHorizontal | Layout::BottomRightVertical => {
                self.cursor -= VecI2::new(space.x, space.y);

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

    pub fn add_space_primary_direction(&mut self, space: u16) {
        if self.layout.is_primary_horizontal() {
            self.add_space(VecI2::new(space, 0));
        } else {
            self.add_space(VecI2::new(0, space));
        }
    }

    pub fn draw(&mut self, text: &str, style: Style, start: VecI2, clip: Rect) {
        self.context.draw(text, style, start, self.layer, clip)
    }
}

struct Gallery<'a> {
    bound: Rect,
    items: Vec<(Rect, StyledText<'a>)>,
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
            let p1_node = self.vertices.entry(p1).or_insert_with(Default::default);
            if p1.y > p2.y {
                p1_node.down = true;
            } else {
                p1_node.up = true;
            }

            let p2_node = self.vertices.entry(p2).or_insert_with(Default::default);
            if p1.y > p2.y {
                p2_node.up = true;
            } else {
                p2_node.down = true;
            }
            self.lines.push((p1, p2, false))
        } else if p1.y == p2.y {
            let p1_node = self.vertices.entry(p1).or_insert_with(Default::default);
            if p1.x > p2.x {
                p1_node.right = true;
            } else {
                p1_node.left = true;
            }

            let p2_node = self.vertices.entry(p2).or_insert_with(Default::default);
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

    pub fn draw(&self, ctx: &mut Context, style: Style, set: &crate::ui::symbols::line::Set) {
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
