use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    pub fn length(value: &str) -> u64 {
        value.len() as u64
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "foo" | >length }}"#)]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
