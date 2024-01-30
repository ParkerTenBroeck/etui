use etui::{
    containers::frame::Frame,
    style::{Color, Style, StyledText},
    widgets::progress_bar::ProgressBar,
    App,
};

pub fn main() -> std::io::Result<()> {
    etui::start_app(MyApp::default())
}

#[derive(Default)]
struct MyApp {}

impl App for MyApp {
    fn init(&mut self, ctx: &etui::context::Context) {
        ctx.set_min_tick(std::time::Duration::from_millis(16));
    }

    fn update(&mut self, ctx: &etui::context::Context) {
        ctx.request_redraw();
        Frame::new().show(ctx, |ui| {
            ui.bordered(|ui| {
                very_colorful(ui);
            });

            let style = Style::default();
            ui.label(StyledText::styled("Hello World", style));
            ui.label(format!("{:#?}", ui.ctx().previous_frame_report()))
        });
    }
}

fn very_colorful(ui: &mut etui::ui::Ui) {
    let percent = time_period(3000000000);
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                for i in 0..16 {
                    let percent = (percent + i as f32 / 16.0) % 1.0;
                    let hue = percent * 360.0;

                    let color = hsv2rgb(hue, 1.0, 1.0);
                    let color2 = hsv2rgb((hue + 180.0) % 360.0, 1.0, 1.0);
                    let style = Style::new().forground(color).background(color2);

                    ProgressBar::new()
                        .style(style)
                        .min_size(16)
                        .width(2)
                        .layout(etui::ui::Layout::BottomLeftVertical)
                        .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                }
            });
            let max = 16;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    for i in 0..16 {
                        let percent = (percent + i as f32 / 16.0) % 1.0;
                        let hue = percent * 360.0;

                        let color = hsv2rgb(hue, 1.0, 1.0);
                        let style = Style::new().forground(color).background(hsv2rgb(
                            (180.0 + hue) % 360.0,
                            1.0,
                            1.0,
                        ));

                        ProgressBar::new()
                            .style(style)
                            .min_size(max)
                            .width(1)
                            .layout(etui::ui::Layout::TopRightHorizontal)
                            .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                    }
                });
                ui.vertical(|ui| {
                    for i in 0..16 {
                        let percent = (percent + i as f32 / 16.0) % 1.0;
                        let hue = percent * 360.0;

                        let color = hsv2rgb(hue, 1.0, 1.0);
                        let style = Style::new().forground(color).background(hsv2rgb(
                            (180.0 + hue) % 360.0,
                            1.0,
                            1.0,
                        ));

                        ProgressBar::new()
                            .style(style)
                            .min_size(max)
                            .width(1)
                            .layout(etui::ui::Layout::TopLeftHorizontal)
                            .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                    }
                });
            });
        });
        ui.vertical(|ui| {
            let max = 16;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    for i in 0..16 {
                        let percent = (percent + i as f32 / 16.0) % 1.0;
                        let hue = percent * 360.0;

                        let color = hsv2rgb(hue, 1.0, 1.0);
                        let style = Style::new().forground(color).background(hsv2rgb(
                            (180.0 + hue) % 360.0,
                            1.0,
                            1.0,
                        ));

                        ProgressBar::new()
                            .style(style)
                            .min_size(max / 2)
                            .width(2)
                            .layout(etui::ui::Layout::TopLeftVertical)
                            .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                    }
                });
                ui.horizontal(|ui| {
                    for i in 0..16 {
                        let percent = (percent + i as f32 / 16.0) % 1.0;
                        let hue = percent * 360.0;

                        let color = hsv2rgb(hue, 1.0, 1.0);
                        let style = Style::new().forground(color).background(hsv2rgb(
                            (180.0 + hue) % 360.0,
                            1.0,
                            1.0,
                        ));

                        ProgressBar::new()
                            .style(style)
                            .min_size(max / 2)
                            .width(2)
                            .layout(etui::ui::Layout::BottomLeftVertical)
                            .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                    }
                });
            });

            ui.vertical(|ui| {
                for i in 0..16 {
                    let percent = (percent + i as f32 / 16.0) % 1.0;
                    let hue = percent * 360.0;

                    let color = hsv2rgb(hue, 1.0, 1.0);
                    let color2 = hsv2rgb((hue + 180.0) % 360.0, 1.0, 1.0);
                    let style = Style::new().forground(color).background(color2);

                    ProgressBar::new()
                        .style(style)
                        .min_size(16 * 2)
                        .width(1)
                        .layout(etui::ui::Layout::TopLeftHorizontal)
                        .show(ui, (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5);
                }
            });
        });
    });
}

pub fn time_period(nanos: u128) -> f32 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        % nanos) as f32
        / nanos as f32
}

pub fn hsv2rgb(h: f32, s: f32, v: f32) -> Color {
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

    Color::Rgb {
        r: ((r1 + m) * 255.0) as u8,
        g: ((g1 + m) * 255.0) as u8,
        b: ((b1 + m) * 255.0) as u8,
    }
}
