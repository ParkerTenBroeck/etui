use etui::{containers::frame::Frame, start_app, ui::Ui, App};

pub fn main() -> std::io::Result<()> {
    start_app(MyApp {})
}

pub struct MyApp {}

impl App for MyApp {
    fn update(&mut self, ctx: &etui::context::Context) {
        Frame::new().show(ctx, |ui| test_layout_text(ui));
    }
}

fn test_layout_text(ui: &mut Ui) {
    use etui::ui::Layout::*;
    ui.label(format!("{:#?}", ui.ctx().previous_frame_report()));

    ui.tabbed_area(
        etui::id::Id::new("TABS"),
        ["Vertical", "Horizontal"],
        |tab, ui| {
            ui.bordered(|ui| {
                ui.with_size(ui.get_max().size(), |ui| {
                    if tab == 1 {
                        let max = ui.get_max();

                        ui.with_layout(TopLeftHorizontal, |ui| {
                            ui.bordered(|ui| {
                                ui.label("TopLeft\nHorizontal");
                                ui.label("TopLeftHorizontal");
                            });
                        });

                        ui.with_layout(BottomLeftHorizontal, |ui| {
                            ui.bordered(|ui| {
                                ui.label("TopLeft\nHorizontal");
                                ui.label("TopLeftHorizontal");
                            });
                        });

                        ui.set_max(max);

                        ui.with_layout(TopRightHorizontal, |ui| {
                            ui.bordered(|ui| {
                                ui.label("TopRight\nHorizontal");
                                ui.label("TopRightHorizontal");
                            });
                        });

                        ui.set_max(max);

                        ui.with_layout(BottomRightHorizontal, |ui| {
                            ui.bordered(|ui| {
                                ui.label("BottomRight\nHorizontal");
                                ui.label("BottomRightHorizontal");
                            });
                        });
                    } else {
                        let max = ui.get_max();

                        ui.with_layout(TopLeftVertical, |ui| {
                            ui.bordered(|ui| {
                                ui.label("TopLeft\nVertical");
                                ui.label("TopLeftVertical");
                            });
                            drop_down(ui, "3");
                            drop_down(ui, "5")
                        });

                        ui.with_layout(BottomLeftVertical, |ui| {
                            ui.bordered(|ui| {
                                ui.label("BottomLeft\nVertical");
                                ui.label("BottomLeftVertical");
                            });
                            drop_down(ui, "3");
                            drop_down(ui, "6")
                        });

                        ui.set_max(max);

                        ui.with_layout(TopRightVertical, |ui| {
                            ui.bordered(|ui| {
                                ui.label("TopRight\nVertical");
                                ui.label("TopRightVertical");
                            });
                            drop_down(ui, "2")
                        });

                        ui.set_max(max);

                        ui.with_layout(BottomRightVertical, |ui| {
                            ui.bordered(|ui| {
                                drop_down(ui, "12");
                                ui.label("BottomRight\nVertical");
                                ui.label("BottomRightVertical");
                                drop_down(ui, "123");
                            });
                            drop_down(ui, "1");
                            ui.label("asdasd")
                        });
                    }
                });
            });
        },
    );
}

fn drop_down(ui: &mut etui::ui::Ui, title: &str) {
    ui.drop_down(title, |ui| {
        ui.label("Bruh");
        if ui.button("bruh").pressed() {
            ui.label("asdasd")
        }
    });
}
