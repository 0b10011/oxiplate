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

## Replace parent contents

You can choose to replace the contents of the parent block with a block with the same name:

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

For the same effect, you can be explicit with `extends(replace)`:

```diff
  {# your-content.html.oxip -#}

- {% extends "layout.html.oxip" %}
+ {% extends(replace) "layout.html.oxip" %}

  {% block content %}
    <p>Replaced content.</p>
  {% endblock %}
```

## Prefix parent contents

To prefix the contents of the parent, you can use `extends(prefix)`:

```diff
  {# your-content.html.oxip -#}

- {% extends "layout.html.oxip" %}
+ {% extends(prefix) "layout.html.oxip" %}
+
+ {% block content %}
+   <p>Prefix.</p>
+ {% endblock %}
```

```diff
  <!DOCTYPE html>
  <main>
+   <p>Prefix.</p>
    <p>Parent content.</p>
  </main>
```

## Suffix parent contents

To suffix the contents of the parent, you can use `extends(suffix)`:

```diff
  {# your-content.html.oxip -#}

- {% extends "layout.html.oxip" %}
+ {% extends(suffix) "layout.html.oxip" %}
+
+ {% block content %}
+   <p>Suffix.</p>
+ {% endblock %}
```

```diff
  <!DOCTYPE html>
  <main>
    <p>Parent content.</p>
+   <p>Suffix.</p>
  </main>
```

## Surround parent contents

To surround the contents of the parent, you can use `extends(surround)` and `{% parent %}`:

```diff
  {# your-content.html.oxip -#}

- {% extends "layout.html.oxip" %}
+ {% extends(surround) "layout.html.oxip" %}
+
+ {% block content %}
+   <p>Prefix.</p>
+ {% parent %}
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
