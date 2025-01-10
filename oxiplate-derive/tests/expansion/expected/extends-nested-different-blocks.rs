#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "./extends-nested-different-blocks.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for AbsoluteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let content = |
            callback: fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            f: &mut ::std::fmt::Formatter<'_>,
        | -> ::std::fmt::Result {
            f.write_fmt(
                format_args!("<h1>{0}</h1>\n  <p>{1}</p>", self.title, self.message),
            )?;
            Ok(())
        };
        #[oxiplate_extends = "{% extends \"extends-nested-different-blocks-layout.html.oxip\" %}\n{% block body -%}\n  <main>\n    {%- block content -%}{%- endblock -%}\n  </main>\n{%- endblock %}"]
        struct Template<'a, F>
        where
            F: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            _data: &'a AbsoluteData,
            content: &'a F,
        }
        impl<'a, F> ::std::fmt::Display for Template<'a, F>
        where
            F: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let body = |
                    callback: fn(
                        f: &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result,
                    f: &mut ::std::fmt::Formatter<'_>,
                | -> ::std::fmt::Result {
                    f.write_str("<main>")?;
                    let content = |
                        f: &mut ::std::fmt::Formatter<'_>,
                    | -> ::std::fmt::Result { Ok(()) };
                    (self.content)(content, f)?;
                    f.write_str("</main>")?;
                    Ok(())
                };
                #[oxiplate_extends = "<DOCTYPE html>\n<head>\n  <title>{{ title }}</title>\n</head>\n<body>\n  {%- block body -%}{%- endblock -%}\n</body>\n"]
                struct ExtendingTemplate<'a, F>
                where
                    F: Fn(
                        fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                        &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result,
                {
                    _data: &'a &'a AbsoluteData,
                    body: &'a F,
                }
                impl<'a, F> ::std::fmt::Display for ExtendingTemplate<'a, F>
                where
                    F: Fn(
                        fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                        &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result,
                {
                    fn fmt(
                        &self,
                        f: &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result {
                        f.write_fmt(
                            format_args!(
                                "<DOCTYPE html>\n<head>\n  <title>{0}</title>\n</head>\n<body>",
                                self._data.title
                            ),
                        )?;
                        let body = |
                            f: &mut ::std::fmt::Formatter<'_>,
                        | -> ::std::fmt::Result { Ok(()) };
                        (self.body)(body, f)?;
                        f.write_str("</body>\n")?;
                        Ok(())
                    }
                }
                let template = ExtendingTemplate {
                    _data: &self._data,
                    body: &body,
                };
                f.write_fmt(format_args!("{0}", template))?;
                Ok(())
            }
        }
        let template = Template {
            _data: self,
            content: &content,
        };
        f.write_fmt(format_args!("{0}", template))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "absolute"]
#[doc(hidden)]
pub const absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\extends-nested-different-blocks.rs",
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
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"<DOCTYPE html>\n<head>\n  <title>Oxiplate \
         Example</title>\n</head>\n<body><main><h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p></main></body>\n",
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
