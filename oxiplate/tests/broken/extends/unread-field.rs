use oxiplate::{Oxiplate, Render};

#[deny(dead_code)]
#[derive(Oxiplate)]
#[oxiplate = "./multiple-blocks-inner.html.oxip"]
struct Data {
    unread_field: String,
}

fn main() {
    let data = Data {
        unread_field: String::from(
            "this isn't read in the template, but will be included in the parent template's \
             extends data and should still cause an error",
        ),
    };

    assert_eq!(
        data.render().unwrap(),
        "<!DOCTYPE html>\n<header>header</header>\n<main>main</main>\\
         n<footer>footer</footer>"
    );
}
