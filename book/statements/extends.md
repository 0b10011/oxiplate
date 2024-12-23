# Extending templates

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