use crossterm::event::KeyModifiers;
use etui::{
    containers::frame::Frame,
    math_util::VecI2,
    start_app,
    style::{Color, DefaultStyle, FromHSV, Style, StyledText},
    widgets::spinner::Spinner,
    App,
};

pub fn main() -> std::io::Result<()> {
    start_app(MyApp::default())
}

pub struct MyApp {
    show_side: bool,
    
    progress_bar: ProgressBar,
    drop_downs: DropDowns,

    cursor: VecI2,
    clicked: bool,
    show: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_side: false,
            progress_bar: ProgressBar::new(),
            drop_downs: DropDowns::new(),
            cursor: VecI2::default(),
            clicked: false,
            show: false,
        }
    }
}

impl App for MyApp {
    fn init(&mut self, ctx: &etui::context::Context) {
        ctx.set_min_tick(std::time::Duration::from_millis(16));
    }

    fn update(&mut self, ctx: &etui::context::Context) {
        self.virtual_mouse(ctx);

        Frame::new().show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.show_side ^= ui.button("UI Info").clicked();
                ui.seperator();
                ui.label(StyledText::styled("F1 to enable arrow/enter cursor", Style::new().forground(crossterm::style::Color::DarkGrey)));
                ui.add_space_primary_direction(u16::MAX);
            });
            ui.seperator();

            ui.horizontal(|ui| {
                if self.show_side {
                    ui.vertical(|ui| {
                        ui.label(format!("frame: {}", ui.ctx().get_frame()));
                        let report = ui.ctx().previous_frame_report();
                        ui.label(format!("bytes_written: {}", report.bytes_written));
                        ui.label(format!("bytes_buffered: {}", report.total_text_len));
                        ui.label(format!("styles_buffered: {}", report.total_styles));
                        if ui.button("Unicode").clicked() {
                            *ui.ctx().style().borrow_mut() = DefaultStyle::new_unicode();
                        }
                        if ui.button("Ascii").clicked() {
                            *ui.ctx().style().borrow_mut() = DefaultStyle::new_ascii();
                        }
                        ui.add_space_primary_direction(u16::MAX);
                    });
                    ui.seperator();
                }

                ui.vertical(|ui| {
                    ui.tabbed_area(
                        etui::id::Id::new("TABS"),
                        ["Progress Bar", "Drop Downs", "Input", "Layouts"],
                        |tab, ui| {
                            ui.bordered(|ui| {
                                ui.with_size(ui.get_max().size(), |ui| match tab {
                                    0 => self.progress_bar.ui(ui),
                                    1 => self.drop_downs.ui(ui),
                                    2 => ui.ctx().clone().input().ui(ui),
                                    3 => layout_fun(ui),
                                    _ => {
                                        let mut text = StyledText::new("How did you get here?");
                                        text.bg(crossterm::style::Color::Red);
                                        ui.label(text);
                                    }
                                });
                            });
                        },
                    );
                });
            });
        });
    }
}

fn layout_fun(ui: &mut etui::ui::Ui) {
    use etui::ui::Layout::*;
    let mut max = ui.get_max();

    let used = ui.vertical(|ui| {
        ui.with_layout(TopLeftHorizontal, |ui| {
            ui.bordered(|ui| {
                ui.label("TopLeft\nHorizontal");
                ui.label("TopLeftHorizontal");
            });
        });

        ui.with_layout(BottomLeftHorizontal, |ui| {
            ui.vertical(|ui| {
                ui.bordered(|ui| {
                    ui.label("TopLeft\nHorizontal");
                    ui.label("TopLeftHorizontal");
                });
                ui.bordered(|ui| {
                    ui.with_layout(TopLeftHorizontal, |ui| {
                        ui.label("In between");
                        ui.add_vertical_space(ui.get_max().height)
                    });
                });
            });
        });
        ui.get_current()
    });
    max.width -= (used.x + used.width) - max.x;
    max.x = used.top_right().x;

    ui.set_max(max);

    let used = ui.with_layout(TopRightHorizontal, |ui| {
        ui.bordered(|ui| {
            ui.label("TopRight\nHorizontal");
            ui.label("TopRightHorizontal");
        });
        ui.get_current()
    });

    max.height -= (used.y) - max.y;
    max.y = used.top_right().y;
    ui.set_max(max);

    ui.with_layout(BottomRightHorizontal, |ui| {
        ui.bordered(|ui| {
            ui.label("BottomRight\nHorizontal");
            ui.label("BottomRightHorizontal");
        });
    });
}

