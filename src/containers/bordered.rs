use crate::{math_util::VecI2, style::Style, ui::Ui};

pub struct Bordered<'a> {
    border: &'a crate::symbols::line::Set,
    border_style: Style,
}

impl<'a> Default for Bordered<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Bordered<'a> {
    pub fn new() -> Self {
        Self {
            border: &crate::symbols::line::NORMAL,
            border_style: Style::default(),
        }
    }

    pub fn set_borders(mut self, border: &'a crate::symbols::line::Set) -> Self {
        self.border = border;
        self
    }

    pub fn set_borders_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, func: impl FnOnce(&mut Ui) -> R) -> R {
        let mut start_max_rect = ui.get_max();
        start_max_rect.shrink_evenly(1);

        let mut child = ui.child_ui(start_max_rect, ui.layout());

        let res = func(&mut child);

        let mut border = child.get_current();
        border.expand_evenly(1);

        ui.draw(
            self.border.top_left,
            self.border_style,
            border.top_left(),
            border,
        );
        ui.draw(
            self.border.top_right,
            self.border_style,
            border.top_right_inner(),
            border,
        );
        ui.draw(
            self.border.bottom_right,
            self.border_style,
            border.bottom_right_inner(),
            border,
        );
        ui.draw(
            self.border.bottom_left,
            self.border_style,
            border.bottom_left_inner(),
            border,
        );

        for i in 1..(border.width - 1) {
            ui.draw(
                self.border.horizontal,
                self.border_style,
                VecI2 {
                    x: border.x + i,
                    y: border.y,
                },
                border,
            );
            ui.draw(
                self.border.horizontal,
                self.border_style,
                VecI2 {
                    x: border.x + i,
                    y: border.bottom_right_inner().y,
                },
                border,
            );
        }

        for i in 1..(border.height - 1) {
            ui.draw(
                self.border.vertical,
                self.border_style,
                VecI2 {
                    x: border.x,
                    y: border.y + i,
                },
                border,
            );
            ui.draw(
                self.border.vertical,
                self.border_style,
                VecI2 {
                    x: border.bottom_right_inner().x,
                    y: border.y + i,
                },
                border,
            );
        }

        _ = ui.allocate_area(border);
        res
    }
}
