use std::num::NonZeroU8;

use crate::{
    context::Context,
    ui::{Layout, Ui},
};

#[derive(Default)]
pub struct Frame {
    layout: Option<Layout>,
}

impl Frame {
    pub fn new() -> Self {
        Self { layout: None }
    }

    pub fn show<F: FnOnce(&mut Ui) -> R, R>(self, ctx: &Context, func: F) -> R {
        let layout = self.layout.unwrap_or(Layout::TopLeftVertical);
        func(&mut Ui::new(
            ctx.clone(),
            layout,
            ctx.screen_rect(),
            NonZeroU8::new(128).unwrap(),
        ))
    }

    pub fn set_layout(mut self, layout: Layout) -> Self {
        self.layout = Some(layout);
        self
    }
}
