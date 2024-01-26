pub mod block {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▉";
    pub const THREE_QUARTERS: &str = "▊";
    pub const FIVE_EIGHTHS: &str = "▋";
    pub const HALF: &str = "▌";
    pub const THREE_EIGHTHS: &str = "▍";
    pub const ONE_QUARTER: &str = "▎";
    pub const ONE_EIGHTH: &str = "▏";

    #[derive(Debug, Clone)]
    pub struct Set {
        pub full: &'static str,
        pub seven_eighths: &'static str,
        pub three_quarters: &'static str,
        pub five_eighths: &'static str,
        pub half: &'static str,
        pub three_eighths: &'static str,
        pub one_quarter: &'static str,
        pub one_eighth: &'static str,
        pub empty: &'static str,
    }

    pub const THREE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: " ",
        empty: " ",
    };

    pub const NINE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: " ",
    };
}

pub mod bar {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▇";
    pub const THREE_QUARTERS: &str = "▆";
    pub const FIVE_EIGHTHS: &str = "▅";
    pub const HALF: &str = "▄";
    pub const THREE_EIGHTHS: &str = "▃";
    pub const ONE_QUARTER: &str = "▂";
    pub const ONE_EIGHTH: &str = "▁";

    #[derive(Debug, Clone)]
    pub struct Set {
        pub full: &'static str,
        pub seven_eighths: &'static str,
        pub three_quarters: &'static str,
        pub five_eighths: &'static str,
        pub half: &'static str,
        pub three_eighths: &'static str,
        pub one_quarter: &'static str,
        pub one_eighth: &'static str,
        pub empty: &'static str,
    }

    pub const THREE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: " ",
        empty: " ",
    };

    pub const NINE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: " ",
    };
}

pub mod line {
    pub const VERTICAL: &str = "│";
    pub const DOUBLE_VERTICAL: &str = "║";
    pub const THICK_VERTICAL: &str = "┃";
    pub const ASCII_VERTICAL: &str = "|";

    pub const HORIZONTAL: &str = "─";
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const THICK_HORIZONTAL: &str = "━";
    pub const ASCII_HORIZONTAL: &str = "-";

    pub const TOP_RIGHT: &str = "┐";
    pub const ROUNDED_TOP_RIGHT: &str = "╮";
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    pub const THICK_TOP_RIGHT: &str = "┓";
    pub const ASCII_TOP_RIGHT: &str = "+";

    pub const TOP_LEFT: &str = "┌";
    pub const ROUNDED_TOP_LEFT: &str = "╭";
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    pub const THICK_TOP_LEFT: &str = "┏";
    pub const ASCII_TOP_LEFT: &str = "+";

    pub const BOTTOM_RIGHT: &str = "┘";
    pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
    pub const THICK_BOTTOM_RIGHT: &str = "┛";
    pub const ASCII_BOTTOM_RIGHT: &str = "+";

    pub const BOTTOM_LEFT: &str = "└";
    pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    pub const THICK_BOTTOM_LEFT: &str = "┗";
    pub const ASCII_BOTTOM_LEFT: &str = "+";

    pub const VERTICAL_LEFT: &str = "┤";
    pub const DOUBLE_VERTICAL_LEFT: &str = "╣";
    pub const THICK_VERTICAL_LEFT: &str = "┫";
    pub const ASCII_VERTICAL_LEFT: &str = "+";

    pub const VERTICAL_RIGHT: &str = "├";
    pub const DOUBLE_VERTICAL_RIGHT: &str = "╠";
    pub const THICK_VERTICAL_RIGHT: &str = "┣";
    pub const ASCII_VERTICAL_RIGHT: &str = "+";

    pub const HORIZONTAL_DOWN: &str = "┬";
    pub const DOUBLE_HORIZONTAL_DOWN: &str = "╦";
    pub const THICK_HORIZONTAL_DOWN: &str = "┳";
    pub const ASCII_HORIZONTAL_DOWN: &str = "+";

    pub const HORIZONTAL_UP: &str = "┴";
    pub const DOUBLE_HORIZONTAL_UP: &str = "╩";
    pub const THICK_HORIZONTAL_UP: &str = "┻";
    pub const ASCII_HORIZONTAL_UP: &str = "+";

    pub const CROSS: &str = "┼";
    pub const DOUBLE_CROSS: &str = "╬";
    pub const THICK_CROSS: &str = "╋";
    pub const ASCII_CROSS: &str = "+";

    #[derive(Debug, Clone)]
    pub struct Set {
        pub vertical: &'static str,
        pub horizontal: &'static str,
        pub top_right: &'static str,
        pub top_left: &'static str,
        pub bottom_right: &'static str,
        pub bottom_left: &'static str,
        pub vertical_left: &'static str,
        pub vertical_right: &'static str,
        pub horizontal_down: &'static str,
        pub horizontal_up: &'static str,
        pub cross: &'static str,
    }

    pub const NORMAL: Set = Set {
        vertical: VERTICAL,
        horizontal: HORIZONTAL,
        top_right: TOP_RIGHT,
        top_left: TOP_LEFT,
        bottom_right: BOTTOM_RIGHT,
        bottom_left: BOTTOM_LEFT,
        vertical_left: VERTICAL_LEFT,
        vertical_right: VERTICAL_RIGHT,
        horizontal_down: HORIZONTAL_DOWN,
        horizontal_up: HORIZONTAL_UP,
        cross: CROSS,
    };

