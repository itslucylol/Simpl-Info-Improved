use std::collections::HashMap;
use std::fmt;

/// Supported colors mimicking Fastfetch's color palette capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    /// Returns the raw ANSI escape sequence string for the terminal
    pub fn to_ansi(self) -> &'static str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Black => "\x1b[0;30m",
            Color::Red => "\x1b[0;31m",
            Color::Green => "\x1b[0;32m",
            Color::Yellow => "\x1b[0;33m",
            Color::Blue => "\x1b[0;34m",
            Color::Magenta => "\x1b[0;35m",
            Color::Cyan => "\x1b[0;36m",
            Color::White => "\x1b[0;37m",
            Color::BrightBlack => "\x1b[1;30m",
            Color::BrightRed => "\x1b[1;31m",
            Color::BrightGreen => "\x1b[1;32m",
            Color::BrightYellow => "\x1b[1;33m",
            Color::BrightBlue => "\x1b[1;34m",
            Color::BrightMagenta => "\x1b[1;35m",
            Color::BrightCyan => "\x1b[1;36m",
            Color::BrightWhite => "\x1b[1;37m",
        }
    }
}

// Allows printing the color directly: println!("{}", Color::Cyan);
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}

/// Helper block to parse Fastfetch text tokens like "{#cyan}" or "{#34}"
pub struct ColorManager {
    token_map: HashMap<String, Color>,
}

impl ColorManager {
    pub fn new() -> Self {
        let mut token_map = HashMap::new();
        
        // Map common Fastfetch-style text names to our enum entries
        let colors = vec![
            ("black", Color::Black), ("red", Color::Red), ("green", Color::Green),
            ("yellow", Color::Yellow), ("blue", Color::Blue), ("magenta", Color::Magenta),
            ("cyan", Color::Cyan), ("white", Color::White),
            ("bright_black", Color::BrightBlack), ("bright_red", Color::BrightRed),
            ("bright_green", Color::BrightGreen), ("bright_yellow", Color::BrightYellow),
            ("bright_blue", Color::BrightBlue), ("bright_magenta", Color::BrightMagenta),
            ("bright_cyan", Color::BrightCyan), ("bright_white", Color::BrightWhite),
        ];

        for (name, color) in colors {
            token_map.insert(name.to_string(), color);
        }

        ColorManager { token_map }
    }

    /// Convert a string name/token to an actual executable color
    pub fn get_color(&self, token: &str) -> Color {
        self.token_map.get(token).cloned().unwrap_or(Color::Reset)
    }
}