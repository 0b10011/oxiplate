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

Using Oxiplate to build XML, RTF, TOML, JSON, or _[insert format here]_ files?

You can switch the default escaper for all of your files:

```toml:/oxiplate.toml
default_escaper_group = "html"
```

Or switch it just for the document you're in:

<div class="warning">

`default_escaper_group` is not yet implemented ([#39](https://github.com/0b10011/oxiplate/issues/39)).

</div>

```json.oxip
{% default_escaper_group json %}
{
    "greeting": "Hello {{ name }}!",
}
```
