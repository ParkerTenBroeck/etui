use crossterm::style::Color;
use etui::{
    containers::{bordered::Bordered, frame::Frame},
    style::Style,
    App,
};

pub fn main() -> std::io::Result<()> {
    etui::start_app(Borders {})
}

struct Borders {}

impl App for Borders {
    fn update(&mut self, ctx: &etui::context::Context) {
        Frame::new().show(ctx, |ui| {
            Bordered::new()
                .set_borders_style(Style::new().forground(Color::Red))
                .show(ui, |ui| ui.label("Wow this is cool"));

            Bordered::new()
                .set_borders(&etui::symbols::line::DOUBLE)
                .set_borders_style(Style::new().forground(Color::Green))
                .show(ui, |ui| ui.label("Wow this is even cooler"));
            ui.bordered(|test| test.label("Bruh"))
        });
    }
}
