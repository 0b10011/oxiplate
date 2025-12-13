use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ text: message }}")]
struct Data<'a> {
    message: &'a str,
}

fn main() {
    let data = Data {
        message: "Hello world!"
    };

    assert_eq!(
        data.render().unwrap(),
        "Hello world!"
    );
}
