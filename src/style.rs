use crate::unique_vec::UniqueVec;

/// Enumeration describing the available styles. Style selection
/// is dependent on variant.
///
/// This Enumeration is used as a parameter for the `styliser(self, style: Styles)`
/// and the `add_style(self, style: Styles)` functions.
///
/// # Example
/// ```
/// # use couleur::{Styles, Couleur};
/// let text = "let's color!".styliser(Styles::Bold);
///
/// ```
/// Add a style:
/// ```
/// # use couleur::{Styles, Couleur};
/// let text = "let's color!".styliser(Styles::Bold)
///                          .add_style(Styles::Underline);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Styles {
    Clear,
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TextStyles {
    inner: UniqueVec<Styles>,
}

impl TextStyles {
    pub(crate) fn new() -> TextStyles {
        TextStyles {
            inner: UniqueVec::<Styles>::new(),
        }
    }

    pub(crate) fn clear(self) -> TextStyles {
        Self::new()
    }

    pub(crate) fn push(&mut self, style: Styles) {
        self.inner.push(style)
    }

    pub(crate) fn code(&self) -> String {
        self.inner
            .clone()
            .into_iter()
            .map(|x| x.code())
            .collect::<Vec<&str>>()
            .join(";")
            .to_string()
    }
}

impl Styles {
    pub(crate) fn code<'a>(&self) -> &'a str {
        match self {
            Styles::Bold => "1",
            Styles::Clear => "0",
            Styles::Italic => "3",
            Styles::Underline => "4",
            Styles::Strikethrough => "9",
        }
    }
}

impl From<&str> for Styles {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Styles::Clear)
    }
}

impl core::str::FromStr for Styles {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_ref() {
            "bold" => Ok(Styles::Bold),
            "italic" => Ok(Styles::Italic),
            "underline" => Ok(Styles::Underline),
            "strikethrough" => Ok(Styles::Strikethrough),
            _ => Err(()),
        }
    }
}
