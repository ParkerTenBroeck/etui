use std::num::NonZeroU8;

use super::{
    math_util::{Rect, VecI2},
    Style,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    str_offset: u32,
    str_len: u8,
    layer: NonZeroU8,
    style_offet: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum CellData {
    Some(Cell),
    Asociated(Asociated),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Asociated {
    None,
    Previous {
        layer: NonZeroU8,
        asociated_x_position: u16,
    },
}

impl Default for CellData {
    fn default() -> Self {
        Self::none()
    }
}

impl CellData {
    pub const fn none() -> Self {
        Self::Asociated(Asociated::None)
    }

    pub const fn asociated(layer: NonZeroU8, asociated_x_position: u16) -> Self {
        Self::Asociated(Asociated::Previous {
            layer,
            asociated_x_position,
        })
    }

    pub const fn some(cell: Cell) -> Self {
        Self::Some(cell)
    }

    pub fn get_layer(&self) -> u8 {
        match self {
            CellData::Some(Cell { layer, .. }) => layer.get(),
            CellData::Asociated(Asociated::None) => 0,
            CellData::Asociated(Asociated::Previous { layer, .. }) => layer.get(),
        }
    }

    pub fn take(&mut self) -> Self {
        let mut res = Self::none();
        std::mem::swap(&mut res, self);
        res
    }
}

#[derive(Debug, Default)]
pub struct Screen {
    cells: Vec<CellData>,
    cells_dismentions: VecI2,
    styles: Vec<Style>,
    text: String,

    last_cell: Option<Cell>,
}

impl Screen {
    pub fn resize(&mut self, size: VecI2) {
        let len = (size.x * size.y) as usize;
        self.cells.reserve(len);
        self.cells.fill(CellData::none());
        self.cells.resize(len, CellData::none());
        self.cells_dismentions = size;
        self.last_cell = None;
        self.text.clear();
        self.styles.clear();
    }

    pub fn size(&self) -> VecI2 {
        self.cells_dismentions
    }

    pub fn push_text(
        &mut self,
        str: &str,
        style: Style,
        mut start: VecI2,
        layer: NonZeroU8,
        clip: Rect,
    ) {
        // assert!(str.len() < 256);
        assert!(self.text.len() + str.len() < u32::MAX as usize);
        assert!(self.cells.len() < u16::MAX as usize);

        let mut style_changed = true;

        if let Some(last) = self.last_cell {
            style_changed = self.cell_style(last) != style;
        }

        let mut cell = Cell {
            str_offset: self.text.len() as u32,
            str_len: 0,
            layer,
            // if the style hasn't changed we use the previous one
            style_offet: self.styles.len() as u16 - u16::from(!style_changed),
        };

        let mut drawn_any = false;
        for char in str.chars() {
            let character_screen_width = unicode_width::UnicodeWidthChar::width(char).unwrap_or(0);

            if let Some(x) = start.x.checked_add(character_screen_width as u16) {
                if x == 0{
                    continue;
                }
                self.text.push(char);

                let character_bytes = char.len_utf8();
                cell.str_len = cell
                    .str_len
                    .checked_add(character_bytes as u8)
                    .expect("Single Cell length exceeded");

                // if the character is zero width we keep the same cell
                if character_screen_width != 0 {
                    if clip.contains(start) {
                        if let Some(last) = self.last_cell {
                            if self.cell_str(last) == self.cell_str(cell) {
                                let char_len = self.cell_str(last).chars().count();
                                for _ in 0..char_len {
                                    //mid
                                    self.text.pop();
                                }
                                assert_eq!(last.str_len, cell.str_len);
                                cell.str_offset = last.str_offset;
                            }
                        }
                        drawn_any |= self.write_cell(cell, character_screen_width as u16, start);
                    }
                    cell.str_len = 0;
                    cell.str_offset = self.text.len() as u32;

                    // when len is zero x should always be equal to start.x
                    // so we just need to update it when its not
                    start.x = x;
                }
            } else {
                break;
            }
        }

        // :3 only add the style if we actually put a cell onto the screen and if the style has changed
        if drawn_any && style_changed {
            self.styles.push(style);
        }
    }

    fn cell_str(&self, cell: Cell) -> &str {
        &self.text[cell.str_offset as usize..cell.str_offset as usize + cell.str_len as usize]
    }

    fn cell_style(&self, cell: Cell) -> Style {
        self.styles[cell.style_offet as usize]
    }

    fn cell_data(&self, cell: Cell) -> (&str, Style) {
        (self.cell_str(cell), self.cell_style(cell))
    }

    fn write_cell(&mut self, cell: Cell, cell_width: u16, position: VecI2) -> bool {
        if Rect::new_pos_size(VecI2::new(0, 0), self.cells_dismentions).contains(position) {
            let index =
                position.x as usize + self.cells_dismentions.x as usize * position.y as usize;
            let last_cell = self.cells[index];
            match last_cell {
                CellData::Some(last_cell) => {
                    if last_cell.layer <= cell.layer {
                        for i in 1..cell_width {
                            if self.cells[i as usize + index].get_layer() > cell.layer.get() {
                                return false;
                            }
                        }
                        for i in 1..cell_width {
                            self.cells[i as usize + index] =
                                CellData::asociated(cell.layer, position.x)
                        }
                        self.cells[index] = CellData::Some(cell);
                        self.last_cell = Some(cell);
                        true
                    } else {
                        false
                    }
                }
                CellData::Asociated(Asociated::None) => {
                    for i in 1..cell_width {
                        if self.cells[i as usize + index].get_layer() > cell.layer.get() {
                            return false;
                        }
                    }
                    for i in 1..cell_width {
                        self.cells[i as usize + index] = CellData::asociated(cell.layer, position.x)
                    }
                    self.cells[index] = CellData::Some(cell);
                    self.last_cell = Some(cell);
                    true
                }
                CellData::Asociated(Asociated::Previous {
                    layer,
                    asociated_x_position,
                }) => {
                    // if the cells layer is infront we can skip computation
                    if layer > cell.layer {
                        return false;
                    }
                    let asociated_cell = last_cell;

                    let mut erase_index = asociated_x_position as usize
                        + self.cells_dismentions.x as usize * position.y as usize;
                    self.cells[erase_index] = CellData::none();
                    erase_index += 1;
                    while self.cells[erase_index] == asociated_cell {
                        self.cells[erase_index] = CellData::none();
                        erase_index += 1;
                    }
                    self.cells[index] = CellData::some(cell);
                    self.last_cell = Some(cell);
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(Default::default());
        self.styles.clear();
        self.text.clear();
    }

    pub fn drain(&mut self) -> ScreenDrain {
        ScreenDrain::new(self)
    }

    pub fn iter(&mut self) -> ScreenIter {
        ScreenIter::new(self)
    }

    pub fn num_styles(&self) -> usize {
        self.styles.len()
    }

    pub fn text_len(&self) -> usize {
        self.text.len()
    }
}

pub trait ScreenCellIterator {
    fn next(&mut self) -> Option<(&str, Style, VecI2)>;
}

pub struct ScreenDrain<'a> {
    iter: ScreenIter<'a>,
}

impl<'a> ScreenCellIterator for ScreenDrain<'a> {
    fn next(&mut self) -> Option<(&str, Style, VecI2)> {
        self.iter.next()
    }
}

impl<'a> ScreenDrain<'a> {
    pub fn new(screen: &'a mut Screen) -> Self {
        Self {
            iter: ScreenIter {
                screen,
                index: 0,
                drain: true,
            },
        }
    }
}

impl<'a> Drop for ScreenDrain<'a> {
    fn drop(&mut self) {
        if !std::thread::panicking(){
            if  self.iter.next().is_some() {
                self.iter.screen.cells.fill(CellData::none())
            }
        }
        self.iter.screen.text.clear();
        self.iter.screen.styles.clear();
        self.iter.screen.last_cell = None;
    }
}

pub struct ScreenIter<'a> {
    screen: &'a mut Screen,
    index: usize,
    drain: bool,
}

impl<'a> ScreenIter<'a> {
    pub fn new(screen: &'a mut Screen) -> Self {
        Self {
            screen,
            index: 0,
            drain: false,
        }
    }
}

impl<'a> ScreenCellIterator for ScreenIter<'a> {
    fn next(&mut self) -> Option<(&str, Style, VecI2)> {
        loop {
            let curr_index = self.index;
            let cell = self.screen.cells.get_mut(self.index)?;
            let cell = if self.drain { cell.take() } else { *cell };
            self.index += 1;
            match cell {
                CellData::Some(cell) => {
                    let dismentions = self.screen.cells_dismentions;
                    let pos = VecI2::new(
                        (curr_index % dismentions.x as usize) as u16,
                        (curr_index / dismentions.x as usize) as u16,
                    );
                    let (cell_text, style) = self.screen.cell_data(cell);
                    return Some((cell_text, style, pos));
                }
                _ => continue,
            }
        }
    }
}
