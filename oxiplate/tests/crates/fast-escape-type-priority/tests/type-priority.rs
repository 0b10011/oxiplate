use std::borrow::Cow;

use oxiplate::private::CowStrWrapper;
use oxiplate::{Oxiplate, Render};

macro_rules! test {
    ($fn_name:ident, $ty:ty, $value:expr, $escaper_group:ident, $specified_escaper_writ:literal, $expected:expr) => {
        test!($fn_name, $ty, $value, $escaper_group, $specified_escaper_writ, $expected, $expected);
    };
    ($fn_name:ident, $ty:ty, $value:expr, $escaper_group:ident, $specified_escaper_writ:literal, $expected_escaped:expr, $expected_raw:expr) => {
        #[test]
        fn $fn_name() {
            test!(
                $escaper_group,
                $specified_escaper_writ,
                $ty,
                $value,
                $expected_escaped,
                "escaped (text)"
            );
            test!(
                $escaper_group,
                "{{ value }}",
                $ty,
                $value,
                $expected_escaped,
                "escaped (default)"
            );
            test!(
                $escaper_group,"{{ raw: value }}", $ty, $value, $expected_raw, "raw");

            // Box
            let value = $value;
            test!(
                $escaper_group,
                "{{ value }}",
                Box<$ty>,
                Box::new(value),
                $expected_escaped,
                "box escaped"
            );
            let value = $value;
            test!(
                $escaper_group,
                "{{ raw: value }}",
                Box<$ty>,
                Box::new(value),
                $expected_raw,
                "box raw"
            );

            // Rc
            let value = $value;
            test!(
                $escaper_group,
                "{{ value }}",
                ::std::rc::Rc<$ty>,
                ::std::rc::Rc::new(value),
                $expected_escaped,
                "rc escaped"
            );
            let value = $value;
            test!(
                $escaper_group,
                "{{ raw: value }}",
                ::std::rc::Rc<$ty>,
                ::std::rc::Rc::new(value),
                $expected_raw,
                "rc raw"
            );

            // Arc
            let value = $value;
            test!(
                $escaper_group,
                "{{ value }}",
                ::std::sync::Arc<$ty>,
                ::std::sync::Arc::new(value),
                $expected_escaped,
                "arc escaped"
            );
            let value = $value;
            test!(
                $escaper_group,
                "{{ raw: value }}",
                ::std::sync::Arc<$ty>,
                ::std::sync::Arc::new(value),
                $expected_raw,
                "arc raw"
            );
        }
    };
    ($escaper_group:ident, $template:literal, $ty:ty, $value:expr, $expected:expr, $message:literal) => {{
        #[derive(Oxiplate)]
        #[oxiplate_inline($escaper_group: $template)]
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

macro_rules! test_int_19 {
    ($fn_name_html:ident, $fn_name_json:ident, $fn_name_md:ident, $ty:ty, $value:expr) => {
        test!(
            $fn_name_html,
            $ty,
            $value,
            html,
            "{{ text: value }}",
            "FastWriteEscaped(FastWriteRaw(int(19)))",
            "FastWriteRaw(int(19))"
        );
        test!(
            $fn_name_json,
            $ty,
            $value,
            json,
            "{{ substring: value }}",
            "FastWriteEscaped(FastWriteRaw(int(19)))",
            "FastWriteRaw(int(19))"
        );
        test!(
            $fn_name_md,
            $ty,
            $value,
            md,
            "{{ text: value }}",
            "FastWriteEscaped(int(19))",
            "FastWriteRaw(int(19))"
        );
    };
}

macro_rules! test_float_19 {
    ($fn_name_html:ident, $fn_name_json:ident, $fn_name_md:ident, $ty:ty, $value:expr) => {
        test!(
            $fn_name_html,
            $ty,
            $value,
            html,
            "{{ text: value }}",
            "DisplayEscaped(DisplayRaw(19))",
            "DisplayRaw(19)"
        );
        test!(
            $fn_name_json,
            $ty,
            $value,
            json,
            "{{ substring: value }}",
            "DisplayEscaped(DisplayRaw(19))",
            "DisplayRaw(19)"
        );
        test!(
            $fn_name_md,
            $ty,
            $value,
            md,
            "{{ text: value }}",
            "DisplayEscaped(19)",
            "DisplayRaw(19)"
        );
    };
}

macro_rules! test_float_19_19 {
    ($fn_name_html:ident, $fn_name_json:ident, $fn_name_md:ident, $ty:ty, $value:expr) => {
        test!(
            $fn_name_html,
            $ty,
            $value,
            html,
            "{{ text: value }}",
            "DisplayEscaped(DisplayRaw(19.19))",
            "DisplayRaw(19.19)"
        );
        test!(
            $fn_name_json,
            $ty,
            $value,
            json,
            "{{ substring: value }}",
            "DisplayEscaped(DisplayRaw(19.19))",
            "DisplayRaw(19.19)"
        );
        test!(
            $fn_name_md,
            $ty,
            $value,
            md,
            "{{ text: value }}",
            r"DisplayEscaped(19\.19)",
            "DisplayRaw(19.19)"
        );
    };
}

test!(
    cow_str_wrapper_string,
    CowStrWrapper<'a>,
    CowStrWrapper::new(Cow::Owned("text".to_string())),
    html,
    "{{ text: value }}",
    "FastWriteEscaped(CowStrWrapper(text))",
    "FastWriteRaw(CowStrWrapper(text))"
);
test!(
    cow_str_wrapper_str,
    CowStrWrapper<'a>,
    CowStrWrapper::new(Cow::Borrowed("text")),
    html,
    "{{ text: value }}",
    "FastWriteEscaped(CowStrWrapper(text))",
    "FastWriteRaw(CowStrWrapper(text))"
);
test!(
    string,
    String,
    "text".to_string(),
    html,
    "{{ text: value }}",
    "FastWriteEscaped(String(text))",
    "FastWriteRaw(String(text))"
);
test!(
    string_mut,
    &'a mut String,
    &mut "text".to_string(),
    html,
    "{{ text: value }}",
    "FastWriteEscaped(String(text))",
    "FastWriteRaw(String(text))"
);
test!(
    str,
    &'a str,
    "text",
    html,
    "{{ text: value }}",
    "FastWriteEscaped(str(text))",
    "FastWriteRaw(str(text))"
);
test!(
    str_2,
    &'a &'a str,
    &"text",
    html,
    "{{ text: value }}",
    "FastWriteEscaped(str(text))",
    "FastWriteRaw(str(text))"
);

test_int_19!(i8_html, i8_json, i8_md, i8, 19_i8);
test_int_19!(i8_mut_html, i8_mut_json, i8_mut_md, &'a mut i8, &mut 19_i8);
test_int_19!(i8_borrow_html, i8_borrow_json, i8_borrow_md, &'a i8, &19_i8);
test_int_19!(i8_borrow_2_html, i8_borrow_2_json, i8_borrow_2_md, &'a &'a i8, &&19_i8);
test_int_19!(i8_borrow_mut_html, i8_borrow_mut_json, i8_borrow_mut_md, &'a mut i8, &mut 19_i8);

test_int_19!(i16_html, i16_json, i16_md, i16, 19_i16);
test_int_19!(i16_mut_html, i16_mut_json, i16_mut_md, &'a mut i16, &mut 19_i16);
test_int_19!(i16_borrow_html, i16_borrow_json, i16_borrow_md, &'a i16, &19_i16);
test_int_19!(i16_borrow_2_html, i16_borrow_2_json, i16_borrow_2_md, &'a &'a i16, &&19_i16);
test_int_19!(i16_borrow_mut_html, i16_borrow_mut_json, i16_borrow_mut_md, &'a mut i16, &mut 19_i16);

test_int_19!(i32_html, i32_json, i32_md, i32, 19_i32);
test_int_19!(i32_mut_html, i32_mut_json, i32_mut_md, &'a mut i32, &mut 19_i32);
test_int_19!(i32_borrow_html, i32_borrow_json, i32_borrow_md, &'a i32, &19_i32);
test_int_19!(i32_borrow_2_html, i32_borrow_2_json, i32_borrow_2_md, &'a &'a i32, &&19_i32);
test_int_19!(i32_borrow_mut_html, i32_borrow_mut_json, i32_borrow_mut_md, &'a mut i32, &mut 19_i32);

test_int_19!(i64_html, i64_json, i64_md, i64, 19_i64);
test_int_19!(i64_mut_html, i64_mut_json, i64_mut_md, &'a mut i64, &mut 19_i64);
test_int_19!(i64_borrow_html, i64_borrow_json, i64_borrow_md, &'a i64, &19_i64);
test_int_19!(i64_borrow_2_html, i64_borrow_2_json, i64_borrow_2_md, &'a &'a i64, &&19_i64);
test_int_19!(i64_borrow_mut_html, i64_borrow_mut_json, i64_borrow_mut_md, &'a mut i64, &mut 19_i64);

test_int_19!(i128_html, i128_json, i128_md, i128, 19_i128);
test_int_19!(i128_mut_html, i128_mut_json, i128_mut_md, &'a mut i128, &mut 19_i128);
test_int_19!(i128_borrow_html, i128_borrow_json, i128_borrow_md, &'a i128, &19_i128);
test_int_19!(i128_borrow_2_html, i128_borrow_2_json, i128_borrow_2_md, &'a &'a i128, &&19_i128);
test_int_19!(i128_borrow_mut_html, i128_borrow_mut_json, i128_borrow_mut_md, &'a mut i128, &mut 19_i128);

test_int_19!(isize_html, isize_json, isize_md, isize, 19_isize);
test_int_19!(isize_mut_html, isize_mut_json, isize_mut_md, &'a mut isize, &mut 19_isize);
test_int_19!(isize_borrow_html, isize_borrow_json, isize_borrow_md, &'a isize, &19_isize);
test_int_19!(isize_borrow_2_html, isize_borrow_2_json, isize_borrow_2_md, &'a &'a isize, &&19_isize);

test_int_19!(u8_html, u8_json, u8_md, u8, 19_u8);
test_int_19!(u8_mut_html, u8_mut_json, u8_mut_md, &'a mut u8, &mut 19_u8);
test_int_19!(u8_borrow_html, u8_borrow_json, u8_borrow_md, &'a u8, &19_u8);
test_int_19!(u8_borrow_2_html, u8_borrow_2_json, u8_borrow_2_md, &'a &'a u8, &&19_u8);

test_int_19!(u16_html, u16_json, u16_md, u16, 19_u16);
test_int_19!(u16_mut_html, u16_mut_json, u16_mut_md, &'a mut u16, &mut 19_u16);
test_int_19!(u16_borrow_html, u16_borrow_json, u16_borrow_md, &'a u16, &19_u16);
test_int_19!(u16_borrow_2_html, u16_borrow_2_json, u16_borrow_2_md, &'a &'a u16, &&19_u16);

test_int_19!(u32_html, u32_json, u32_md, u32, 19_u32);
test_int_19!(u32_mut_html, u32_mut_json, u32_mut_md, &'a mut u32, &mut 19_u32);
test_int_19!(u32_borrow_html, u32_borrow_json, u32_borrow_md, &'a u32, &19_u32);
test_int_19!(u32_borrow_2_html, u32_borrow_2_json, u32_borrow_2_md, &'a &'a u32, &&19_u32);

test_int_19!(u64_html, u64_json, u64_md, u64, 19_u64);
test_int_19!(u64_mut_html, u64_mut_json, u64_mut_md, &'a mut u64, &mut 19_u64);
test_int_19!(u64_borrow_html, u64_borrow_json, u64_borrow_md, &'a u64, &19_u64);
test_int_19!(u64_borrow_2_html, u64_borrow_2_json, u64_borrow_2_md, &'a &'a u64, &&19_u64);

test_int_19!(u128_html, u128_json, u128_md, u128, 19_u128);
test_int_19!(u128_mut_html, u128_mut_json, u128_mut_md, &'a mut u128, &mut 19_u128);
test_int_19!(u128_borrow_html, u128_borrow_json, u128_borrow_md, &'a u128, &19_u128);
test_int_19!(u128_borrow_2_html, u128_borrow_2_json, u128_borrow_2_md, &'a &'a u128, &&19_u128);

test_int_19!(usize_html, usize_json, usize_md, usize, 19_usize);
test_int_19!(usize_mut_html, usize_mut_json, usize_mut_md, &'a mut usize, &mut 19_usize);
test_int_19!(usize_borrow_html, usize_borrow_json, usize_borrow_md, &'a usize, &19_usize);
test_int_19!(usize_borrow_2_html, usize_borrow_2_json, usize_borrow_2_md, &'a &'a usize, &&19_usize);

test_float_19_19!(f64_html, f64_json, f64_md, f64, 19.19_f64);
test_float_19!(f64_mut_html, f64_mut_json, f64_mut_md, &'a mut f64, &mut 19_f64);
test_float_19_19!(f64_borrow_html, f64_borrow_json, f64_borrow_md, &'a f64, &19.19_f64);
test_float_19_19!(f64_borrow_2_html, f64_borrow_2_json, f64_borrow_2_md, &'a &'a f64, &&19.19_f64);

test_float_19_19!(f32_html, f32_json, f32_md, f32, 19.19_f32);
test_float_19!(f32_mut_html, f32_mut_json, f32_mut_md, &'a mut f32, &mut 19_f32);
test_float_19_19!(f32_borrow_html, f32_borrow_json, f32_borrow_md, &'a f32, &19.19_f32);
test_float_19_19!(f32_borrow_2_html, f32_borrow_2_json, f32_borrow_2_md, &'a &'a f32, &&19.19_f32);
