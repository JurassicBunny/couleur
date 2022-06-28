use crate::colors::Colors;
use crate::style::{Styles, TextStyles};

use std::fmt::Display;

// internal trait used to mark struts with which
// the Couleur trait will be implemented.
trait CouleurMarker {
    fn too_string(self) -> String;
}

impl CouleurMarker for String {
    fn too_string(self) -> String {
        self
    }
}

impl<'a> CouleurMarker for &'a str {
    fn too_string(self) -> String {
        self.to_string()
    }
}

/// French for color. Trait that extends the String and &str structs
/// allowing for the coloring and stylization of strings to be displayed
/// to the console.
pub trait Couleur {
    fn coloriser(self, color: Colors) -> ColorTxt;
    fn styliser(self, style: Styles) -> ColorTxt;
}

/// Wrapper around the String struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Text {
    txt: String,
}

impl Text {
    /// Make a new Text from a string
    fn new(txt: String) -> Text {
        Text { txt }
    }
}

/// Colored and or styled text that may be printed to the console.
///
/// Contains the text to be printed, the color selected, and Styles
/// added. To initialize a ColorTxt, one only needs to call either
/// `coloriser` or `styliser` on a `String` or `&str`.
///
/// # Examples
/// ```rust
/// // import trait and structs
/// use couleur::{Couleur, Colors, Styles};
///
/// fn main() {
///     // define ColorTxt with color and add style
///     let color_text = "Hello, World!".coloriser(Colors::Red)
///                                     .add_style(Styles::Bold);
///
///     // define ColorTxt with style and add color
///     let style_text = "Hello, World!".styliser(Styles::Bold)
///                                     .edit_color(Colors::Red);
///
///     // color_text and style_text will be the same
///     assert_eq!(color_text, style_text);
/// }
/// ```
/// Color or style may be omitted. In such a case, the omitted field
/// will resort to the default i.e. uncolored text or no style.
///
/// Multiple styles can be added to a ColorTxt:
///
/// ```rust
/// # use couleur::{Couleur, Colors, Styles};
/// let text = "Hello, World!".coloriser(Colors::Blue)
///                           .add_style(Styles::Bold)
///                           .add_style(Styles::Underline);
/// ```
/// In the above example, the styles `Bold` and `Underline` are added
/// to a `UniqueVec<Styles>` maintained by the ColorTxt. This insures
/// that each style is only accounted for one time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorTxt {
    text: Text,
    color: Colors,
    style: TextStyles,
}

impl Default for ColorTxt {
    fn default() -> Self {
        ColorTxt {
            text: Text::new("".to_string()),
            color: Colors::Clear,
            style: TextStyles::new(),
        }
    }
}

impl Display for ColorTxt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gen_string())
    }
}

impl<T> Couleur for T
where
    T: CouleurMarker,
{
    /// Given a color return a ColorTxt
    fn coloriser(self, color: Colors) -> ColorTxt {
        ColorTxt::new().txt(self.too_string()).edit_color(color)
    }

    /// Given a style return a ColorTxt
    fn styliser(self, style: Styles) -> ColorTxt {
        ColorTxt::new().txt(self.too_string()).add_style(style)
    }
}

impl ColorTxt {
    /// Generate a new ColorTxt
    pub fn new() -> ColorTxt {
        ColorTxt::default()
    }

    /// Keep the Text and clear the styles and remove color
    pub fn clear(self) -> ColorTxt {
        ColorTxt {
            color: Colors::Clear,
            style: self.style.clear(),
            ..self
        }
    }

    /// Change the color of ColorTxt
    pub fn edit_color(self, color: Colors) -> ColorTxt {
        ColorTxt { color, ..self }
    }

    /// Add a unique style to TextStyles
    pub fn add_style(mut self, style: Styles) -> ColorTxt {
        self.style.push(style);
        ColorTxt { ..self }
    }

    /// Return the color of ColorTxt
    pub fn color(&self) -> Colors {
        self.color.to_owned()
    }

    // private function to generate the Text of a ColorTxt
    fn txt(self, txt: String) -> ColorTxt {
        let text = Text::new(txt);
        ColorTxt { text, ..self }
    }

    // when called, format the of a ColorTxt
    fn gen_string(&self) -> String {
        format!(
            "\x1b[{};{}m{}\x1b[m",
            self.style.code(),
            self.color.code(),
            self.text.txt
        )
    }
}
