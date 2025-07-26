#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "./multiple-blocks-inner.html.oxip"]
struct Data;
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render(self, f)
    }
}
impl ::oxiplate::Render for Data {
    #[inline]
    fn render<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        let header = {
            use ::std::fmt::Write;
            |
                callback: fn(f: &mut dyn Write) -> ::std::fmt::Result,
                f: &mut dyn Write,
            | -> ::std::fmt::Result {
                f.write_str("header")?;
                Ok(())
            }
        };
        let main = {
            use ::std::fmt::Write;
            |
                callback: fn(f: &mut dyn Write) -> ::std::fmt::Result,
                f: &mut dyn Write,
            | -> ::std::fmt::Result {
                f.write_str("main")?;
                Ok(())
            }
        };
        let footer = {
            use ::std::fmt::Write;
            |
                callback: fn(f: &mut dyn Write) -> ::std::fmt::Result,
                f: &mut dyn Write,
            | -> ::std::fmt::Result {
                f.write_str("footer")?;
                Ok(())
            }
        };
        #[oxiplate_extends = "multiple-blocks.html.oxip"]
        struct Template<'a, Block1, Block2, Block3>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block2: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block3: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a Data,
            header: &'a Block1,
            main: &'a Block2,
            footer: &'a Block3,
        }
        impl<'a, Block1, Block2, Block3> ::std::fmt::Display
        for Template<'a, Block1, Block2, Block3>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block2: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block3: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::oxiplate::Render::render(self, f)
            }
        }
        impl<'a, Block1, Block2, Block3> ::oxiplate::Render
        for Template<'a, Block1, Block2, Block3>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block2: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
            Block3: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            #[inline]
            fn render<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
                use ::std::fmt::Write;
                f.write_str("<!DOCTYPE html>\n<header>")?;
                {
                    use ::std::fmt::Write;
                    let header = |f: &mut dyn Write| -> ::std::fmt::Result { Ok(()) };
                    (self.header)(header, f)?;
                }
                f.write_str("</header>\n<main>")?;
                {
                    use ::std::fmt::Write;
                    let main = |f: &mut dyn Write| -> ::std::fmt::Result { Ok(()) };
                    (self.main)(main, f)?;
                }
                f.write_str("</main>\n<footer>")?;
                {
                    use ::std::fmt::Write;
                    let footer = |f: &mut dyn Write| -> ::std::fmt::Result { Ok(()) };
                    (self.footer)(footer, f)?;
                }
                f.write_str("</footer>")?;
                Ok(())
            }
        }
        let template = Template {
            oxiplate_extends_data: self,
            header: &header,
            main: &main,
            footer: &footer,
        };
        f.write_fmt(format_args!("{0}", template))?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "multiple_blocks"]
#[doc(hidden)]
pub const multiple_blocks: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("multiple_blocks"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/multiple-blocks.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(multiple_blocks()),
    ),
};
fn multiple_blocks() {
    let data = Data;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<header>header</header>\n<main>main</main>\n<footer>footer</footer>",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&multiple_blocks])
}
