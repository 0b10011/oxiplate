use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "./multiple-blocks-inner.html.oxip"]
struct Data;

#[test]
fn multiple_blocks() {
    let data = Data;

    assert_eq!(
        data.render().unwrap(),
        "<!DOCTYPE html>\n<header>header</header>\n<main>main</main>\n<footer>footer</footer>"
    );
}
