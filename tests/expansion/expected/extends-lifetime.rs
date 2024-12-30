#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate = "extends-deep.html.oxip"]
struct AbsoluteData<'a> {
    title: &'a str,
    message: &'a str,
}
impl<'a> std::fmt::Display for AbsoluteData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
            f.write_fmt(format_args!("{0}", "<h2>"))?;
            f.write_fmt(format_args!("{0}", self.title))?;
            f.write_fmt(format_args!("{0}", "</h2>\n  <div>"))?;
            f.write_fmt(format_args!("{0}", self.message))?;
            f.write_fmt(format_args!("{0}", "</div>"))?;
            Ok(())
        };
        #[oxiplate_extends = "{% extends \"extends-wrapper.html.oxip\" %}\n\n{% block content -%}\n    Some test content.\n{%- endblock %}\n"]
        struct Template<'a, F>
        where
            F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
        {
            _data: &'a AbsoluteData<'a>,
            content: &'a F,
        }
        impl<'a, F> std::fmt::Display for Template<'a, F>
        where
            F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let content = self.content;
                #[oxiplate_extends = "<!DOCTYPE html>\n<title>{{ title }}</title>\n{% block content -%}test{%- endblock %}\n"]
                struct ExtendingTemplate<'a, F>
                where
                    F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                {
                    _data: &'a &'a AbsoluteData<'a>,
                    content: &'a F,
                }
                impl<'a, F> std::fmt::Display for ExtendingTemplate<'a, F>
                where
                    F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_fmt(format_args!("{0}", "<!DOCTYPE html>\n<title>"))?;
                        f.write_fmt(format_args!("{0}", self._data.title))?;
                        f.write_fmt(format_args!("{0}", "</title>"))?;
                        f.write_fmt(format_args!("{0}", "\n"))?;
                        (self.content)(f)?;
                        f.write_fmt(format_args!("{0}", "\n"))?;
                        Ok(())
                    }
                }
                let template = ExtendingTemplate {
                    _data: &self._data,
                    content: &self.content,
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
        source_file: "tests\\extends-lifetime.rs",
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
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  <div>Hello world!</div>\n",
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
