use std::fmt::Display;

use oxiplate::Oxiplate;

struct HelloWorld;

impl HelloWorld {
    fn hello() -> String {
        String::from("Hello world")
    }
}

impl Display for HelloWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hello world")
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline = "
{{ slice }}
{{ string }}
{{ integer }}
{{ float }}
{{ display }}
{{ fn_string }}

{{ text: slice }}
{{ text: string }}
{{ text: integer }}
{{ text: float }}
{{ text: display }}
{{ text: fn_string }}

{{ raw: slice }}
{{ raw: string }}
{{ raw: integer }}
{{ raw: float }}
{{ raw: display }}
{{ raw: fn_string }}
"]
struct Types<'a> {
    slice: &'a str,
    string: String,
    integer: u64,
    float: f64,
    display: HelloWorld,
    fn_string: String,
}

#[test]
fn types() {
    let data = Types {
        slice: "Hello world",
        string: String::from("Hello world"),
        integer: 19,
        float: 19.89,
        display: HelloWorld,
        fn_string: HelloWorld::hello(),
    };

    assert_eq!(
        format!("{data}"),
        r"
Hello world
Hello world
19
19.89
Hello world
Hello world

Hello world
Hello world
19
19.89
Hello world
Hello world

Hello world
Hello world
19
19.89
Hello world
Hello world
"
    );
}
