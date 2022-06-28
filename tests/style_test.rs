#[cfg(test)]
use couleur::*;

#[test]
fn bold_test() {
    let test = "testing".styliser(Styles::Bold);
    assert_eq!(test.to_string(), "\x1b[1;0mtesting\x1b[m")
}

#[test]
fn strike_test() {
    let test = "testing".styliser(Styles::Strikethrough);
    assert_eq!(test.to_string(), "\x1b[9;0mtesting\x1b[m")
}

#[test]
fn italic_test() {
    let test = "testing".styliser(Styles::Italic);
    assert_eq!(test.to_string(), "\x1b[3;0mtesting\x1b[m")
}

#[test]
fn underline_test() {
    let test = "testing".styliser(Styles::Underline);
    assert_eq!(test.to_string(), "\x1b[4;0mtesting\x1b[m")
}

#[test]
fn italic_underline_test() {
    let test = "testing"
        .styliser(Styles::Italic)
        .add_style(Styles::Underline);
    assert_eq!(test.to_string(), "\x1b[3;4;0mtesting\x1b[m")
}

#[test]
fn bold_italic_test() {
    let test = "testing".styliser(Styles::Bold).add_style(Styles::Italic);
    assert_eq!(test.to_string(), "\x1b[1;3;0mtesting\x1b[m")
}

#[test]
fn bold_italic_underline_test() {
    let test = "testing"
        .styliser(Styles::Bold)
        .add_style(Styles::Italic)
        .add_style(Styles::Underline);
    assert_eq!(test.to_string(), "\x1b[1;3;4;0mtesting\x1b[m")
}
