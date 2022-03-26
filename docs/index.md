title: "Oxiplate: Template language for Rust"

{% raw -%}

**Oxiplate is experimental and features described here may not yet be implemented, or may be implemented in a different way.**

## Template syntax

The syntax for templates is similar to many other systems, but the terminology may be slightly different.

A **writ** is an [**expression**](#expressions) wrapped with `{{` and `}}` that will be evaluated and output into the template. For example, `Hello {{ name }}!` may become `Hello Luna!`.

A [**statement**](#statements) is wrapped with `{%` and `%}` and includes variable assignments and control structures. See the [**statement**](#statements) section for a list of possible statements.

A **comment** is text wrapped with `{#` and `#}` that will be removed from the final template, but can be useful to the template designer(s). For example, `{# None of this text would show up in the final template. #}`.

Whitespace before and after **tags** can be removed or collapsed. See the [whitespace control](#whitespace-control) section for more information.

Anything else in the template is considered **static** and will be left as-is.

### Expressions

#### Extends and block

```oxip
{# layout.html.oxip -#}

<!DOCTYPE html>
<main>{% block content %}{% endblock %}</main>
```

```oxip
{# your-content.html.oxip -#}

{% extends "layout.html.oxip" %}

{% block content %}
  <h1>Your content</h1>
  <p>Goes here...</p>
{% endblock %}
```

```html
<!DOCTYPE html>
<main>
  <h1>Your content</h1>
  <p>Goes here...</p>
</main>
```

#### If/elseif/else

```rust
struct YourStruct {
    autofocus: &'static bool,
    minlength: &'static Option<u64>,
    maxlength: &'static Option<u64>,
    id: &'static str,
}
```

```oxip
<input
  {{_ autofocus ? "autofocus" }}
  {%_ if let Some(minlength) = minlength %}minlength="{{ attr: minlength }}"{% endif %}
  {%_ if maxlength.is_some() %}maxlength="{{ attr: maxlength }}"{% endif %}
```

#### For

```
<ul>
  {% for name in names %}
    <li>{{ name }}
  {% endfor %}
</ul>
```

### Statements

### Whitespace control

Whitespace (spaces, tabs, newlines, etc) that comes before a **tag** can be removed by appending `-` to the open sequence (`{{-`, `{%-`, or `{#-`), or collapsed to a single space (` `) by appending `_` to the open tag (`{{_`, `{%_`, or `{#_`). The same is true for whitespace that comes after, but by prefixing the close sequence with the whitespace control characters (`-}}`, `-%}`, or `-#}` to remove; `_}}`, `_%}`, `_#}` to collapse).
{% endraw %}