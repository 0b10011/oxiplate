use oxiplate::prelude::*;

// This behavior isn't necessarily required;
// but it'll require a breaking change to make the parser more strict.
#[test]
fn empty_table() {
    #[derive(Oxiplate)]
    #[oxiplate_inline("hello world")]
    struct Data;

    assert_eq!(format!("{}", Data.render().unwrap()), "hello world");
}
