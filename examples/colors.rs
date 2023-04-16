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

                    ui.progress_bar(
                        style,
                        16,
                        16,
                        2,
                        etui::ui::Layout::BottomLeftVertical,
                        (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                    );
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

                        ui.progress_bar(
                            style,
                            max,
                            max,
                            1,
                            etui::ui::Layout::TopRightHorizontal,
                            (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                        );
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

                        ui.progress_bar(
                            style,
                            max,
                            max,
                            1,
                            etui::ui::Layout::TopLeftHorizontal,
                            (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                        );
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

                        ui.progress_bar(
                            style,
                            max / 2,
                            max / 2,
                            2,
                            etui::ui::Layout::TopLeftVertical,
                            (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                        );
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

                        ui.progress_bar(
                            style,
                            max / 2,
                            max / 2,
                            2,
                            etui::ui::Layout::BottomLeftVertical,
                            (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                        );
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

                    ui.progress_bar(
                        style,
                        16 * 2,
                        16 * 2,
                        1,
                        etui::ui::Layout::TopLeftHorizontal,
                        (percent * std::f32::consts::TAU).sin() / 2.0 + 0.5,
                    );
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
