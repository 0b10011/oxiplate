# Setting the default escaper group from within a template

Normally, the default escaper group for a template
is inferred from the file's extension.

For cases where the file extension doesn't correlate to the contents of the file
(e.g., using `.oxip` instead of `.html.oxip`),
the default group can be set within a template:

```html:oxip
{% default_escaper_group html %}
<h1 title="{{ attr: title }}">{{ title }}</h1>
```

But when the escaper group is inferred incorrectly from the file extension
(e.g., `.tmpl` is set to `html` by default),
the default group has to be _replaced_ instead:

```json:html
{% replace_escaper_group json %}
{
    "greeting": "Hello {{ name }}!",
}
```
