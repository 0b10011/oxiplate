use alloc::format;
use alloc::string::String;
use core::fmt::Display;

use oxiplate_derive::Oxiplate;

#[test]
fn as_u64() {
    #[derive(Oxiplate)]
    #[oxiplate_inline("{{ unsigned_16 as u64 + unsigned_64 }}")]
    struct Data {
        unsigned_16: u16,
        unsigned_64: u64,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                unsigned_16: 10_u16,
                unsigned_64: 9_u64,
            }
        ),
        "19"
    );
}

#[test]
fn as_trait() {
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
    #[oxiplate_inline("{{ &greeting }}")]
    struct AsGreeting {
        greeting: Greeting,
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline("{{ &greeting as &dyn Yell }}")]
    struct AsYell {
        greeting: Greeting,
    }

    assert_eq!(
        format!(
            "{}",
            AsGreeting {
                greeting: Greeting {
                    message: "Hello world!"
                },
            }
        ),
        "Hello world!"
    );

    assert_eq!(
        format!(
            "{}",
            AsYell {
                greeting: Greeting {
                    message: "Hello world!"
                },
            }
        ),
        "HELLO WORLD!"
    );
}
