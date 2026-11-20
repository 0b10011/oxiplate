# Escaping

Escaping values ensures user-generated content
can be safely used within trusted markup
without causing unintended side-effects.

In Oxiplate,
escapers are infallible;
they must always successfully output a safe string
for inclusion in the provided context.
Sometimes this means all unacceptable character sequences will be escaped,
while other times it could mean they are replaced or removed entirely.
This makes escapers improper for contexts
where doing so could change the correctness of the output,
like a JSON object value
where `raw` output in conjuction with known valid output is better.

## An example

```html:html.oxip
<p>Hello {{ name }}!</p>
```

HTML escaping is on by default for `.html` and `.html.oxip` files,
so if a user provides this as their name in the example above:

```html:name
<script>alert('oh no');</script>
```

It would be safely escaped (even if it may look pretty strange):

```html
<p>Hello &lt;script>alert('oh no');&lt;/script>!</p>
```

You can use a different escape method when appropriate, like for HTML attributes:

```html:html.oxip
<a href="/{{ attr: handle }}" title="{{ attr: name }}">{{ name }}</a>
```

If you need to skip escaping, you can do that:

```html:html.oxip
<aside>{{ raw: your_html }}</aside>
```

And if you want to be explicit, `{{ name }}` and `{{ text: name }}` are equivalent.

### Escaping templates without matching file extensions

Using Oxiplate to build inline templates,
or templates that don't use file extensions that cleanly match up with escapers?

You can specify the escaper from the attribute:

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ name }}")]
struct Data { name: &'static str }

assert_eq!(
    Data { name: "<Scarlett>" }.render()?,
    "&lt;Scarlett>",
);
#
# Ok::<(), ::core::fmt::Error>(())
```

You can also set a fallback escaper for any of your templates that don't specify an escaper group:

```toml:/oxiplate.toml
fallback_escaper_group = "html"
```

And finally, you can set the escaper group for the template you're in with the [default escaper group statement](../statements/default-escaper-group.md).

### Marking a type as "safe" for an escaper

All escapers should provide a trait
to mark a type as not needing to be escaped.
They should also mark types in the Rust library
as safe when appropriate;
for example,
integers are marked as safe for the built-in HTML escaper,
but not for the Markdown escaper.
This speeds up template generation
when these types are output in writs.

To mark one of your own types
as safe in HTML for text,
but not attributes or comments:

```rust
# extern crate oxiplate;
#
# struct YourType;
#
use oxiplate::escapers::html::{HtmlEscaper, IsEscapedForHtml};

impl IsEscapedForHtml for YourType {
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        match escaper {
            HtmlEscaper::text => true,
            HtmlEscaper::attr | HtmlEscaper::comment => false,
        }
    }
}
```

Each escaper provides a different trait to implement
to mark your type as safe for it.
The built-in escapers use:

- HTML: `oxiplate_traits::IsEscapedForHtml`
- JSON: `oxiplate_traits::IsEscapedForJson`
- Markdown: `oxiplate_traits::IsEscapedForMarkdown`

<div class="warning">

Marking a type as already escaped for an escaper
prevents escaping code from running against it for that escaper.
If a type can _ever_ convert into a string
that contains any character that should be escaped in that context,
it should not be marked as escaped.
Doing so on a website would make the website vulnerable
to [cross site scripting attacks][].
The one exception is a type
that wraps/generates markup
and handles escaping user-generated content itself.
When in doubt,
don't mark a type as safe.

</div>

[cross site scripting attacks]: https://community.owasp.org/attacks/xss/

### Require specifying the escaper

Oxiplate can be configured to require all writs to specify which escaper to use,
rather than falling back to the default escaper for the current escaper group:

```toml:/oxiplate.toml
require_specifying_escaper = true
```