    pub const ASCII: Set = Set {
        vertical: ASCII_VERTICAL,
        horizontal: ASCII_HORIZONTAL,
        top_right: ASCII_TOP_RIGHT,
        top_left: ASCII_TOP_LEFT,
        bottom_right: ASCII_BOTTOM_RIGHT,
        bottom_left: ASCII_BOTTOM_LEFT,
        vertical_left: ASCII_VERTICAL_LEFT,
        vertical_right: ASCII_VERTICAL_RIGHT,
        horizontal_down: ASCII_HORIZONTAL_DOWN,
        horizontal_up: ASCII_HORIZONTAL_UP,
        cross: ASCII_CROSS,
    };

    pub const ROUNDED: Set = Set {
        top_right: ROUNDED_TOP_RIGHT,
        top_left: ROUNDED_TOP_LEFT,
        bottom_right: ROUNDED_BOTTOM_RIGHT,
        bottom_left: ROUNDED_BOTTOM_LEFT,
        ..NORMAL
    };

    pub const DOUBLE: Set = Set {
        vertical: DOUBLE_VERTICAL,
        horizontal: DOUBLE_HORIZONTAL,
        top_right: DOUBLE_TOP_RIGHT,
        top_left: DOUBLE_TOP_LEFT,
        bottom_right: DOUBLE_BOTTOM_RIGHT,
        bottom_left: DOUBLE_BOTTOM_LEFT,
        vertical_left: DOUBLE_VERTICAL_LEFT,
        vertical_right: DOUBLE_VERTICAL_RIGHT,
        horizontal_down: DOUBLE_HORIZONTAL_DOWN,
        horizontal_up: DOUBLE_HORIZONTAL_UP,
        cross: DOUBLE_CROSS,
    };

    pub const THICK: Set = Set {
        vertical: THICK_VERTICAL,
        horizontal: THICK_HORIZONTAL,
        top_right: THICK_TOP_RIGHT,
        top_left: THICK_TOP_LEFT,
        bottom_right: THICK_BOTTOM_RIGHT,
        bottom_left: THICK_BOTTOM_LEFT,
        vertical_left: THICK_VERTICAL_LEFT,
        vertical_right: THICK_VERTICAL_RIGHT,
        horizontal_down: THICK_HORIZONTAL_DOWN,
        horizontal_up: THICK_HORIZONTAL_UP,
        cross: THICK_CROSS,
    };
}

pub const DOT: &str = "•";

pub mod braille {
    pub const BLANK: u16 = 0x2800;
    pub const DOTS: [[u16; 2]; 4] = [
        [0x0001, 0x0008],
        [0x0002, 0x0010],
        [0x0004, 0x0020],
        [0x0040, 0x0080],
    ];
}

/// Marker to use when plotting data points
#[derive(Debug, Clone, Copy)]
pub enum Marker {
    /// One point per cell in shape of dot
    Dot,
    /// One point per cell in shape of a block
    Block,
    /// Up to 8 points per cell
    Braille,
}

pub mod pointers {
    #[derive(Debug, Clone)]
    pub struct Set {
        pub up: &'static str,
        pub up_right: &'static str,
        pub right: &'static str,
        pub right_down: &'static str,
        pub down: &'static str,
        pub down_left: &'static str,
        pub left: &'static str,
        pub left_up: &'static str,
    }

    pub const ARROW_UP: &str = "↑";
    pub const ARROW_UP_RIGHT: &str = "↗";
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_RIGHT_DOWN: &str = "↘";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_DOWN_LEFT: &str = "↙";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_LEFT_UP: &str = "↖";

    pub const TRIANGLE_UP: &str = "▲";
    pub const TRIANGLE_UP_RIGHT: &str = " ";
    pub const TRIANGLE_RIGHT: &str = "▶";
    pub const TRIANGLE_RIGHT_DOWN: &str = " ";
    pub const TRIANGLE_DOWN: &str = "▼";
    pub const TRIANGLE_DOWN_LEFT: &str = " ";
    pub const TRIANGLE_LEFT: &str = "◀";
    pub const TRIANGLE_LEFT_UP: &str = " ";

    pub const ASCII_UP: &str = "^";
    pub const ASCII_UP_RIGHT: &str = " ";
    pub const ASCII_RIGHT: &str = ">";
    pub const ASCII_RIGHT_DOWN: &str = " ";
    pub const ASCII_DOWN: &str = " ";
    pub const ASCII_DOWN_LEFT: &str = " ";
    pub const ASCII_LEFT: &str = "<";
    pub const ASCII_LEFT_UP: &str = " ";

    pub static ASCII: Set = Set {
        up: ASCII_UP,
        up_right: ASCII_UP_RIGHT,
        right: ASCII_RIGHT,
        right_down: ASCII_RIGHT_DOWN,
        down: ASCII_DOWN,
        down_left: ASCII_DOWN_LEFT,
        left: ASCII_LEFT,
        left_up: ASCII_LEFT_UP,
    };

    pub static TRIANGLE: Set = Set {
        up: TRIANGLE_UP,
        up_right: TRIANGLE_UP_RIGHT,
        right: TRIANGLE_RIGHT,
        right_down: TRIANGLE_RIGHT_DOWN,
        down: TRIANGLE_DOWN,
        down_left: TRIANGLE_DOWN_LEFT,
        left: TRIANGLE_LEFT,
        left_up: TRIANGLE_LEFT_UP,
    };

    pub static ARROW: Set = Set {
        up: ARROW_UP,
        up_right: ARROW_UP_RIGHT,
        right: ARROW_RIGHT,
        right_down: ARROW_RIGHT_DOWN,
        down: ARROW_DOWN,
        down_left: ARROW_DOWN_LEFT,
        left: ARROW_LEFT,
        left_up: ARROW_LEFT_UP,
    };
}
