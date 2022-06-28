#[cfg(test)]
use couleur::*;

#[test]
fn bold_red() {
    let test = "testing".coloriser(Colors::Red).add_style(Styles::Bold);
    assert_eq!(test.to_string(), "\x1b[1;31mtesting\x1b[m")
}

#[test]
fn strik_under_blue() {
    let test = "testing"
        .coloriser(Colors::Blue)
        .add_style(Styles::Strikethrough)
        .add_style(Styles::Underline);
    assert_eq!(test.to_string(), "\x1b[4;9;34mtesting\x1b[m")
}

#[test]
fn strik_under_italic_ture() {
    let test = "testing"
        .coloriser(Colors::TrueColor {
            r: 200,
            g: 55,
            b: 55,
        })
        .add_style(Styles::Strikethrough)
        .add_style(Styles::Underline)
        .add_style(Styles::Italic);
    assert_eq!(test.to_string(), "\x1b[3;4;9;38;2;200;55;55mtesting\x1b[m")
}

#[test]
fn under_italic_black() {
    let test = "testing"
        .coloriser(Colors::Black)
        .add_style(Styles::Underline)
        .add_style(Styles::Italic);
    assert_eq!(test.to_string(), "\x1b[3;4;30mtesting\x1b[m")
}

#[test]
fn italic_strik_green() {
    let test = "testing"
        .coloriser(Colors::Green)
        .add_style(Styles::Italic)
        .add_style(Styles::Strikethrough);
    assert_eq!(test.to_string(), "\x1b[3;9;32mtesting\x1b[m")
}
