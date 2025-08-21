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

```html.oxip
Hello {{ name }}!
```

HTML escaping is on by default for `.html` and `.html.oxip` files,
so if a user provides this as their name in the example above:

```html
<script>alert('oh no');</script>
```

It would be safely escaped (even if it may look pretty strange):

```html
Hello &lt;script>alert('oh no');&lt;/script>!
```

You can use a different escape method whenever you want, like for HTML attributes:

```html.oxip
<a href="/{{ attr: handle }}" title="{{ attr: name }}">{{ name }}</a>
```

If you need to skip escaping, you can do that:

```html.oxip
<aside>{{ raw: your_html }}</aside>
```

And if you want to be explicit, `{{ name }}` and `{{ text: name }}` are equivalent.

### Escaping templates without matching file extensions

Using Oxiplate to build inline templates,
or templates that don't use file extensions that cleanly match up with escapers?

You can specify the escaper from the attribute:

```rust
#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ name }}")]
struct Data { name: &'static str }
```

You can also set a fallback escaper for any of your templates that don't specify an escaper group:

```toml:/oxiplate.toml
fallback_escaper_group = "html"
```

And finally, you can set the escaper group for the template you're in:

```json.oxip
{% default_escaper_group json %}
{
    "greeting": "Hello {{ name }}!",
}
```

If an escaper group was inferred from the extension, you can still override it:

```json.oxip:json.html
{% replace_escaper_group json %}
{
    "greeting": "Hello {{ name }}!",
}
```

### Require specifying the escaper

Oxiplate can be configured to require all writs to specify which escaper to use,
rather than falling back to the default escaper for the current escaper group:

```toml:/oxiplate.toml
require_specifying_escaper = true
```
