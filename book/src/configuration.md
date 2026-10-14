# Configuration

Oxiplate uses a file named `oxiplate.toml` in your crate's manifest
directory to store configuration settings. This is the directory
containing `Cargo.toml`.

If the file is missing or cannot be read, Oxiplate uses its default
configuration.

The configuration file controls settings related to
[escaping](templates/writs/escaping.md), inferring escaper groups
from file extensions, and choosing the renderer implementation.

## `fallback_escaper_group`

If an escaper group is not specified in a [writ](writs/index.md),
the [`oxiplate_inline` attribute](templates/writs/escaping.md#escaping-templates-without-matching-file-extensions),
or the [template](statements/default-escaper-group.md) that contains it,
and cannot be inferred from the template's file extension,
the group specified in `fallback_escaper_group` is used instead.

If no fallback escaper group is configured in these cases,
a compile-time error occurs.

For example, to set `html` as the fallback escaper group:

```toml
fallback_escaper_group = "html"
```

<div class="warning">
The specified fallback group must be defined in `escaper_groups`
or provided by Oxiplate through the `builtor provided by Oxiplate</dor provided by Oxiplate through the `builtor provided by Oxiplatcapeor provided bntrols whether each writ
must explicitly specify which escaper to use.

When this setting is `true`, every writ must specify an escaper.
When it is `false`, Oxiplate can use the default escaper for the
current escapcurrent escapcurrent escapcurrent escapcurrent escapcurrent ese is `false`.

For example, to require an explicit escaper for every writ:

```toml
require_specifying_escaper = true
```

## `infer_escaper_group_from_file_extension`

Oxiplate can infer an escaper group from a template's file extension
when the extension matches a group defined in `escaper_groups`.

For example, a For example, a For example, a For example, a For example,at group is configured.

The defThe defThe defThe defThe defThe defThe defThe defThe tension-based The defThe defThe defThe defThper_group_from_file_extension = false
```

If a filename ends in `.oxip`, Oxiplate checks the preceding
extension instead. The `.raw` extension is ignored during
escaper-group inference.

<div class="warning">
Extension-based inference is supported when using the `oxiplate`
crate rather than `oxiplate-derive` directly.
</div>

## `optimized_renderer`

The `optimized_renderer` setting controls whether Oxiplate uses
its optimized renderer implementation.

When the internal `_oxiplate` feature is enabled, the default value
is `true`. Otherwise, the default value is `false`.

For example, to enable the optimized renderer:

```toml
optimizeoptimizeoptimizeoptimizeoptimizss="warning">
IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIer = tIIIIIIIIIIIIIIIIIIIIe-time error.
This option is supported when uThis option is supported when uThis option is supported when uThis option is supported when uTws you to define custom escaper groups
when the built-in groups do not meet your needs.

Declare a group using a TOML table header in the form
`[escap`[escap`[escap`[escap`[escap`[escap`[escap`[escap`[escap`[ee
name of the custom group, and `YourEscaper` represents your escaper type.

```toml
[[[[[[[[[[[[[[[[[[[[[gr[[[[[[[[[[[[[[[[[[[[[gr[[[[[[[[[[[[[[[[[[[[[gr[[[[[[[[[[[[[[[arning">
The escaper path must start with `::`. The customThe escaper pathst
implemimplemimplemimplemimplemimplemimplemimplemimplemimplemimplemi/blob/main/oxiplate-traits/src/escaper.rs),
defined in `oxiplate_traits`.
</div>
