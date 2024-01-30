use crate::{
    math_util::VecI2, style::Style, symbols::line, ui::Ui
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Separator {
    pub style: Option<Style>,
    pub line: Option<&'static line::Set>,
}

impl Separator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, style: Style) -> Self{
        self.style = Some(style);
        self
    }

    pub fn lines(mut self, lines: &'static line::Set) -> Self{
        self.line = Some(lines);
        self
    }

    pub fn show(self, ui: &mut Ui){
        let lines = self.line.unwrap_or_else(||ui.ctx().style().borrow().lines);
        let style = self.style.unwrap_or_else(||ui.ctx().style().borrow().button);
        
        if ui.layout().is_primary_horizontal() {
            let area = ui.allocate_size(VecI2::new(1, ui.get_current().height));

            for i in 0..area.height {
                ui.ctx().draw(
                    lines.vertical,
                    style,
                    VecI2 {
                        x: area.x,
                        y: ui.get_current().y + i,
                    },
                    ui.layer(),
                    area,
                );
            }
        } else {
            let area = ui.allocate_size(VecI2::new(ui.get_current().width, 1));
            for i in 0..area.width {
                ui.ctx().draw(
                    lines.horizontal,
                    style,
                    VecI2 {
                        x: ui.get_current().x + i,
                        y: area.y,
                    },
                    ui.layer(),
                    area,
                );
            }
        }
    }
}