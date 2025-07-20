# Whitespace control

All [tags](tags.md) support the following whitespace control characters:

- `-` (`U+002D HYPHEN-MINUS`) will remove all matched whitespace
- `_` (`U+005F LOW LINE`) will replace all matched whitespace with a single space (`U+0020 SPACE`)

To adjust whitespace before the tag, the whitespace control character must be added immediately following the opening `{{`, `{%`, or `{#`.

To adjust whitespace after the tag, the whitespace control character must be added immediately before the closing `}}`, `%}`, or `#}`.

If no whitespace control character is present, the matched whitespace will be left as-is.

For example, `a {{- "b" _}}     c` would become `ab c`.

## Short tags

There are also a couple short tags available for controlling whitespace elsewhere in templates:

- `{-}` will remove all surrounding whitespace
- `{_}` will replace all surrounding whitespace with a single space (`U+0020 SPACE`)

For example:

```html.oxip
<p>{-}
    Hello {_}
    world! {-}
</p>
```

will become:

```html
<p>Hello world!</p>
```
