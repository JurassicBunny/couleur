#[cfg(test)]
use couleur::*;

#[test]
fn red_txt() {
    let test = "testing".coloriser(Colors::Red);
    assert_eq!(test.to_string(), "\x1b[;31mtesting\x1b[m")
}

#[test]
fn black_txt() {
    let test = "testing".coloriser(Colors::Black);
    assert_eq!(test.to_string(), "\x1b[;30mtesting\x1b[m")
}

#[test]
fn white_txt() {
    let test = "testing".coloriser(Colors::White);
    assert_eq!(test.to_string(), "\x1b[;37mtesting\x1b[m")
}

#[test]
fn magenta_txt() {
    let test = "testing".coloriser(Colors::Magenta);
    assert_eq!(test.to_string(), "\x1b[;35mtesting\x1b[m")
}

#[test]
fn yellow_txt() {
    let test = "testing".coloriser(Colors::Yellow);
    assert_eq!(test.to_string(), "\x1b[;33mtesting\x1b[m")
}

#[test]
fn truecolor_txt() {
    let test = "testing".coloriser(Colors::TrueColor {
        r: 100,
        g: 200,
        b: 100,
    });
    assert_eq!(test.to_string(), "\x1b[;38;2;100;200;100mtesting\x1b[m")
}
