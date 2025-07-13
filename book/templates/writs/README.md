# Writs

A writ is an [expression](../expressions.md) wrapped with `{{` and `}}` that will be evaluated and output into the template.

```oxip
Hello {{ name }}!
```

```text
Hello Luna!
```

But writs support _any_ expression:

```oxip
{{ a }} + {{ b }} = {{ a + b }}
```

```text
1 + 2 = 3
```

No matter how complicated:

```oxip
{{ (user.name() | upper) ~ " (" ~ (user.company() | lower) ~ ")" }}
```

```text
CASPER (sloths and stuff, inc.)
```

Expressions will be explained in more detail in a later chapter.

## Escaping

Eventually you'll likely want to [escape user-provided text](escaping.md) for safe usage within a markup language. Set a default escaper group and manually specify the escaper anywhere the default escaper for the group won't work:

```toml
escaper_groups.html.escaper = "::oxiplate::escapers::HtmlEscaper"
```

```oxip
<a href="/{{ attr: user.username }}" title="Visit {{ attr: user.name }}'s profile">
    {{ user.name }}
</a>
```

```html
<a href="/toya_the_sequoia" title="Visit Isabelle &#34;Cat &amp; Mouse&#34; Toya's profile">
    Isabelle "Cat &amp; Mouse" Toya
</a>
```

Read more about escaping in the [next chapter](escaping.md).
