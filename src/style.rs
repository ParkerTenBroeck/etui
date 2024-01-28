use std::borrow::Cow;

use crate::symbols;

pub type Attributes = crossterm::style::Attributes;
pub type Color = crossterm::style::Color;
pub type Attribute = crossterm::style::Attribute;

pub trait FromHSV {
    fn from_hsv(h: f32, s: f32, v: f32) -> Self;
}

impl FromHSV for Color {
    fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let c: f32 = v * s;
        let x: f32 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m: f32 = v - c;

        let mut r1: f32 = 0.0;
        let mut g1: f32 = 0.0;
        let mut b1: f32 = 0.0;

        if h < 60.0 {
            r1 = c;
            g1 = x;
            b1 = 0.0;
        } else if (60.0..120.0).contains(&h) {
            r1 = x;
            g1 = c;
            b1 = 0.0;
        } else if (120.0..180.0).contains(&h) {
            r1 = 0.0;
            g1 = c;
            b1 = x;
        } else if (180.0..240.0).contains(&h) {
            r1 = 0.0;
            g1 = x;
            b1 = c;
        } else if (240.0..300.0).contains(&h) {
            r1 = x;
            g1 = 0.0;
            b1 = c;
        } else if (300.0..360.0).contains(&h) {
            r1 = c;
            g1 = 0.0;
            b1 = x;
        }

        Color::Rgb {
            r: ((r1 + m) * 255.0) as u8,
            g: ((g1 + m) * 255.0) as u8,
            b: ((b1 + m) * 255.0) as u8,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StyledText<'a> {
    pub text: Cow<'a, str>,
    pub style: Style,
}

impl<'a> StyledText<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
        }
    }

    pub fn fg(&mut self, color: Color) {
        self.style.fg = color;
    }

    pub fn bg(&mut self, color: Color) {
        self.style.bg = color;
    }

    pub fn modifiers(&mut self, attributes: Attributes) {
        self.style.attributes = attributes;
    }

    pub fn underline(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Underlined);
        } else {
            self.style.attributes.unset(Attribute::Underlined);
        }
    }

    pub fn bold(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Bold);
        } else {
            self.style.attributes.unset(Attribute::Bold);
        }
    }

    pub fn slow_blink(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::SlowBlink);
        } else {
            self.style.attributes.unset(Attribute::SlowBlink);
        }
    }

    pub fn rapid_blink(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::RapidBlink);
        } else {
            self.style.attributes.unset(Attribute::RapidBlink);
        }
    }

    pub fn italic(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Italic);
        } else {
            self.style.attributes.unset(Attribute::Italic);
        }
    }

    pub fn dim(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Dim);
        } else {
            self.style.attributes.unset(Attribute::Dim);
        }
    }

    pub fn crossed_out(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::CrossedOut);
        } else {
            self.style.attributes.unset(Attribute::CrossedOut);
        }
    }

    pub fn hidden(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Hidden);
        } else {
            self.style.attributes.unset(Attribute::Hidden);
        }
    }

    pub fn reversed(&mut self, show: bool) {
        if show {
            self.style.attributes.set(Attribute::Reverse);
        } else {
            self.style.attributes.unset(Attribute::Reverse);
        }
    }

    pub fn styled(text: impl Into<Cow<'a, str>>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn forground(mut self, forground: Color) -> Self {
        self.fg = forground;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.bg = background;
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn set_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.set(attribute);
        self
    }

    pub fn unset_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.unset(attribute);
        self
    }
}

macro_rules! edit_styles {
    ($($get:ident, $set:ident, $attr:ident,)*) => {
        impl Style{
            $(
            pub fn $get(self) -> Self{
                self.set_attribute(crossterm::style::Attribute::$attr)
            }

            pub fn $set(self) -> Self{
                self.unset_attribute(crossterm::style::Attribute::$attr)
            }
            )*
        }
    };
}

