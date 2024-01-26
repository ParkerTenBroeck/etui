use etui::{
    containers::frame::Frame,
    style::{Style, StyledText},
    widgets::spinner::Spinner,
    App,
};

pub fn main() -> std::io::Result<()> {
    etui::start_app(MyApp::default())
}

#[derive(Default)]
struct MyApp {
    val: i32,
}

impl App for MyApp {
    fn update(&mut self, ctx: &etui::context::Context) {
        Frame::new().show(ctx, |ui| {
            ui.label(StyledText::styled(
                "Hello World",
                Style::new().set_underlined(),
            ));

            ui.horizontal(|ui| {
                if ui.button("Increase").clicked() {
                    self.val += 1;
                }
                ui.add_space_primary_direction(1);
                if ui.button("Decrease").clicked() {
                    self.val -= 1;
                }
            });
            Spinner::new().show(ui);
            ui.label(format!("value: {}", self.val))
        });
    }
}
