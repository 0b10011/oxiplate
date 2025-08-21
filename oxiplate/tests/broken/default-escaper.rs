use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{% default_escaper_group json %}{% default_escaper_group json %}{{ title }}")]
struct JsonInHtml {
    title: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline("<!DOCTYPE html>{% default_escaper_group html %}{{ title }}")]
struct DefaultAfterContent {
    title: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline("<!DOCTYPE html>{% replace_escaper_group html %}{{ title }}")]
struct ReplaceAfterContent {
    title: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline("{% default_escaper_group escaper_that_does_not_exist %}{{ title }}")]
struct MissingEscaperGroup {
    title: &'static str,
}

pub fn main() {
    let title = "<!DOCTYPE html>Hello world";
    JsonInHtml { title }.render().unwrap();
    DefaultAfterContent { title }.render().unwrap();
    ReplaceAfterContent { title }.render().unwrap();
    MissingEscaperGroup { title }.render().unwrap();
}