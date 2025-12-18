use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "extends-with-empty-content-crlf.html.oxip"]
struct AbsoluteData {}

fn main() {
    // Intentionally missing the "title" field used by the parent template
    let data = AbsoluteData {};

    print!("{}", data);
}
