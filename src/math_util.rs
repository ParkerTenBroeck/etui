#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn add_top_left(&mut self, translation: VecI2) {
        self.x = self.x.saturating_sub(translation.x);
        self.y = self.y.saturating_sub(translation.y);

        self.width = self.width.saturating_add(translation.x);
        self.height = self.height.saturating_add(translation.y);
    }

    pub fn sub_top_left(&mut self, translation: VecI2) {
        self.x = self.x.saturating_sub(translation.x);
        self.y = self.y.saturating_sub(translation.y);

        self.width = self.width.saturating_add(translation.x);
        self.height = self.height.saturating_add(translation.y);
    }

    pub fn add_bottom_right(&mut self, translation: VecI2) {
        self.width = self.width.saturating_add(translation.x);
        self.height = self.height.saturating_add(translation.y);
    }

    pub fn sub_bottom_right(&mut self, translation: VecI2) {
        self.width = self.width.saturating_sub(translation.x);
        self.height = self.height.saturating_sub(translation.y);
    }

    pub fn expand_to_include(&mut self, other: &Self) {
        let s_top_left = self.top_left();
        let o_top_left = other.top_left();

        let s_bottom_right = self.bottom_right();
        let o_bottom_right = other.bottom_right();

        let top_left = VecI2::new(
            s_top_left.x.min(o_top_left.x),
            s_top_left.y.min(o_top_left.y),
        );
        let bottom_right = VecI2::new(
            s_bottom_right.x.max(o_bottom_right.x),
            s_bottom_right.y.max(o_bottom_right.y),
        );

        *self = Rect::new_pos_pos(top_left, bottom_right)
    }

    pub fn overlap(&self, other: &Self) -> bool {
        if self.height == 0 || self.width == 0 || other.width == 0 || other.height == 0 {
            return false;
        }
        let sx2 = self.x.saturating_add(self.width);
        let sy2 = self.y.saturating_add(self.height);

        let ox2 = other.x.saturating_add(other.width);
        let oy2 = other.y.saturating_add(other.height);

        if self.x > ox2 || other.x > sx2 {
            return false;
        }

        if sy2 > other.y || oy2 > self.y {
            return false;
        }

        true
    }

    pub fn contains(&self, pos: VecI2) -> bool {
        self.x <= pos.x
            && (self.x.saturating_add(self.width)) > pos.x
            && self.y <= pos.y
            && (self.y.saturating_add(self.height)) > pos.y
    }

    pub fn top_left(&self) -> VecI2 {
        VecI2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn top_right(&self) -> VecI2 {
        VecI2 {
            x: self.x.saturating_add(self.width),
            y: self.y,
        }
    }

    pub fn top_right_inner(&self) -> VecI2 {
        VecI2 {
            x: self.x.saturating_add(self.width).saturating_sub(1),
            y: self.y,
        }
    }

    pub fn bottom_left(&self) -> VecI2 {
        VecI2 {
            x: self.x,
            y: self.y.saturating_add(self.height),
        }
    }

    pub fn bottom_left_inner(&self) -> VecI2 {
        VecI2 {
            x: self.x,
            y: self.y.saturating_add(self.height).saturating_sub(1),
        }
    }

    pub fn bottom_right(&self) -> VecI2 {
        VecI2 {
            x: self.x.saturating_add(self.width),
            y: self.y.saturating_add(self.height),
        }
    }

    pub fn bottom_right_inner(&self) -> VecI2 {
        VecI2 {
            x: self.x.saturating_add(self.width).saturating_sub(1),
            y: self.y.saturating_add(self.height).saturating_sub(1),
        }
    }

    pub fn new_pos_size(pos: VecI2, size: VecI2) -> Rect {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    pub fn new_pos_pos(top_left: VecI2, bottom_right: VecI2) -> Rect {
        let width = bottom_right.x.saturating_sub(top_left.x);
        let height = bottom_right.y.saturating_sub(top_left.y);

        Self {
            x: top_left.x,
            y: top_left.y,
            width,
            height,
        }
    }

    pub fn size(&self) -> VecI2 {
        VecI2 {
            x: self.width,
            y: self.height,
        }
    }

    pub fn expand_evenly(&mut self, ammount: u16) {
        self.x = self.x.saturating_sub(ammount);
        self.y = self.y.saturating_sub(ammount);

        self.width = self.width.saturating_add(ammount);
        self.width = self.width.saturating_add(ammount);

        self.height = self.height.saturating_add(ammount);
        self.height = self.height.saturating_add(ammount);
    }

    pub fn shrink_evenly(&mut self, ammount: u16) {
        self.x = self.x.saturating_add(ammount);
        self.y = self.y.saturating_add(ammount);

        self.width = self.width.saturating_sub(ammount);
        self.width = self.width.saturating_sub(ammount);

        self.height = self.height.saturating_sub(ammount);
        self.height = self.height.saturating_sub(ammount);
    }

    pub fn shrink_to_fit_within(&mut self, max_rect: Rect) {
        let mut bottom_left = self.bottom_right();
        let max_bottom_left = max_rect.bottom_right();

        self.x = self.x.max(max_rect.x);
        self.y = self.y.max(max_rect.y);

        bottom_left.x = bottom_left.x.min(max_bottom_left.x);
        bottom_left.y = bottom_left.y.min(max_bottom_left.y);

        self.width = bottom_left.x.saturating_sub(self.x);
        self.height = bottom_left.y.saturating_sub(self.y);
    }

    pub fn move_top_left_to(&mut self, top_left: VecI2) {
        let bottom_right = self.bottom_right();
        *self = Rect::new_pos_pos(top_left, bottom_right)
    }

    pub fn move_top_right_to(&mut self, top_right: VecI2) {
        let bottom_left = self.bottom_left();
        let top_left = VecI2::new(bottom_left.x, top_right.y);
        let bottom_right = VecI2::new(top_right.x, bottom_left.y);
        *self = Self::new_pos_pos(top_left, bottom_right)
    }

    pub fn move_bottom_left_to(&mut self, bottom_left: VecI2) {
        let top_right = self.top_right();
        let top_left = VecI2::new(bottom_left.x, top_right.y);
        let bottom_right = VecI2::new(top_right.x, bottom_left.y);
        *self = Self::new_pos_pos(top_left, bottom_right)
    }

    pub fn move_bottom_right_to(&mut self, bottom_right: VecI2) {
        *self = Self::new_pos_pos(self.top_left(), bottom_right);
        if self.x > bottom_right.x {
            self.x = bottom_right.x;
            self.width = 0;
        }
        if self.y > bottom_right.y {
            self.y = bottom_right.y;
            self.height = 0;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct VecI2 {
    pub x: u16,
    pub y: u16,
}

impl VecI2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for VecI2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x = self.x.saturating_add(rhs.x);
        self.y = self.y.saturating_add(rhs.y);
        self
    }
}

impl std::ops::AddAssign for VecI2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.saturating_add(rhs.x);
        self.y = self.y.saturating_add(rhs.y);
    }
}

impl std::ops::SubAssign for VecI2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.saturating_sub(rhs.x);
        self.y = self.y.saturating_sub(rhs.y);
    }
}

impl std::ops::Sub for VecI2 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x = self.x.saturating_sub(rhs.x);
        self.y = self.y.saturating_sub(rhs.y);
        self
    }
}
