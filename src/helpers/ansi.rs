// ----------
// ANSI CODES
// ----------

/// ANSI escape codes for text formatting
enum Color {
    Red,
    // Green,
    // Yellow,
    // Blue,
    // Magenta,
    // Cyan,
    // White,
}

impl Color {
    fn code(&self) -> u8 {
        match self {
            Color::Red => 31,
            // Color::Green => 32,
            // Color::Yellow => 33,
            // Color::Blue => 34,
            // Color::Magenta => 35,
            // Color::Cyan => 36,
            // Color::White => 37,
        }
    }

    fn wrap(&self, text: &str) -> String {
        format!("\u{001b}[{}m{}\u{001b}[0m", self.code(), text)
    }
}

// ---------------
// COLORABLE TRAIT
// ---------------

/// A trait for coloring text with ANSI escape codes
pub trait Colorable {
    fn red(&self) -> String;
    // fn green(&self) -> String;
    // fn yellow(&self) -> String;
    // fn blue(&self) -> String;
    // fn magenta(&self) -> String;
    // fn cyan(&self) -> String;
    // fn white(&self) -> String;
}

impl Colorable for String {
    fn red(&self) -> String {
        Color::Red.wrap(self)
    }
    // fn green(&self) -> String {
    //     Color::Green.wrap(self)
    // }
    // fn yellow(&self) -> String {
    //     Color::Yellow.wrap(self)
    // }
    // fn blue(&self) -> String {
    //     Color::Blue.wrap(self)
    // }
    // fn magenta(&self) -> String {
    //     Color::Magenta.wrap(self)
    // }
    // fn cyan(&self) -> String {
    //     Color::Cyan.wrap(self)
    // }
    // fn white(&self) -> String {
    //     Color::White.wrap(self)
    // }
}

impl Colorable for std::io::Error {
    fn red(&self) -> String {
        Color::Red.wrap(&self.to_string())
    }
    // fn green(&self) -> String {
    //     Color::Green.wrap(&self.to_string())
    // }
    // fn yellow(&self) -> String {
    //     Color::Yellow.wrap(&self.to_string())
    // }
    // fn blue(&self) -> String {
    //     Color::Blue.wrap(&self.to_string())
    // }
    // fn magenta(&self) -> String {
    //     Color::Magenta.wrap(&self.to_string())
    // }
    // fn cyan(&self) -> String {
    //     Color::Cyan.wrap(&self.to_string())
    // }
    // fn white(&self) -> String {
    //     Color::White.wrap(&self.to_string())
    // }
}
