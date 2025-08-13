use std::fmt::Display;

use oxiplate::{Oxiplate, Render};

struct HelloWorld;

impl HelloWorld {
    fn hello() -> String {
        String::from("Hello world &lt;<script><!--")
    }
}

impl Display for HelloWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hello world &lt;<script><!--")
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: "
# default:
{{ slice }}
{{ string }}
{{ integer }}
{{ float }}
{{ display }}
{{ display_borrowed }}
{{ fn_string }}

# text:
{{ text: slice }}
{{ text: string }}
{{ text: integer }}
{{ text: float }}
{{ text: display }}
{{ text: display_borrowed }}
{{ text: fn_string }}

# comment:
{{ comment: slice }}
{{ comment: string }}
{{ comment: integer }}
{{ comment: float }}
{{ comment: display }}
{{ comment: display_borrowed }}
{{ comment: fn_string }}

# raw:
{{ raw: slice }}
{{ raw: string }}
{{ raw: integer }}
{{ raw: float }}
{{ raw: display }}
{{ raw: display_borrowed }}
{{ raw: fn_string }}
")]
struct Types<'a> {
    slice: &'a str,
    string: String,
    integer: u64,
    float: f64,
    display: HelloWorld,
    display_borrowed: &'a HelloWorld,
    fn_string: String,
}

#[test]
fn types() {
    let data = Types {
        slice: "Hello world &lt;<script><!--",
        string: String::from("Hello world &lt;<script><!--"),
        integer: 19,
        float: 19.89,
        display: HelloWorld,
        display_borrowed: &HelloWorld,
        fn_string: HelloWorld::hello(),
    };

    assert_eq!(
        data.render().unwrap(),
        r"
# default:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# text:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# comment:
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−
19
19.89
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−

# raw:
Hello world &lt;<script><!--
Hello world &lt;<script><!--
19
19.89
Hello world &lt;<script><!--
Hello world &lt;<script><!--
Hello world &lt;<script><!--
"
    );
}
