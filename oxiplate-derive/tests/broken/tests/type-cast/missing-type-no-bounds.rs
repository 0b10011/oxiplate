use std::fmt::Display;

use oxiplate_derive::Oxiplate;

struct Greeting {
    message: &'static str,
}

impl Display for Greeting {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message)
    }
}

trait Yell {
    fn yell(&self) -> String;
}

impl Yell for Greeting {
    fn yell(&self) -> String {
        self.message.to_uppercase()
    }
}

impl Display for dyn Yell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.yell())
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ &greeting as }}")]
struct AsYell {
    greeting: Greeting,
}

fn main() {
    let data = AsYell {
        greeting: Greeting {
            message: "Hello world!",
        },
    };

    print!("{}", data);
}
