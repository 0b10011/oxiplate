#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "extends.html.oxip"]
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
        #[oxiplate_extends = "extends-wrapper.html.oxip"]
        struct Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a AbsoluteData,
            content: &'a Block1,
        }
        impl<'a, Block1> ::std::fmt::Display for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "<!DOCTYPE html>\n<title>{0}</title>\n",
                        self.oxiplate_extends_data.title,
                    ),
                )?;
                let content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                    f.write_str("test")?;
                    Ok(())
                };
                (self.content)(content, f)?;
                f.write_str("\n")?;
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
        source_file: "oxiplate/tests/extends.rs",
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
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n",
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
extern crate test;
#[rustc_test_marker = "absolute_2"]
#[doc(hidden)]
pub const absolute_2: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute_2"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/extends.rs",
        start_line: 25usize,
        start_col: 4usize,
        end_line: 25usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(absolute_2()),
    ),
};
fn absolute_2() {
    let data = AbsoluteData {
        title: "Oxiplate Example #2",
        message: "Goodbye world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Oxiplate Example #2</title>\n<h1>Oxiplate Example #2</h1>\n  \
         <p>Goodbye world!</p>\n",
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
#[oxiplate_inline = r#"{% extends "extends-wrapper.html.oxip" %}
{% block(prefix) content -%}
    <p>{{ message }}</p>
{%- endblock %}
"#]
struct Prefix {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let content = |
            callback: fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            f: &mut ::std::fmt::Formatter<'_>,
        | -> ::std::fmt::Result {
            f.write_fmt(format_args!("<p>{0}</p>", self.message))?;
            callback(f)?;
            Ok(())
        };
        #[oxiplate_extends = "extends-wrapper.html.oxip"]
        struct Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a Prefix,
            content: &'a Block1,
        }
        impl<'a, Block1> ::std::fmt::Display for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "<!DOCTYPE html>\n<title>{0}</title>\n",
                        self.oxiplate_extends_data.title,
                    ),
                )?;
                let content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                    f.write_str("test")?;
                    Ok(())
                };
                (self.content)(content, f)?;
                f.write_str("\n")?;
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
#[rustc_test_marker = "prefix"]
#[doc(hidden)]
pub const prefix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("prefix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/extends.rs",
        start_line: 50usize,
        start_col: 4usize,
        end_line: 50usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(prefix())),
};
fn prefix() {
    let data = Prefix {
        title: "Prefixed block",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Prefixed block</title>\n<p>Hello world!</p>test\n",
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
#[oxiplate_inline = r#"{% extends "extends-wrapper.html.oxip" %}
{% block(replace) content -%}
    <p>{{ message }}</p>
{%- endblock %}
"#]
struct Replace {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for Replace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let content = |
            callback: fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            f: &mut ::std::fmt::Formatter<'_>,
        | -> ::std::fmt::Result {
            f.write_fmt(format_args!("<p>{0}</p>", self.message))?;
            Ok(())
        };
        #[oxiplate_extends = "extends-wrapper.html.oxip"]
        struct Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a Replace,
            content: &'a Block1,
        }
        impl<'a, Block1> ::std::fmt::Display for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "<!DOCTYPE html>\n<title>{0}</title>\n",
                        self.oxiplate_extends_data.title,
                    ),
                )?;
                let content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                    f.write_str("test")?;
                    Ok(())
                };
                (self.content)(content, f)?;
                f.write_str("\n")?;
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
#[rustc_test_marker = "replace"]
#[doc(hidden)]
pub const replace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("replace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/extends.rs",
        start_line: 74usize,
        start_col: 4usize,
        end_line: 74usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(replace())),
};
fn replace() {
    let data = Replace {
        title: "Replaced block",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Replaced block</title>\n<p>Hello world!</p>\n",
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
#[oxiplate_inline = r#"{% extends "extends-wrapper.html.oxip" %}
{% block(suffix) content -%}
    <p>{{ message }}</p>
{%- endblock %}
"#]
struct Suffix {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for Suffix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let content = |
            callback: fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            f: &mut ::std::fmt::Formatter<'_>,
        | -> ::std::fmt::Result {
            callback(f)?;
            f.write_fmt(format_args!("<p>{0}</p>", self.message))?;
            Ok(())
        };
        #[oxiplate_extends = "extends-wrapper.html.oxip"]
        struct Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            #[allow(dead_code)]
            oxiplate_extends_data: &'a Suffix,
            content: &'a Block1,
        }
        impl<'a, Block1> ::std::fmt::Display for Template<'a, Block1>
        where
            Block1: Fn(
                fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "<!DOCTYPE html>\n<title>{0}</title>\n",
                        self.oxiplate_extends_data.title,
                    ),
                )?;
                let content = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                    f.write_str("test")?;
                    Ok(())
                };
                (self.content)(content, f)?;
                f.write_str("\n")?;
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
#[rustc_test_marker = "suffix"]
#[doc(hidden)]
pub const suffix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("suffix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/extends.rs",
        start_line: 98usize,
        start_col: 4usize,
        end_line: 98usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(suffix())),
};
fn suffix() {
    let data = Suffix {
        title: "Suffixed block",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Suffixed block</title>\ntest<p>Hello world!</p>\n",
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
    test::test_main_static(&[&absolute, &absolute_2, &prefix, &replace, &suffix])
}
