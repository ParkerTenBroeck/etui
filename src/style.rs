use std::borrow::Cow;

pub type Attributes = crossterm::style::Attributes;
pub type Color = crossterm::style::Color;
pub type Attribute = crossterm::style::Attribute;

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
            fg: Color::Reset,
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
