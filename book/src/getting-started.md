# Getting started

Include [Oxiplate](https://crates.io/crates/oxiplate) in your project:

```bash
cargo add oxiplate
```

Create a couple templates in the `/template` directory:

```html:/templates/layout.html.oxip
<!DOCTYPE html>
<html>
    <head>
        <title>{{ title }} - {{ site_name }}</title>
    </head>
    <body>
        <header>
            <h1>{{ site_name }}</h1>
        </header>
        <main>
            {% block content %}{% endblock %}
        </main>
    </body>
</html>
```

```html:/templates/index.html.oxip
{% extends "layout.html.oxip" %}

{% block content %}
    <h1>{{ title }}</h1>
    <p>{{ message }}</p>
{% endblock %}
```

Build the template and output it:

```rust:/src/main.rs
use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "index.html.oxip"]
struct Homepage {
    site_name: &'static str,
    title: &'static str,
    message: &'static str,
}

fn main() {
    let template = Homepage {
        site_name: "Oxiplate Documentation"
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    print!("{}", template);
}
```

Which should output something like:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>Oxiplate Example - Oxiplate Documentation</title>
    </head>
    <body>
        <header>
            <h1>Oxiplate Documentation</h1>
        </header>
        <main>
    <h1>Oxiplate Example</h1>
    <p>Hello world!</p>
        </main>
    </body>
</html>
```