edit_styles!(
    set_bold,
    unset_bold,
    Bold,
    set_crossedout,
    unset_crossedout,
    CrossedOut,
    set_dim,
    unset_dim,
    Dim,
    set_double_underlined,
    unset_double_underlined,
    DoubleUnderlined,
    set_encircled,
    unset_encircled,
    Encircled,
    set_franktur,
    unset_frakkur,
    Fraktur,
    set_framed,
    unset_framed,
    Framed,
    set_hidden,
    unset_hidden,
    Hidden,
    set_italic,
    unset_italic,
    Italic,
    set_no_blink,
    unset_no_blink,
    NoBlink,
    set_no_bold,
    unset_no_bold,
    NoBold,
    set_no_hidden,
    unset_no_hidden,
    NoHidden,
    set_no_italic,
    unset_no_italic,
    NoItalic,
    set_no_reverse,
    unset_no_reverse,
    NoReverse,
    set_no_underline,
    unset_no_underline,
    NoUnderline,
    set_normal_intensity,
    unset_normal_intensity,
    NormalIntensity,
    set_not_crossedout,
    unset_not_crossedout,
    NotCrossedOut,
    set_not_framed_or_encircled,
    unset_not_framed_or_encircled,
    NotFramedOrEncircled,
    set_not_overlined,
    unset_not_overlined,
    NotOverLined,
    set_oberlined,
    unset_overlined,
    OverLined,
    set_rapid_blink,
    unset_rapid_blink,
    RapidBlink,
    set_reset,
    unset_reset,
    Reset,
    set_reverse,
    unset_reverse,
    Reverse,
    set_slowblink,
    unset_slowblink,
    SlowBlink,
    set_undercircled,
    unset_uncercircled,
    Undercurled,
    set_underdashed,
    unset_underdashed,
    Underdashed,
    set_underdotted,
    unset_underdotted,
    Underdotted,
    set_underlined,
    unset_underlined,
    Underlined,
);

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::White,
            bg: Color::Reset,
            attributes: Attributes::default(),
        }
    }
}

impl<'a> From<&'a str> for StyledText<'a> {
    fn from(text: &'a str) -> Self {
        Self {
            text: Cow::Borrowed(text),
            ..Default::default()
        }
    }
}

impl<'a> From<String> for StyledText<'a> {
    fn from(text: String) -> Self {
        Self {
            text: Cow::Owned(text),
            ..Default::default()
        }
    }
}

impl<'a, 'b> From<&'b StyledText<'a>> for StyledText<'b> {
    fn from(value: &'b StyledText<'a>) -> Self {
        Self {
            text: Cow::Borrowed(&value.text),
            style: value.style,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DefaultStyle {
    pub button: Style,
    pub button_hovered: Style,
    pub button_clicked: Style,
    pub button_focused: Style,
    pub button_active: Style,
    pub button_active_hovered: Style,
    pub button_active_clicked: Style,
    pub button_active_focused: Style,

    pub lines: &'static symbols::line::Set,
    pub blocks: &'static symbols::block::Set,
    pub bars: &'static symbols::bar::Set,
    pub pointers: &'static symbols::pointers::Set,
}
impl DefaultStyle {
    pub fn new_unicode() -> Self {
        Self {
            button: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::default(),
            },
            button_hovered: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_clicked: Style {
                fg: Color::White,
                bg: Color::Blue,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_focused: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::default(),
            },
            button_active: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::default(),
            },
            button_active_hovered: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_active_clicked: Style {
                fg: Color::White,
                bg: Color::Blue,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_active_focused: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::default(),
            },
            lines: &symbols::line::NORMAL,
            blocks: &symbols::block::NINE_LEVELS,
            bars: &symbols::bar::NINE_LEVELS,
            pointers: &symbols::pointers::TRIANGLE,
        }
    }

    pub fn new_ascii() -> Self {
        Self {
            button: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::default(),
            },
            button_hovered: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_clicked: Style {
                fg: Color::White,
                bg: Color::Blue,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_focused: Style {
                fg: Color::White,
                bg: Color::Black,
                attributes: Attributes::default(),
            },
            button_active: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::default(),
            },
            button_active_hovered: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_active_clicked: Style {
                fg: Color::White,
                bg: Color::Blue,
                attributes: Attributes::from(&[Attribute::Underlined][..]),
            },
            button_active_focused: Style {
                fg: Color::White,
                bg: Color::Grey,
                attributes: Attributes::default(),
            },
            lines: &symbols::line::ASCII,
            blocks: &symbols::block::THREE_LEVELS,
            bars: &symbols::bar::THREE_LEVELS,
            pointers: &symbols::pointers::ASCII,
        }
    }
}
