# Oxiplate: Template engine for Rust

**Oxiplate is experimental and features described here may not yet be implemented, or may be implemented in a different way.**

## Template syntax

The syntax for templates is similar to many other systems, but the terminology may be slightly different.

A **writ** is an [**expression**](#expressions) wrapped with `{{` and `}}` that will be evaluated and output into the template. For example, `Hello {{ name }}!` may become `Hello Luna!`.

A [**statement**](#statements) is wrapped with `{%` and `%}` and includes variable assignments and control structures. See the [**statement**](#statements) section for a list of possible statements.

A **comment** is text wrapped with `{#` and `#}` that will be removed from the final template, but can be useful to the template designer(s). For example, `{# None of this text would show up in the final template. #}`.

Whitespace before and after **tags** can be removed or collapsed. See the [whitespace control](#whitespace-control) section for more information.

Anything else in the template is considered **static** and will be left as-is.
