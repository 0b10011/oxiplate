# Extending templates with `extends` and `block`

Start with a template that contains one or more blocks:

```html.oxip:layout.html.oxip
<!DOCTYPE html>
<main>
{%- block content %}
  <p>Parent content.</p>
{% endblock -%}
</main>
```

Then create a template to extend it:

```html.oxip:your-content.html.oxip
{% extends "layout.html.oxip" %}
```

This is essentially the same as using `layout.html.oxip` directly:

```html
<!DOCTYPE html>
<main>
  <p>Parent content.</p>
</main>
```

## Named `endblock`

`endblock` tags can also include the name of the block
that it is supposed to be ending.

```html.oxip:layout.html.oxip
<!DOCTYPE html>
<main>
{%- block content %}
  <p>Parent content.</p>
{% endblock content -%}
</main>
```

This can help in situations where an `endblock` tag
gets added or removed accidentally:

```rust:/oxiplate-derive/tests/broken/tests/extends/endblock-mismatch.rs
{{#include ../../../../oxiplate-derive/tests/broken/tests/extends/endblock-mismatch.rs:3:16}}
```

```stderr:/oxiplate-derive/tests/broken/tests/extends/endblock-mismatch.stderr
{{#include ../../../../oxiplate-derive/tests/broken/tests/extends/endblock-mismatch.stderr}}
```

## Adding to or replacing block content

Anything you add to a block of the same name in a child template
will replace the content of the parent block:

```diff:your-content.html.oxip
  {% extends "layout.html.oxip" %}

+ {% block content %}
+   <p>Replaced content.</p>
+ {% endblock %}
```

```diff
  <!DOCTYPE html>
  <main>
-   <p>Parent content.</p>
+   <p>Replaced content.</p>
  </main>
```

The parent's content can be kept by using the `{% parent %}` tag in the block:

```diff:your-content.html.oxip
  {% extends "layout.html.oxip" %}+

+ {% block(surround) content %}
+   <p>Prefix.</p>
+   {% parent %}
+   <p>Suffix.</p>
+ {% endblock %}
```

```diff
  <!DOCTYPE html>
  <main>
+   <p>Prefix.</p>
    <p>Parent content.</p>
+   <p>Suffix.</p>
  </main>
```
