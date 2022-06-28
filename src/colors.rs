use std::borrow::Cow;

/// Enumeration describing the available colors. Color selection
/// is dependent on variant. If desired color is not listed,
/// a custom variant may be created using `Colors::TrueColor {r, g, b}`.
///
/// This Enumeration is used as a parameter for the `coloriser(self, color: Colors)`
/// and the `edit_color(self, color: Colors)` functions.
///
/// # Example
/// ```
/// # use couleur::{Colors, Couleur};
/// let text = "let's color!".coloriser(Colors::Green);
/// assert_eq!(text.color(), Colors::Green)
/// ```
///
/// Editing a color:
/// ```
/// # use couleur::{Colors, Couleur};
/// // initialize a ColorTxt
/// let text = "let's color!".coloriser(Colors::Green);
/// assert_eq!(text.color(), Colors::Green);
///
/// // edit the ColorTxt's color
/// let new_text = text.edit_color(Colors::White);
/// assert_eq!(new_text.color(), Colors::White)
/// ```
///
/// Using TureColor:
/// ```
/// # use couleur::{Colors, Couleur};
/// let text = "true red color".coloriser(Colors::TrueColor {
///         r: 255,
///         b: 0,
///         g: 0});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Colors {
    Clear,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    TrueColor { r: u8, g: u8, b: u8 },
}

impl Colors {
    pub(crate) fn code<'a>(&self) -> Cow<'a, str> {
        match self {
            Colors::Clear => "0".into(),
            Colors::Black => "30".into(),
            Colors::Red => "31".into(),
            Colors::Green => "32".into(),
            Colors::Yellow => "33".into(),
            Colors::Blue => "34".into(),
            Colors::Magenta => "35".into(),
            Colors::Cyan => "36".into(),
            Colors::White => "37".into(),
            Colors::TrueColor { r, g, b } => format!("38;2;{};{};{}", r, g, b).into(),
        }
    }
}

impl From<&str> for Colors {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Colors::Clear)
    }
}

impl core::str::FromStr for Colors {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "black" => Ok(Colors::Black),
            "blue" => Ok(Colors::Blue),
            "clear" => Ok(Colors::Clear),
            "cyan" => Ok(Colors::Cyan),
            "green" => Ok(Colors::Green),
            "magenta" => Ok(Colors::Magenta),
            "red" => Ok(Colors::Red),
            "white" => Ok(Colors::White),
            "yellow" => Ok(Colors::Yellow),
            _ => Err(()),
        }
    }
}
