# Escaping

HTML escaping is on by default, so if a user provides this as their name in the example above:

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

### Escaping for other formats

Using Oxiplate to build TOML, JSON, XML, RTF, or _[insert format here]_ files?

You can switch the default escaper for all of your files:

```toml:/oxiplate.toml
default_escaper_group = "html"
```

Or switch it just for the document you're in:

```rust:
unimplemented!("Syntax not yet implemented and subject to change!")
```

```json.oxip
{% default_escaper_group json %}
{
    "name": "{{ name }}",
    "age": {{ number: age }},
}
```
