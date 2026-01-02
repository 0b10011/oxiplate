# Statements

[Default escaper statements](default-escaper.md) sets or replaces the default escaper for a template.

```oxip:
{% default_escaper_group NAME %}
```

```oxip:
{% replace_escaper_group NAME %}
```

[Extends statements](extends.md) extend a template with the option to prepend, replace, append, or surround any `block` from the parent template.

```oxip:
{% extends PATH %}
{% block NAME %}
    CONTENT
{% parent %}
    CONTENT
{% endblock %}
```

[Include statements](include.md) include the contents of a template built using the variables from the current scope.

```oxip:
{% include PATH %}
```

[If statements](if-else.md) add branching to templates with `if`, `elseif`, and `else`.

```oxip:
{% if [let PATTERN =] EXPRESSION %}
    CONTENT
{% elseif [let PATTERN =] EXPRESSION %}
    CONTENT
{% else %}
    CONTENT
{% endif %}
```

[Match statements](match.md) add pattern matching with `match` and `case`.

```oxip:
{% match EXPRESSION %}
{% case PATTERN %}
    ...
{% endmatch %}
```

[For statements](for.md) bring iteration to templates with `for` and `else`.

```oxip:
{% for PATTERN in EXPRESSION %}
    {% if EXPRESSION %}
        {% continue %}
    {% elseif EXPRESSION %}
        {% break %}
    {% endif %}

    CONTENT
{% endfor %}
```

[Let statements](let.md) assign the result of an expression to a variable.

```oxip:
{% let NAME = EXPRESSION %}
```
