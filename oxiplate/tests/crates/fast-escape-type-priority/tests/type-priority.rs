use oxiplate::{Oxiplate, Render};

macro_rules! test {
    ($fn_name:ident, $ty:ty, $value:expr, $expected:expr) => {
        #[test]
        fn $fn_name() {
            test!("{{ value }}", $ty, $value, $expected, "escaped");
            test!("{{ raw: value }}", $ty, $value, $expected, "raw");

            // Box
            let value = $value;
            test!(
                "{{ value }}",
                Box<$ty>,
                Box::new(value),
                $expected,
                "box escaped"
            );
            let value = $value;
            test!(
                "{{ raw: value }}",
                Box<$ty>,
                Box::new(value),
                $expected,
                "box raw"
            );

            // Rc
            let value = $value;
            test!(
                "{{ value }}",
                ::std::rc::Rc<$ty>,
                ::std::rc::Rc::new(value),
                $expected,
                "rc escaped"
            );
            let value = $value;
            test!(
                "{{ raw: value }}",
                ::std::rc::Rc<$ty>,
                ::std::rc::Rc::new(value),
                $expected,
                "rc raw"
            );

            // Arc
            let value = $value;
            test!(
                "{{ value }}",
                ::std::sync::Arc<$ty>,
                ::std::sync::Arc::new(value),
                $expected,
                "arc escaped"
            );
            let value = $value;
            test!(
                "{{ raw: value }}",
                ::std::sync::Arc<$ty>,
                ::std::sync::Arc::new(value),
                $expected,
                "arc raw"
            );
        }
    };
    ($template:literal, $ty:ty, $value:expr, $expected:expr, $message:literal) => {{
        #[derive(Oxiplate)]
        #[oxiplate_inline(html: $template)]
        struct Value<'a> {
            value: $ty,
            #[allow(dead_code)]
            borrow: &'a ::std::marker::PhantomData<()>,
        }
        let value = Value {
            value: $value,
            borrow: &::std::marker::PhantomData,
        };
        assert_eq!(value.render().unwrap(), $expected, $message);
    }};
}

