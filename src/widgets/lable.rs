use crate::{style::StyledText, ui::Ui};


#[derive(Clone, Debug)]
pub struct Label<'a> {
    pub text: StyledText<'a>,
}

impl<'a> Label<'a> {
    pub fn new(text: impl Into<StyledText<'a>>) -> Self {
        Self { text: text.into() }
    }

    pub fn show(self, ui: &mut Ui){
        let gallery = ui.create_gallery(&self.text);
        ui.allocate_area(gallery.bound);
        ui.draw_gallery(gallery);
    }
}