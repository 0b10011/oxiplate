use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "extends-with-empty-content.html.oxip"]
struct AbsoluteData {
    tile: &'static str,
}

fn main() {
    // Intentionally misspelled the "title" field used by the parent template
    let data = AbsoluteData {
        tile: "Hello world",
    };

    let _ = data.render().unwrap();
}
