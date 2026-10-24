use std::marker::PhantomData;

use oxiplate_derive::Oxiplate;

const BAZ: usize = 19;

trait Bar<'a> {}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <'a: 'b + 'c + 'd> Bar<'a>>::from([19, 89]).join("") }}"#)]
struct Data<'b, 'c, 'd> {
    phantom_data: PhantomData<&'b &'c &'d ()>,
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
