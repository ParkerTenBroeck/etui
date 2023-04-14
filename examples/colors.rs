use etui::{
    containers::frame::Frame,
    style::{Color, Style, StyledText},
    App,
};

pub fn main() -> std::io::Result<()> {
    etui::start_app(MyApp::default())
}

#[derive(Default)]
struct MyApp {}

impl App for MyApp {
    fn update(&mut self, ctx: &etui::context::Context) {
        Frame::new().show(ctx, |ui| {
            let percent = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                % 3000000000) as f32
                / 3000000000.0;

            let hue = percent * 360.0;

            ui.label(format!("H: {:.02}", hue));
            let (r, g, b) = hsv2rgb(hue, 1.0, 1.0);
            let style = Style::new().forground(Color::Rgb { r, g, b });

            ui.progress_bar(
                style,
                11,
                11,
                1,
                etui::ui::Layout::TopLeftHorizontal,
                percent,
            );

            let style = style.set_underlined();
            ui.label(StyledText::styled("Hello World", style));
        });
    }
}

pub fn hsv2rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c: f32 = v * s;
    let x: f32 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m: f32 = v - c;

    let mut r1: f32 = 0.0;
    let mut g1: f32 = 0.0;
    let mut b1: f32 = 0.0;

    if h < 60.0 {
        r1 = c;
        g1 = x;
        b1 = 0.0;
    } else if (60.0..120.0).contains(&h) {
        r1 = x;
        g1 = c;
        b1 = 0.0;
    } else if (120.0..180.0).contains(&h) {
        r1 = 0.0;
        g1 = c;
        b1 = x;
    } else if (180.0..240.0).contains(&h) {
        r1 = 0.0;
        g1 = x;
        b1 = c;
    } else if (240.0..300.0).contains(&h) {
        r1 = x;
        g1 = 0.0;
        b1 = c;
    } else if (300.0..360.0).contains(&h) {
        r1 = c;
        g1 = 0.0;
        b1 = x;
    }

    (
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}
