# Extending templates with `extends` and `block`

Start with a template that contains one or more blocks:

```oxip
{# layout.html.oxip -#}

<!DOCTYPE html>
<main>
{%- block content %}
  <p>Parent content.</p>
{% endblock -%}
</main>
```

Then create a template to extend it:

```oxip
{# your-content.html.oxip -#}

{% extends "layout.html.oxip" %}
```

This is essentially the same as using `layout.html.oxip` directly:

```html
<!DOCTYPE html>
<main>
  <p>Parent content.</p>
</main>
```

## Adding to or replacing block content

Anything you add to a block of the same name in a child template
will replace the content of the parent block:

```diff
  {# your-content.html.oxip -#}

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

```diff
  {# your-content.html.oxip -#}

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
