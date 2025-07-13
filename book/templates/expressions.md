# Expressions

Expressions can be a lone literal like `"A string."`
or a more complicated calculation or comparison.
While expressions _often_ evaluate to strings for output in writs,
they can also be mathematical equations and comparisons for branching logic.

## Literals

Oxiplate supports many of the same literals Rust itself does:

- String (e.g., `"This is a string."`)
- Boolean (i.e., `true` or `false`)
- Integer (e.g., `19`)

With improved support planned:

- Float ([#22](https://github.com/0b10011/oxiplate/issues/22); e.g., `1.9e1`)
- Binary ([#19](https://github.com/0b10011/oxiplate/issues/19); e.g., `0b10011`)
- Octal ([#20](https://github.com/0b10011/oxiplate/issues/20); e.g., `0o23`)
- Hexadecimal ([#21](https://github.com/0b10011/oxiplate/issues/21); e.g., `0x13`)
- Underscore number separators ([#18](https://github.com/0b10011/oxiplate/issues/18); e.g., `1_000_000`)

## Variables, fields, and functions

<div class="warning">

Variables cannot be named `self` or `super`.

</div>

Oxiplate accesses variables, fields, and functions similarly to Rust:

```oxip
{{ foo }}
{{ foo.bar }}
{{ foo.hello() }}
```

All data available to templates is stored in the struct
that referenced the template,
or within the template itself.
Local variables override those set for the template.
Therefore, `self.` is neither needed nor allowed;
it will be implied when a local variable of the same name doesn't exist.

## Filters

<div class="warning">

Filters are not yet implemented ([#26](https://github.com/0b10011/oxiplate/issues/26)).

</div>

Filters modify expressions that precede them:

```oxip
{{ "foo"|upper }}
```

> FOO

Behind the scenes, filters are functions that are passed the result of the expression as the first argument. Additional arguments can be passed to the filter directly:

```oxip
{{ "hello world"|replace("hello", "goodbye") }}
```

> goodbye world

## Operators

Unless otherwise specified, the operators behave the same as [they do in Rust](https://doc.rust-lang.org/book/appendix-02-operators.html).

Math:

- `+`
- `-`
- `*`
- `/`
- `%`

Comparison:

- `==`
- `!=`
- `>`
- `<`
- `>=`
- `<=`

Logic:

- `||`
- `&&`

Other:

- `~`: Concatenate the left and right sides into a single string.
