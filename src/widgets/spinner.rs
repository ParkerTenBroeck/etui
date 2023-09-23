use crate::{math_util::{VecI2, Rect}, style::Style, ui::Ui};

pub struct Spinner {
    color: Style,
    speed: u32,
    visible_dots: u8,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            color: Default::default(),
            speed: 16*4,
            visible_dots: 5,
        }
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(self, ui: &mut Ui) {
        let area = ui.allocate_size(VecI2::new(2, 1));
        //ui.request_repaint();
        let mut symbols: [u16; 2] = [0; 2];
        let indexes: [(usize, usize, usize); 10] = [
            (0, 0, 0),
            (0, 1, 0),
            (1, 0, 0),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
            (1, 0, 2),
            (0, 1, 2),
            (0, 0, 2),
            (0, 0, 1),
        ];
        let t = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() / self.speed as u128;
        let t = t as usize % 10;
        for i in 0..self.visible_dots {
            let index = indexes[(i as usize + t) % 10];
            symbols[index.0] += crate::symbols::braille::DOTS[index.2][index.1];
        }
        let brail_start: [u8; 3] = [0b11100010, 0b101000_00, 0b10_000000];
        let mut bts1 = brail_start;
        bts1[1] |= symbols[0] as u8 >> 6;
        bts1[2] |= symbols[0] as u8 & 0b111111;
        let mut bts2 = brail_start;
        bts2[1] |= symbols[1] as u8 >> 6;
        bts2[2] |= symbols[1] as u8 & 0b111111;
        let str1 = std::str::from_utf8(&bts1).unwrap();
        let str2 = std::str::from_utf8(&bts2).unwrap();
        //TODO calculate clip correctly
        ui.draw(str1, self.color, area.top_left(), Rect::MAX_SIZE);
        ui.draw(str2, self.color, area.top_right_inner(), Rect::MAX_SIZE);
    }
}