test!(
    string,
    String,
    "text".to_string(),
    "FastEscape(String(text))"
);
test!(
    string_mut,
    &'a mut String,
    &mut "text".to_string(),
    "FastEscape(String(text))"
);
test!(str, &'a str, "text", "FastEscape(str(text))");
test!(str_2, &'a &'a str, &"text", "FastEscape(str(text))");

test!(i8, i8, 19_i8, "FastEscape(int(19))");
test!(i8_mut, &'a mut i8, &mut 19_i8, "FastEscape(int(19))");
test!(i8_borrow, &'a i8, &19_i8, "FastEscape(int(19))");
test!(i8_borrow_2, &'a &'a i8, &&19_i8, "FastEscape(int(19))");
test!(i8_borrow_mut, &'a mut i8, &mut 19_i8, "FastEscape(int(19))");

test!(i16, i16, 19_i16, "FastEscape(int(19))");
test!(i16_mut, &'a mut i16, &mut 19_i16, "FastEscape(int(19))");
test!(i16_borrow, &'a i16, &19_i16, "FastEscape(int(19))");
test!(i16_borrow_2, &'a &'a i16, &&19_i16, "FastEscape(int(19))");
test!(
    i16_borrow_mut,
    &'a mut i16,
    &mut 19_i16,
    "FastEscape(int(19))"
);

test!(i32, i32, 19_i32, "FastEscape(int(19))");
test!(i32_mut, &'a mut i32, &mut 19_i32, "FastEscape(int(19))");
test!(i32_borrow, &'a i32, &19_i32, "FastEscape(int(19))");
test!(i32_borrow_2, &'a &'a i32, &&19_i32, "FastEscape(int(19))");
test!(
    i32_borrow_mut,
    &'a mut i32,
    &mut 19_i32,
    "FastEscape(int(19))"
);

test!(i64, i64, 19_i64, "FastEscape(int(19))");
test!(i64_mut, &'a mut i64, &mut 19_i64, "FastEscape(int(19))");
test!(i64_borrow, &'a i64, &19_i64, "FastEscape(int(19))");
test!(i64_borrow_2, &'a &'a i64, &&19_i64, "FastEscape(int(19))");
test!(
    i64_borrow_mut,
    &'a mut i64,
    &mut 19_i64,
    "FastEscape(int(19))"
);

test!(i128, i128, 19_i128, "FastEscape(int(19))");
test!(i128_mut, &'a mut i128, &mut 19_i128, "FastEscape(int(19))");
test!(i128_borrow, &'a i128, &19_i128, "FastEscape(int(19))");
test!(
    i128_borrow_2,
    &'a &'a i128,
    &&19_i128,
    "FastEscape(int(19))"
);
test!(
    i128_borrow_mut,
    &'a mut i128,
    &mut 19_i128,
    "FastEscape(int(19))"
);

test!(isize, isize, 19_isize, "FastEscape(int(19))");
test!(
    isize_mut,
    &'a mut isize,
    &mut 19_isize,
    "FastEscape(int(19))"
);
test!(isize_borrow, &'a isize, &19_isize, "FastEscape(int(19))");
test!(
    isize_borrow_2,
    &'a &'a isize,
    &&19_isize,
    "FastEscape(int(19))"
);

test!(u8, u8, 19_u8, "FastEscape(int(19))");
test!(u8_mut, &'a mut u8, &mut 19_u8, "FastEscape(int(19))");
test!(u8_borrow, &'a u8, &19_u8, "FastEscape(int(19))");
test!(u8_borrow_2, &'a &'a u8, &&19_u8, "FastEscape(int(19))");

test!(u16, u16, 19_u16, "FastEscape(int(19))");
test!(u16_mut, &'a mut u16, &mut 19_u16, "FastEscape(int(19))");
test!(u16_borrow, &'a u16, &19_u16, "FastEscape(int(19))");
test!(u16_borrow_2, &'a &'a u16, &&19_u16, "FastEscape(int(19))");

test!(u32, u32, 19_u32, "FastEscape(int(19))");
test!(u32_mut, &'a mut u32, &mut 19_u32, "FastEscape(int(19))");
test!(u32_borrow, &'a u32, &19_u32, "FastEscape(int(19))");
test!(u32_borrow_2, &'a &'a u32, &&19_u32, "FastEscape(int(19))");

test!(u64, u64, 19_u64, "FastEscape(int(19))");
test!(u64_mut, &'a mut u64, &mut 19_u64, "FastEscape(int(19))");
test!(u64_borrow, &'a u64, &19_u64, "FastEscape(int(19))");
test!(u64_borrow_2, &'a &'a u64, &&19_u64, "FastEscape(int(19))");

test!(u128, u128, 19_u128, "FastEscape(int(19))");
test!(u128_mut, &'a mut u128, &mut 19_u128, "FastEscape(int(19))");
test!(u128_borrow, &'a u128, &19_u128, "FastEscape(int(19))");
test!(
    u128_borrow_2,
    &'a &'a u128,
    &&19_u128,
    "FastEscape(int(19))"
);

test!(usize, usize, 19_usize, "FastEscape(int(19))");
test!(
    usize_mut,
    &'a mut usize,
    &mut 19_usize,
    "FastEscape(int(19))"
);
test!(usize_borrow, &'a usize, &19_usize, "FastEscape(int(19))");
test!(
    usize_borrow_2,
    &'a &'a usize,
    &&19_usize,
    "FastEscape(int(19))"
);

test!(f64, f64, 19.19_f64, "Display(19.19)");
test!(f64_mut, &'a mut f64, &mut 19_f64, "Display(19)");
test!(f64_borrow, &'a f64, &19.19_f64, "Display(19.19)");
test!(f64_borrow_2, &'a &'a f64, &&19.19_f64, "Display(19.19)");

test!(f32, f32, 19.19_f32, "Display(19.19)");
test!(f32_mut, &'a mut f32, &mut 19_f32, "Display(19)");
test!(f32_borrow, &'a f32, &19.19_f32, "Display(19.19)");
test!(f32_borrow_2, &'a &'a f32, &&19.19_f32, "Display(19.19)");
