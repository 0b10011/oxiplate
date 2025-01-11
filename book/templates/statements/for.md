# Iterating with `for` and `else`

```oxip
<ul>
  {% for name in names %}
    <li>{{ name }}
  {% else %}
    <li><em>No names found</em>
  {% endfor %}
</ul>
```

Could produce something like:

```html
<ul>
  <li>Jasmine
  <li>Malachi
  <li>Imogen
</ul>
```

Or if `names` was empty:

```html
<ul>
  <li><em>No names found</em>
</ul>
```
