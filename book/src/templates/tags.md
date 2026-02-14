# Tags

Tags start with `{` and end with `}` with one or more characters between to define the type of tag and any contained logic.

## Writs

[Writs](writs/index.md) are [expressions](expressions.md) wrapped with `{{` and `}}` that will be evaluated and output into the template:

```oxip
Hello {{ name }}!
```

```text
Hello Luna!
```

## Statements

[Statements](statements/index.md) are wrapped with `{%` and `%}` and include variable assignments and control structures:

```html.oxip
{% if user.is_some() %}<a href="/account/">Account</a>{% endif %}
```

```html
<a href="/login/">Log In</a>
```

## Comments

Comments are text wrapped with `{#` and `#}` that won't appear in the final template:

```oxip
Hello world.{# Comments are ignored. #}
```

```text
Hello world.
```
