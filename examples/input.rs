use etui::{containers::frame::Frame, start_app, App};

pub fn main() -> std::io::Result<()> {
    start_app(InputDemo {})
}

struct InputDemo {}

impl App for InputDemo {    
    fn update(&mut self, ctx: &etui::context::Context) {
        Frame::new().show(ctx, |ui| {
            ui.label(format!("frame: {}", ctx.get_frame()));
            ctx.input(|input|{
                input.clone()
            }).ui(ui)
        });
    }
}
