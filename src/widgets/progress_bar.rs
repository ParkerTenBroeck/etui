use crate::{
    id::Id,
    math_util::VecI2,
    response::Response,
    style::{Attribute, Style, StyledText},
    ui::{Layout, Ui},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressBar {
    pub style: Option<Style>,
    pub min_size: u16,
    pub max_size: u16,
    pub width: u16,
    pub layout: Layout,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            style: None,
            min_size: 1,
            max_size: 10,
            width: 2,
            layout: Layout::TopLeftHorizontal,
        }
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn min_size(mut self, min: u16) -> Self {
        self.max_size = self.max_size.max(min);
        self.min_size = min;
        self
    }

    pub fn max_size(mut self, max: u16) -> Self {
        self.min_size = self.min_size.min(max);
        self.max_size = max;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn show(self, ui: &mut Ui, progress: f32) -> Response {
        let Self {
            style,
            min_size,
            max_size,
            width,
            layout,
        } = self;
        let mut style = style.unwrap_or_else(|| ui.ctx().style().borrow().button_active);
        let mut string = String::new();

        let cursor: VecI2 = ui.get_cursor();

        let (len, area) = if layout.is_primary_horizontal() {
            let size = ui.get_current().width.clamp(min_size, max_size);
            let rect = ui.allocate_size(VecI2::new(size, width));
            (rect.width, rect)
        } else {
            let size = ui.get_current().height.clamp(min_size, max_size);
            let rect = ui.allocate_size(VecI2::new(width, size));
            (rect.height, rect)
        };

        let complete = (len as f32 * progress.clamp(0.0, 1.0) * 8.0) as u32;
        let whole = complete / 8;
        let remaining = ((len as u32 * 8) - complete) / 8;

        let full = if layout.is_primary_vertical() {
            ui.ctx().style().borrow().blocks.full
        } else {
            ui.ctx().style().borrow().bars.full
        };

        for _ in 0..whole {
            for _ in 0..width {
                string.push_str(full);
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
                let bars = ui.ctx().style().borrow().blocks;
                match complete % 8 {
                    0 => bars.empty,
                    1 => bars.one_eighth,
                    2 => bars.one_quarter,
                    3 => bars.three_eighths,
                    4 => bars.half,
                    5 => bars.five_eighths,
                    6 => bars.three_quarters,
                    7 => bars.seven_eighths,
                    // not gonna happen
                    _ => bars.empty,
                }
            } else {
                let bars = ui.ctx().style().borrow().bars;
                match complete % 8 {
                    0 => bars.empty,
                    1 => bars.one_eighth,
                    2 => bars.one_quarter,
                    3 => bars.three_eighths,
                    4 => bars.half,
                    5 => bars.five_eighths,
                    6 => bars.three_quarters,
                    7 => bars.seven_eighths,
                    // not gonna happen
                    _ => bars.empty,
                }
            };

            string.push_str(t);
            if layout.is_primary_vertical() {
                for _ in 0..(width - 1) {
                    string.push_str(t);
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
        let gallery = ui.create_gallery_at(cursor, &text);

        ui.draw_gallery(gallery);

        let id = Id::new(ui.next_id_source());
        ui.interact(id, area)
    }
}
