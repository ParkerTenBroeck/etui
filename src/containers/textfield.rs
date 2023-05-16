use std::num::Wrapping;

use crate::{math_util::VecI2, style::Style, ui::Ui};

fn wordwrap(line: &str, wrapindex: u16) -> Vec<String> {
    let mut wrapped = Vec::<String>::new();

    let mut stringwrap = String::from("");

    let mut linepos: u16 = 0;
    let mut hardpos: u16 = 0;

    for i in line.chars() {
        println!(
            "entering with char {}, linepos {}, and hardpos {}",
            i, linepos, hardpos
        );

        if linepos >= wrapindex {
            wrapped.push(stringwrap.clone());
            println!("pushing {}", &stringwrap);
            stringwrap = String::from("");
            linepos = 0;
        }

        if i.is_whitespace() {
            let mut lookrelpos = linepos + 1;

            let mut iterate = line.chars();
            let mut val = iterate.nth((1 + hardpos).into());

            'inner: while val.is_some() {
                if val.unwrap().is_whitespace() {
                    break 'inner;
                }
                lookrelpos += 1;

                if lookrelpos >= wrapindex {
                    wrapped.push(stringwrap.clone());
                    println!("pushing {}", &stringwrap);
                    stringwrap = String::from("");
                    linepos = 0;

                    break 'inner;
                }
                val = iterate.next()
            }
        }
        stringwrap.push(i);
        linepos += 1;
        hardpos += 1;
    }
    return wrapped;
}

pub struct TextField<'a> {
    border: &'a crate::symbols::line::Set,
    border_style: Style,
    text_style: Style,
    cursorpos: VecI2,
    contents: String,
    numbered: bool,
    scroll_line: u32, //line at the top of the screen based on scrolling
}

impl<'a> Default for TextField<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TextField<'a> {
    pub fn new() -> Self {
        Self {
            border: &crate::symbols::line::NORMAL,
            border_style: Style::default(),
            text_style: Style::default(),
            cursorpos: VecI2 { x: 0, y: 0 },
            contents: "".to_owned(),
            numbered: true,
            scroll_line: 0,
        }
    }

    pub fn set_borders(mut self, border: &'a crate::symbols::line::Set) -> Self {
        self.border = border;
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

        let mut border = child.get_current();
        border.expand_evenly(1);

        ui.draw(
            self.border.top_left,
            self.border_style,
            border.top_left(),
            border,
        );
        ui.draw(
            self.border.top_right,
            self.border_style,
            border.top_right_inner(),
            border,
        );
        ui.draw(
            self.border.bottom_right,
            self.border_style,
            border.bottom_right_inner(),
            border,
        );
        ui.draw(
            self.border.bottom_left,
            self.border_style,
            border.bottom_left_inner(),
            border,
        );

        for i in 1..(border.width - 1) {
            ui.draw(
                self.border.horizontal,
                self.border_style,
                VecI2 {
                    x: border.x + i,
                    y: border.y,
                },
                border,
            );
            ui.draw(
                self.border.horizontal,
                self.border_style,
                VecI2 {
                    x: border.x + i,
                    y: border.bottom_right_inner().y,
                },
                border,
            );
        }

        for i in 1..(border.height - 1) {
            ui.draw(
                self.border.vertical,
                self.border_style,
                VecI2 {
                    x: border.x,
                    y: border.y + i,
                },
                border,
            );
            ui.draw(
                self.border.vertical,
                self.border_style,
                VecI2 {
                    x: border.bottom_right_inner().x,
                    y: border.y + i,
                },
                border,
            );
        }
        //-------------------------------------------------------------------------------------------

        let mut soft_count: u16 = 0; //lines with wrap to current pos
        let mut hard_count: u16 = 0; //lines without wrap to current pos
        let lines = self.contents.split("\n");
        let ofset: u16 = {
            if self.numbered {
                4
            } else {
                0
            }
        };
        let cutoff: u16 = border.width - ofset - 2;

        for line in lines {
            soft_count = soft_count + 1;
            hard_count = hard_count + 1;

            if self.numbered {
                ui.draw(
                    &hard_count.to_string(),
                    self.border_style,
                    VecI2 {
                        x: border.bottom_left().x + 1,
                        y: border.y + soft_count,
                    },
                    border,
                );
                ui.draw(
                    self.border.vertical,
                    self.border_style,
                    VecI2 {
                        x: border.bottom_left().x + 4,
                        y: border.y + soft_count,
                    },
                    border,
                );
            }

            let hardlinelen = line.chars().count();

            if hardlinelen > cutoff.into() {
                let wrapped = wordwrap(line, cutoff);
                soft_count -= 1;
                for line in wrapped.iter() {
                    soft_count += 1;
                    ui.draw(
                        line,
                        self.text_style,
                        VecI2 {
                            x: (border.bottom_left().x + ofset+1),
                            y: (soft_count),
                        },
                        border,
                    );
                }
            }
        }

        _ = ui.allocate_area(border);
        res
    }
}
