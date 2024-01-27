use crate::{math_util::VecI2, style::Style, ui::Ui};

pub struct Bordered<'a> {
    border: Option<&'a crate::symbols::line::Set>,
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
            border: None,
            border_style: Style::default(),
        }
    }

    pub fn set_borders(mut self, border: &'a crate::symbols::line::Set) -> Self {
        self.border = Some(border);
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

        let mut border_rect = child.get_current();
        border_rect.expand_evenly(1);

        let border = self
            .border
            .unwrap_or_else(|| ui.ctx().style().borrow().lines);

        ui.draw(
            border.top_left,
            self.border_style,
            border_rect.top_left(),
            border_rect,
        );
        ui.draw(
            border.top_right,
            self.border_style,
            border_rect.top_right_inner(),
            border_rect,
        );
        ui.draw(
            border.bottom_right,
            self.border_style,
            border_rect.bottom_right_inner(),
            border_rect,
        );
        ui.draw(
            border.bottom_left,
            self.border_style,
            border_rect.bottom_left_inner(),
            border_rect,
        );

        for i in 1..(border_rect.width - 1) {
            ui.draw(
                border.horizontal,
                self.border_style,
                VecI2 {
                    x: border_rect.x + i,
                    y: border_rect.y,
                },
                border_rect,
            );
            ui.draw(
                border.horizontal,
                self.border_style,
                VecI2 {
                    x: border_rect.x + i,
                    y: border_rect.bottom_right_inner().y,
                },
                border_rect,
            );
        }

        for i in 1..(border_rect.height - 1) {
            ui.draw(
                border.vertical,
                self.border_style,
                VecI2 {
                    x: border_rect.x,
                    y: border_rect.y + i,
                },
                border_rect,
            );
            ui.draw(
                border.vertical,
                self.border_style,
                VecI2 {
                    x: border_rect.bottom_right_inner().x,
                    y: border_rect.y + i,
                },
                border_rect,
            );
        }

        _ = ui.allocate_area(border_rect);
        res
    }
}
