use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "./multiple-blocks-inner.html.oxip"]
struct Data;

#[test]
fn multiple_blocks() {
    let data = Data;

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<header>header</header>\n<main>main</main>\n<footer>footer</footer>"
    );
}
