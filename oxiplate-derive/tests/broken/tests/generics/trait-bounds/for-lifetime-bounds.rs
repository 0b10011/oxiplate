use std::marker::PhantomData;

use oxiplate_derive::Oxiplate;

const BAZ: usize = 19;

trait Bar<'a> {}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <'a: 'b> Bar<'a>>::from([19, 89]).join("") }}"#)]
struct Data<'b> {
    phantom_data: PhantomData<&'b ()>,
}

fn main() {
    assert_eq!(
        format!(
            "{}",
            Data {
                phantom_data: PhantomData
            }
        ),
        "1989"
    );
}
