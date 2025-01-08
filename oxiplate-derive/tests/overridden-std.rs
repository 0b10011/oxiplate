use oxiplate_derive::Oxiplate;

mod std {}

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ foo }}"]
struct Data {
    foo: &'static str,
}

#[test]
fn overridden_std() {
    let data = Data {
        foo: "Hello world!",
    };

    print!("{data}");
}
