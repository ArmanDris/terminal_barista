use ratatui::style::Color;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LiquidColors {
    Red,
    Green,
    Blue,
}

impl fmt::Display for LiquidColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LiquidColors::Red => "Red",
            LiquidColors::Green => "Green",
            LiquidColors::Blue => "Blue",
        };
        write!(f, "{}", s)
    }
}

impl LiquidColors {
    pub fn to_color(&self) -> Color {
        match self {
            LiquidColors::Red => Color::LightRed,
            LiquidColors::Green => Color::LightGreen,
            LiquidColors::Blue => Color::LightBlue,
        }
    }
}
