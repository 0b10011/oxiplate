# Configuration

Oxiplate uses a file named `oxiplate.toml` to store configuration settings.
The file must be located in your crate's manifest directory, alongside `Cargo.toml`.

If the file is missing or cannot be read, Oxiplate uses its default configuration.

The configuration file controls how escaper groups are selected, whether they must be specified explicitly, and which renderer implementation is used.
It can also define custom escaper groups.

## `fallback_escaper_group`

The `fallback_escaper_group` setting specifies the escaper group to use when Oxiplate cannot determine a group from the template or its file extension.

If an escaper group is not specified in a [writ](templates/writs/index.html),
the [`oxiplate_inline` attribute](templates/writs/escaping.md#escaping-templates-without-matching-file-extensions),
or the [template](templates/statements/default-escaper-group.md) that contains it,
and also cannot be inferred from that template's file extension,
the group specified in `fallback_escaper_group` is used instead.
If no fallback escaper group is set in these cases and a writ is not prefixed with `raw:`, a compile-time error will occur.

For example, to use `html` as the fallback escaper group:

```toml
fallback_escaper_group = "html"
```

<div class="warning">

The specified fallback group must be defined as one of the groups in `escaper_groups` or provided by Oxiplate with the `built-in-escapers` feature.

</div>

## `require_specifying_escaper`

The `require_specifying_escaper` setting controls whether each writ must explicitly specify its escaper.

When this setting is `true`, writs must specify the escaper group and escaper to use.
When it is `false`, Oxiplate can use the applicable default escaper group.

The default value is `false`.

For example, to require every writ to specify its escaper explicitly:

```toml
require_specifying_escaper = true
```

## `infer_escaper_group_from_file_extension`

The `infer_escaper_group_from_file_extension` setting controls whether Oxiplate infers an escaper group from a template's file extension.

When enabled, Oxiplate uses the file extension to determine the escaper group when one has not been specified elsewhere.
This can reduce the need to configure an escaper group explicitly for every template.

The default value is `true`.

The `.oxip` extension is used for Oxiplate templates and does not, by itself, identify an escaper group.
The `.raw` extension is ignored when inferring, so it never selects an escaper group.

When this setting is `false`, Oxiplate does not infer escaper groups from file extensions.
If no escaper group can be determined through another applicable setting or declaration, a compile-time error may occur.

For example, to disable inference from file extensions:

```toml
infer_escaper_group_from_file_extension = false
```

## `optimized_renderer`

The `optimized_renderer` setting controls whether Oxiplate uses its optimized renderer implementation.

When enabled, Oxiplate uses the optimized rendering path where supported.
When disabled, it uses the regular renderer.

By default, the optimized renderer is enabled when the internal `_oxiplate` feature is enabled, which is the case when using the main `oxiplate` crate.
Otherwise, the default is `false`.

For example, to enable the optimized renderer explicitly:

```toml
optimized_renderer = true
```

<div class="warning">

Setting `optimized_renderer = true` when using `oxiplate-derive` directly causes a compile-time error.
The optimized renderer is supported when using the main `oxiplate` crate.

</div>

## `escaper_groups`

The `escaper_groups` setting allows you to define custom escaper groups.
Each group specifies the Rust type that implements the escaping behavior.

The path to the escaper type must start with `::` so that Oxiplate can resolve it as an absolute path.

For example, to define a custom escaper group named `your_group`:

```toml
[escaper_groups.your_group]
escaper = "::your_crate::YourEscaper"
```

The specified type must implement the [`oxiplate_traits::Escaper` trait](https://github.com/0b10011/oxiplate/blob/main/oxiplate-traits/src/escaper.rs).

## Related documentation

- [Escaping templates](templates/writs/escaping.md)
- [Default escaper group statement](templates/statements/default-escaper-group.md)
