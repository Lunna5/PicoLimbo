use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl NamedColor {
    pub const ALL: [NamedColor; 16] = [
        NamedColor::Black,
        NamedColor::DarkBlue,
        NamedColor::DarkGreen,
        NamedColor::DarkAqua,
        NamedColor::DarkRed,
        NamedColor::DarkPurple,
        NamedColor::Gold,
        NamedColor::Gray,
        NamedColor::DarkGray,
        NamedColor::Blue,
        NamedColor::Green,
        NamedColor::Aqua,
        NamedColor::Red,
        NamedColor::LightPurple,
        NamedColor::Yellow,
        NamedColor::White,
    ];

    pub const fn name(&self) -> &'static str {
        match self {
            NamedColor::Black => "black",
            NamedColor::DarkBlue => "dark_blue",
            NamedColor::DarkGreen => "dark_green",
            NamedColor::DarkAqua => "dark_aqua",
            NamedColor::DarkRed => "dark_red",
            NamedColor::DarkPurple => "dark_purple",
            NamedColor::Gold => "gold",
            NamedColor::Gray => "gray",
            NamedColor::DarkGray => "dark_gray",
            NamedColor::Blue => "blue",
            NamedColor::Green => "green",
            NamedColor::Aqua => "aqua",
            NamedColor::Red => "red",
            NamedColor::LightPurple => "light_purple",
            NamedColor::Yellow => "yellow",
            NamedColor::White => "white",
        }
    }

    pub const fn rgb(&self) -> (u8, u8, u8) {
        match self {
            NamedColor::Black => (0x00, 0x00, 0x00),
            NamedColor::DarkBlue => (0x00, 0x00, 0xAA),
            NamedColor::DarkGreen => (0x00, 0xAA, 0x00),
            NamedColor::DarkAqua => (0x00, 0xAA, 0xAA),
            NamedColor::DarkRed => (0xAA, 0x00, 0x00),
            NamedColor::DarkPurple => (0xAA, 0x00, 0xAA),
            NamedColor::Gold => (0xFF, 0xAA, 0x00),
            NamedColor::Gray => (0xAA, 0xAA, 0xAA),
            NamedColor::DarkGray => (0x55, 0x55, 0x55),
            NamedColor::Blue => (0x55, 0x55, 0xFF),
            NamedColor::Green => (0x55, 0xFF, 0x55),
            NamedColor::Aqua => (0x55, 0xFF, 0xFF),
            NamedColor::Red => (0xFF, 0x55, 0x55),
            NamedColor::LightPurple => (0xFF, 0x55, 0xFF),
            NamedColor::Yellow => (0xFF, 0xFF, 0x55),
            NamedColor::White => (0xFF, 0xFF, 0xFF),
        }
    }

    pub const fn legacy_char(&self) -> char {
        match self {
            NamedColor::Black => '0',
            NamedColor::DarkBlue => '1',
            NamedColor::DarkGreen => '2',
            NamedColor::DarkAqua => '3',
            NamedColor::DarkRed => '4',
            NamedColor::DarkPurple => '5',
            NamedColor::Gold => '6',
            NamedColor::Gray => '7',
            NamedColor::DarkGray => '8',
            NamedColor::Blue => '9',
            NamedColor::Green => 'a',
            NamedColor::Aqua => 'b',
            NamedColor::Red => 'c',
            NamedColor::LightPurple => 'd',
            NamedColor::Yellow => 'e',
            NamedColor::White => 'f',
        }
    }

    pub fn closest_to_rgb(r: u8, g: u8, b: u8) -> Self {
        let mut closest = NamedColor::White;
        let mut min_distance = u32::MAX;

        for color in Self::ALL {
            let (cr, cg, cb) = color.rgb();
            let dr = (r as i32) - (cr as i32);
            let dg = (g as i32) - (cg as i32);
            let db = (b as i32) - (cb as i32);
            let dist = (dr * dr + dg * dg + db * db) as u32;

            if dist < min_distance {
                min_distance = dist;
                closest = color;
                if dist == 0 {
                    break;
                }
            }
        }

        closest
    }
}