impl MyApp {
    fn virtual_mouse(&mut self, ctx: &etui::context::Context) {
        let mut updated = false;
        if let Some((_, _)) = ctx
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Left)
        {
            self.cursor.x = self.cursor.x.saturating_sub(1);
            updated = true;
        }
        if let Some((_, _)) = ctx
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Right)
        {
            self.cursor.x = self.cursor.x.saturating_add(1);
            updated = true;
        }
        if let Some((_, _)) = ctx
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Up)
        {
            self.cursor.y = self.cursor.y.saturating_sub(1);
            updated = true;
        }
        if let Some((_, _)) = ctx
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Down)
        {
            self.cursor.y = self.cursor.y.saturating_add(1);
            updated = true;
        }

        if ctx.input().keyboard.pressed.get(&crossterm::event::KeyCode::F(1)).is_some(){
            self.show ^= true;
        }
        if !self.show{
            return;
        }
        if ctx.input().mouse.changed{
            self.cursor.x = ctx.input().mouse.position.unwrap_or_default().x;
            self.cursor.y = ctx.input().mouse.position.unwrap_or_default().y;
        }

        if updated {
            ctx.try_input_mut(|input| {
                input.handle_event(crossterm::event::Event::Mouse(
                    crossterm::event::MouseEvent {
                        kind: crossterm::event::MouseEventKind::Moved,
                        column: self.cursor.x,
                        row: self.cursor.y,
                        modifiers: KeyModifiers::empty(),
                    },
                ))
            });
            ctx.request_redraw();
        }

        if self.show{
            ctx.set_cursor(etui::context::Cursor {
                x: self.cursor.x,
                y: self.cursor.y,
            });
        }
        

        if self.clicked {
            ctx.try_input_mut(|input| {
                input.handle_event(crossterm::event::Event::Mouse(
                    crossterm::event::MouseEvent {
                        kind: crossterm::event::MouseEventKind::Up(
                            crossterm::event::MouseButton::Left,
                        ),
                        column: self.cursor.x,
                        row: self.cursor.y,
                        modifiers: KeyModifiers::empty(),
                    },
                ))
            });
            self.clicked = false;
            ctx.request_redraw();
        } else if let Some((_, _)) = ctx
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Enter)
        {
            ctx.try_input_mut(|input| {
                input.handle_event(crossterm::event::Event::Mouse(
                    crossterm::event::MouseEvent {
                        kind: crossterm::event::MouseEventKind::Down(
                            crossterm::event::MouseButton::Left,
                        ),
                        column: self.cursor.x,
                        row: self.cursor.y,
                        modifiers: KeyModifiers::empty(),
                    },
                ))
            });
            self.clicked = true;
            ctx.request_redraw();
        }

    }
}

struct ProgressBar {
    width: u16,
    progress: f32,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            width: 30,
            progress: 0.50,
        }
    }

    fn ui(&mut self, ui: &mut etui::ui::Ui) {
        ui.label(
            "Use left/right arrow keys to increase / decrease the progress (ctrl increases speed)",
        );
        ui.label("Use up/down arrow keys to increase / decrease the size");
        if let Some((KeyModifiers::NONE, _)) = ui
            .ctx()
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Up)
        {
            self.width = self.width.saturating_add(1);
        }
        if let Some((KeyModifiers::NONE, _)) = ui
            .ctx()
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Down)
        {
            self.width = self.width.saturating_sub(1);
        }
        if let Some((modifier, _)) = ui
            .ctx()
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Left)
        {
            let val = if modifier.contains(KeyModifiers::CONTROL) {
                1.0 / (self.width as f32)
            } else if modifier.is_empty() {
                1.0 / (self.width as f32 * 8.0)
            } else {
                0.0
            };
            self.progress = (self.progress - val).clamp(0.0, 1.0)
        }
        if let Some((modifier, _)) = ui
            .ctx()
            .input()
            .keyboard
            .pressed
            .get(&crossterm::event::KeyCode::Right)
        {
            let val = if modifier.contains(KeyModifiers::CONTROL) {
                -1.0 / (self.width as f32)
            } else if modifier.is_empty() {
                -1.0 / (self.width as f32 * 8.0)
            } else {
                0.0
            };
            self.progress = (self.progress - val).clamp(0.0, 1.0)
        }

        let style = etui::style::Style {
            fg: Color::from_hsv(self.progress * 359.999, 1.0, 1.0),
            bg: Color::from_hsv((180.0 + self.progress * 360.0) % 360.0, 1.0, 1.0),
            ..Default::default()
        };
        ui.progress_bar(
            style,
            0,
            self.width,
            1,
            etui::ui::Layout::TopLeftHorizontal,
            self.progress,
        );
    }
}

struct DropDowns {
    data: String,
    value: i32,
}

impl DropDowns {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            value: 0,
        }
    }

    fn ui(&mut self, ui: &mut etui::ui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.drop_down("Drop Down", |ui| {
                    self.data.push_str(&ui.ctx().input().keyboard.frame_input);
                    if ui
                        .ctx()
                        .input()
                        .keyboard
                        .pressed
                        .get(&crossterm::event::KeyCode::Backspace)
                        .is_some()
                    {
                        self.data.pop();
                    }

                    ui.label("input");
                    ui.bordered(|ui| {
                        ui.label(self.data.as_str());
                    });
                });
            });
            ui.add_space_primary_direction(1);

            ui.vertical(|ui| {
                ui.drop_down("Woah Another One", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Funny Spinner: ");
                        Spinner::new().show(ui);
                    });
                    let mut text = StyledText::new("مرحبا اسمي باركر");
                    text.fg(crossterm::style::Color::Red);
                    ui.label(text);
                });
            });
        });
        ui.drop_down("Second One", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Increase").clicked() {
                    self.value += 1;
                }
                ui.add_space_primary_direction(1);
                if ui.button("Decrease").clicked() {
                    self.value -= 1;
                }
            });

            ui.label(format!("value: {}", self.value));
        })
    }
}
