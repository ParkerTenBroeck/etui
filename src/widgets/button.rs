use crate::{id::Id, response::Response, style::{Color, StyledText}, ui::Ui};

#[derive(Clone, Debug)]
pub struct Button<'a> {
    pub text: StyledText<'a>,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<StyledText<'a>>) -> Self {
        Self { text: text.into() }
    }

    pub fn show(self, ui: &mut Ui) -> Response{
        let mut gallery = ui.create_gallery(&self.text);
        let area: crate::math_util::Rect = ui.allocate_area(gallery.bound);

        gallery.bound = area;

        let id = Id::new(ui.next_id_source());
        let response = ui.interact(id, gallery.bound);

        if response.pressed() {
            for item in &mut gallery.items {
                item.1.bg(Color::Blue);
            }
        }

        if response.hovered() {
            for item in &mut gallery.items {
                item.1.underline(true);
            }
        }

        ui.draw_gallery(gallery);
        response
    }
}