impl FromStr for NamedColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "black" => Ok(NamedColor::Black),
            "dark_blue" => Ok(NamedColor::DarkBlue),
            "dark_green" => Ok(NamedColor::DarkGreen),
            "dark_aqua" => Ok(NamedColor::DarkAqua),
            "dark_red" => Ok(NamedColor::DarkRed),
            "dark_purple" => Ok(NamedColor::DarkPurple),
            "gold" => Ok(NamedColor::Gold),
            "gray" | "grey" => Ok(NamedColor::Gray),
            "dark_gray" | "dark_grey" => Ok(NamedColor::DarkGray),
            "blue" => Ok(NamedColor::Blue),
            "green" => Ok(NamedColor::Green),
            "aqua" => Ok(NamedColor::Aqua),
            "red" => Ok(NamedColor::Red),
            "light_purple" => Ok(NamedColor::LightPurple),
            "yellow" => Ok(NamedColor::Yellow),
            "white" => Ok(NamedColor::White),
            _ => Err(()),
        }
    }
}

impl fmt::Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Named(NamedColor),
    Hex(u8, u8, u8),
}

impl Color {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if let Some(hex_digits) = s
            .strip_prefix('#')
            .filter(|h| h.len() == 6 && h.chars().all(|c| c.is_ascii_hexdigit()))
        {
            let r = u8::from_str_radix(&hex_digits[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex_digits[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex_digits[4..6], 16).ok()?;
            return Some(Color::Hex(r, g, b));
        }

        if let Ok(named) = NamedColor::from_str(s) {
            return Some(Color::Named(named));
        }

        None
    }

    pub fn to_fallback(&self) -> NamedColor {
        match self {
            Color::Named(named) => *named,
            Color::Hex(r, g, b) => NamedColor::closest_to_rgb(*r, *g, *b),
        }
    }

    pub fn legacy_char(&self) -> char {
        self.to_fallback().legacy_char()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Named(named) => write!(f, "{}", named.name()),
            Color::Hex(r, g, b) => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::parse(&s).ok_or_else(|| serde::de::Error::custom(format!("Invalid color: {s}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_named_colors() {
        assert_eq!(Color::parse("red"), Some(Color::Named(NamedColor::Red)));
        assert_eq!(
            Color::parse("dark_blue"),
            Some(Color::Named(NamedColor::DarkBlue))
        );
        assert_eq!(Color::parse("grey"), Some(Color::Named(NamedColor::Gray)));
        assert_eq!(
            Color::parse("DARK_RED"),
            Some(Color::Named(NamedColor::DarkRed))
        );
    }

    #[test]
    fn test_parse_hex_colors() {
        assert_eq!(Color::parse("#ff0000"), Some(Color::Hex(0xFF, 0x00, 0x00)));
        assert_eq!(Color::parse("#00AAFF"), Some(Color::Hex(0x00, 0xAA, 0xFF)));
        assert_eq!(Color::parse("#123456"), Some(Color::Hex(0x12, 0x34, 0x56)));
        assert_eq!(Color::parse("#fff"), None);
        assert_eq!(Color::parse("#gggggg"), None);
        assert_eq!(Color::parse("random"), None);
    }

    #[test]
    fn test_fallback_closest_color() {
        assert_eq!(Color::Hex(0xFF, 0x55, 0x55).to_fallback(), NamedColor::Red);
        assert_eq!(
            Color::Hex(0x00, 0x00, 0x00).to_fallback(),
            NamedColor::Black
        );
        assert_eq!(
            Color::Hex(0xFF, 0xFF, 0xFF).to_fallback(),
            NamedColor::White
        );
        assert_eq!(Color::Hex(0xFE, 0x50, 0x50).to_fallback(), NamedColor::Red);
        assert_eq!(
            Color::Hex(0x00, 0xAA, 0x00).to_fallback(),
            NamedColor::DarkGreen
        );
        assert_eq!(
            Color::Hex(0x55, 0xFE, 0x55).to_fallback(),
            NamedColor::Green
        );
    }

    #[test]
    fn test_color_serialization() {
        assert_eq!(
            serde_json::to_string(&Color::Named(NamedColor::Red)).unwrap(),
            "\"red\""
        );
        assert_eq!(
            serde_json::to_string(&Color::Hex(0x12, 0xAB, 0xCD)).unwrap(),
            "\"#12abcd\""
        );
    }
}
