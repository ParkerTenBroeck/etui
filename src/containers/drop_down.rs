use std::hash::Hash;

use crate::{context::Context, id::Id, math_util::{Rect, VecI2}, response::Response, style::{Style, StyledText}, ui::{Layout, Ui}};

#[derive(Debug, Clone)]
pub struct DropDown<'a> {
    id: Id,
    header: StyledText<'a>,
    arrow: &'a crate::symbols::pointers::Set,
    lines: &'a crate::symbols::line::Set,
    default_shown: bool,
}

impl<'a> DropDown<'a> {
    pub fn new(header: impl Into<StyledText<'a>>) -> Self {
        let header: StyledText = header.into();

        Self {
            id: Id::new(header.text.as_ref()),
            header,
            arrow: &crate::symbols::pointers::TRIANGLE,
            lines: &crate::symbols::line::NORMAL,
            default_shown: false,
        }
    }

    pub fn with_id(mut self, source: impl Hash) -> Self {
        self.id = Id::new(source);
        self
    }

    pub fn arrow_style(mut self, set: &'a crate::symbols::pointers::Set) -> Self {
        self.arrow = set;
        self
    }

    pub fn line_style(mut self, set: &'a crate::symbols::line::Set) -> Self {
        self.lines = set;
        self
    }

    pub fn default_shown(mut self, shown: bool) -> Self{
        self.default_shown = shown;
        self
    }

    pub fn is_shown(&self, ctx: &Context) -> bool {
        ctx.get_memory_or(self.id, self.default_shown)
    }

    pub fn set_shown(&self, ctx: &Context, shown: bool) {
        ctx.insert_into_memory(self.id, shown)
    }

    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        func: impl FnOnce(&mut Ui, &mut Self) -> R,
    ) -> DropDownResponse<R> {
        // let header_res = ui.button(&self.header);
        // if header_res.clicked() {
        //     self.set_shown(ui.ctx(), !self.is_shown(ui.ctx()))
        // }
        // ui.ctx().check_for_id_clash(self.id, header_res.rect);

        // let res = if self.is_shown(ui.ctx()) {
        //     let mut child = ui.child_ui(ui.get_clip(), ui.layout());
        //     Some(func(&mut child, &mut self))
        // } else {
        //     None
        // };

        let currently_down = ui.ctx().get_memory_or(self.id, false);
        let val = if currently_down {
            match ui.layout() {
                Layout::TopLeftVertical
                | Layout::TopLeftHorizontal
                | Layout::TopRightVertical
                | Layout::TopRightHorizontal => self.arrow.down,
                Layout::BottomLeftVertical
                | Layout::BottomLeftHorizontal
                | Layout::BottomRightVertical
                | Layout::BottomRightHorizontal => self.arrow.up,
            }
        } else {
            self.arrow.right
        };

        match self.header.to_owned().text {
            std::borrow::Cow::Owned(mut owned_text) => {
                owned_text.push_str(val);
                self.header.text = std::borrow::Cow::Owned(owned_text);
            }
            std::borrow::Cow::Borrowed(str) => {
                let mut owned_text = str.to_owned();
                owned_text.push_str(val);
                self.header.text = std::borrow::Cow::Owned(owned_text);
            }
        }
        let button_res = ui.button(&self.header);
        if button_res.clicked() {
            ui.ctx().insert_into_memory(self.id, !currently_down);
        }
        ui.ctx().check_for_id_clash(self.id, button_res.rect);

        let layout = ui.layout();
        let (res, used) = ui.horizontal(|ui| {
            ui.add_horizontal_space(1);
            ui.with_layout(layout, |ui| {
                let res = if currently_down {
                    Some(func(ui, self))
                }else{
                    None
                };
                (res, ui.get_current())
            })
        });

        for i in 0..used.height {
            let x = match ui.layout() {
                Layout::TopLeftVertical
                | Layout::TopLeftHorizontal
                | Layout::BottomLeftVertical
                | Layout::BottomLeftHorizontal => used.x - 1,
                Layout::TopRightVertical
                | Layout::TopRightHorizontal
                | Layout::BottomRightVertical
                | Layout::BottomRightHorizontal => used.x + used.width,
            };

            ui.ctx().draw(
                self.lines.vertical,
                Style::default(),
                VecI2 { x, y: used.y + i },
                ui.layer(),
                //TODO: actaully calculate what our clip should be
                Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(u16::MAX, u16::MAX)),
            );
        }

        DropDownResponse {
            inner_return: res,
            header_res: button_res,
        }
    }
}

pub struct DropDownResponse<R> {
    pub header_res: Response,
    pub inner_return: Option<R>,
}

// let mut text: StyledText = title.into();
//         let id = Id::new(&text.text);
//         let currently_down = self.ctx().get_memory_or(id, false);
//             let val = if currently_down {
//                 match self.layout {
//                     Layout::TopLeftVertical
//                     | Layout::TopLeftHorizontal
//                     | Layout::TopRightVertical
//                     | Layout::TopRightHorizontal => symbols::pointers::TRIANGLE_DOWN,
//                     Layout::BottomLeftVertical
//                     | Layout::BottomLeftHorizontal
//                     | Layout::BottomRightVertical
//                     | Layout::BottomRightHorizontal => symbols::pointers::TRIANGLE_UP,
//                 }
//             } else {
//                 symbols::pointers::TRIANGLE_RIGHT
//             };

//             match text.to_owned().text {
//                 std::borrow::Cow::Owned(mut owned_text) => {
//                     owned_text.push_str(val);
//                     text.text = std::borrow::Cow::Owned(owned_text);
//                 }
//                 std::borrow::Cow::Borrowed(str) => {
//                     let mut owned_text = str.to_owned();
//                     owned_text.push_str(val);
//                     text.text = std::borrow::Cow::Owned(owned_text);
//                 }
//             }
//             let button_res = self.button(text);
//             if button_res.clicked() {
//                 self.ctx().insert_into_memory(id, !currently_down);
//             }
//             self.ctx().check_for_id_clash(id, button_res.rect);

//             let layout = self.layout;
//             let used = self.horizontal(|ui| {
//                 ui.add_horizontal_space(1);
//                 ui.with_layout(layout, |ui| {
//                     if currently_down {
//                         func(ui)
//                     }
//                     ui.current
//                 })
//             });

//             for i in 0..used.height {
//                 let x = match self.layout {
//                     Layout::TopLeftVertical
//                     | Layout::TopLeftHorizontal
//                     | Layout::BottomLeftVertical
//                     | Layout::BottomLeftHorizontal => used.x - 1,
//                     Layout::TopRightVertical
//                     | Layout::TopRightHorizontal
//                     | Layout::BottomRightVertical
//                     | Layout::BottomRightHorizontal => used.x + used.width,
//                 };

//                 self.context.draw(
//                     VERTICAL,
//                     Style::default(),
//                     VecI2 { x, y: used.y + i },
//                     self.layer,
//                     //TODO: actaully calculate what our clip should be
//                     Rect::new_pos_size(VecI2::new(0, 0), VecI2::new(u16::MAX, u16::MAX)),
//                 );
//             }
