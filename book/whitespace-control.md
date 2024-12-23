# Whitespace control

Whitespace (spaces, tabs, newlines, etc) that comes before a **tag** can be removed by appending `-` to the open sequence (`{{-`, `{%-`, or `{#-`), or collapsed to a single space (` `) by appending `_` to the open tag (`{{_`, `{%_`, or `{#_`). The same is true for whitespace that comes after, but by prefixing the close sequence with the whitespace control characters (`-}}`, `-%}`, or `-#}` to remove; `_}}`, `_%}`, `_#}` to collapse).
