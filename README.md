# couleur
Colored and or styled text that may be printed to the console.

Contains the text to be printed, the color selected, and Styles
added. To initialize a ColorTxt, one only needs to call either
`coloriser` or `styliser` on a `String` or `&str`.

# Examples
```rust
// import trait and structs
use couleur::{Couleur, Colors, Styles};

fn main() {
    // define ColorTxt with color and add style
    let color_text = "Hello, World!".coloriser(Colors::Red)
                                    .add_style(Styles::Bold);

    // define ColorTxt with style and add color
    let style_text = "Hello, World!".styliser(Styles::Bold)
                                    .edit_color(Colors::Red);

    // color_text and style_text will be the same
    assert_eq!(color_text, style_text);
}
```
Color or style may be omitted. In such a case, the omitted field
will resort to the default i.e. uncolored text or no style.

Multiple styles can be added to a ColorTxt:

```rust
# use couleur::{Couleur, Colors, Styles};
let text = "Hello, World!".coloriser(Colors::Blue)
                          .add_style(Styles::Bold)
                          .add_style(Styles::Underline);
```
In the above example, the styles `Bold` and `Underline` are added
to a `UniqueVec<Styles>` maintained by the ColorTxt. This insures
that each style is only accounted for one time.
