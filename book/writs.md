# Writs

A **writ** is an [**expression**](expressions.md) wrapped with `{{` and `}}` that will be evaluated and output into the template. For example, `Hello {{ name }}!` may become `Hello Luna!`.

## Escaping

HTML escaping is on by default, so if a user provides this as their name in the example above:

```html
<script>alert('oh no');</script>
```

It would be safely escaped (even if it may look pretty strange):

```html
Hello &lt;script>alert('oh no');&lt;/script>!
```

You can use a different escape method whenever you want, like for HTML attributes:

```oxip.html
<a href="/{{ attr: handle }}" title="{{ attr: name }}">{{ name }}</a>
```

If you need to skip escaping, you can do that:

```oxip.html
<aside>{{ raw: your_html }}</aside>
```

And if you want to be explicit, `{{ name }}` and `{{ text: name }}` are equivalent.

### Escaping for other formats

```rust
unimplemented!("Syntax not yet implemented and subject to change!")
```

Using Oxiplate to build JSON, YAML, XML, RTF, or _[insert format here]_ files?

You can switch the default escaper for all of your files:

```oxip.yaml
# /oxiplate.yml
default_escaper: json
```

Or switch it for the document you're in:

```oxip.json
{% default_escaper json %}
{
    "name": "{{ name }}",
    "age": {{ number: age }},
}
```
