#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "./extends-deep.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for AbsoluteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render(self, f)
    }
}
impl ::oxiplate::Render for AbsoluteData {
    fn render<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        let content = {
            use ::std::fmt::Write;
            |
                callback: fn(f: &mut dyn Write) -> ::std::fmt::Result,
                f: &mut dyn Write,
            | -> ::std::fmt::Result {
                f.write_str("<h2>")?;
                f.write_str(
                    &::oxiplate::escapers::escape(
                        &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                        &::std::string::ToString::to_string(&self.title),
                    ),
                )?;
                f.write_str("</h2>\n  <div>")?;
                f.write_str(
                    &::oxiplate::escapers::escape(
                        &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                        &::std::string::ToString::to_string(&self.message),
                    ),
                )?;
                f.write_str("</div>")?;
                Ok(())
            }
        };
        #[oxiplate_extends = "extends-inner-wrapper.html.oxip"]
        struct Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a AbsoluteData,
            content: &'a Block1,
        }
        impl<'a, Block1> ::std::fmt::Display for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::oxiplate::Render::render(self, f)
            }
        }
        impl<'a, Block1> ::oxiplate::Render for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut dyn Write) -> ::std::fmt::Result,
                &mut dyn Write,
            ) -> ::std::fmt::Result,
        {
            fn render<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
                use ::std::fmt::Write;
                let content = self.content;
                #[oxiplate_extends = "extends-wrapper.html.oxip"]
                struct ExtendingTemplate<'a, Block1>
                where
                    Block1: Fn(
                        fn(f: &mut dyn Write) -> ::std::fmt::Result,
                        &mut dyn Write,
                    ) -> ::std::fmt::Result,
                {
                    #[allow(dead_code)]
                    oxiplate_extends_data: &'a &'a AbsoluteData,
                    content: &'a Block1,
                }
                impl<'a, Block1> ::std::fmt::Display for ExtendingTemplate<'a, Block1>
                where
                    Block1: Fn(
                        fn(f: &mut dyn Write) -> ::std::fmt::Result,
                        &mut dyn Write,
                    ) -> ::std::fmt::Result,
                {
                    fn fmt(
                        &self,
                        f: &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result {
                        ::oxiplate::Render::render(self, f)
                    }
                }
                impl<'a, Block1> ::oxiplate::Render for ExtendingTemplate<'a, Block1>
                where
                    Block1: Fn(
                        fn(f: &mut dyn Write) -> ::std::fmt::Result,
                        &mut dyn Write,
                    ) -> ::std::fmt::Result,
                {
                    fn render<W: ::std::fmt::Write>(
                        &self,
                        f: &mut W,
                    ) -> ::std::fmt::Result {
                        use ::std::fmt::Write;
                        f.write_str("<!DOCTYPE html>\n<title>")?;
                        f.write_str(
                            &::oxiplate::escapers::escape(
                                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                                &::std::string::ToString::to_string(
                                    &self.oxiplate_extends_data.title,
                                ),
                            ),
                        )?;
                        f.write_str("</title>\n")?;
                        {
                            use ::std::fmt::Write;
                            let content = |f: &mut dyn Write| -> ::std::fmt::Result {
                                f.write_str("test")?;
                                Ok(())
                            };
                            (self.content)(content, f)?;
                        }
                        f.write_str("\n")?;
                        Ok(())
                    }
                }
                let template = ExtendingTemplate {
                    oxiplate_extends_data: &self.oxiplate_extends_data,
                    content: &self.content,
                };
                f.write_fmt(format_args!("{0}", template))?;
                Ok(())
            }
        }
        let template = Template {
            oxiplate_extends_data: self,
            content: &content,
        };
        f.write_fmt(format_args!("{0}", template))?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "absolute"]
#[doc(hidden)]
pub const absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/extends-nested.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(absolute())),
};
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  \
         <div>Hello world!</div>\n",
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
    test::test_main_static(&[&absolute])
}
