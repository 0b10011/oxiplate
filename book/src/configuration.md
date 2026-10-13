# Configuration

## Configuration file
Oxiplate uses a file named `oxiplate.toml` to store configuration settings.

It looks for this file in the crate's manifest directory, unless `CARGO_MANIFEST_DIR_OVERRIDE` specifies a different directory.
If the file is missing or cannot be read, Oxiplate uses its default configuration.
The configuration file controls settings related to escaping, inferring escaper groups from file extensions, and choosing the renderer implementation.


## fallback_escaper_group
Purpose: If Oxiplate cannot determine the escaper group from a template's file extension, this setting specifies which group it should use as a fallback.
Default value: None, meaning no fallback group is configured by default.
Important: The `html` group must be defined in the `escaper_groups` configuration.

Example:
```toml 
fallback_escaper_group = "html"
```
This sets `html` as the fallback escaper group.

## require_specifying_escaper
Purpose: Controls whether you must explicitly specify an escaper for each writ in a template.
When true: Every writ must explicitly specify which escaper to use.
When false: Oxiplate can use the default escaper for the current escaper group if one isn't explicitly specified.
Default value: false.

Example:
```toml 
require_specifying_escaper = true
```


## infer_escaper_group_from_file_extension
Purpose: Oxiplate can use a template's file extension to figure out which escaper group to use.
How it works: If the extension matches a group defined in `escaper_groups`, Oxiplate can infer that group. For example, a template ending in `.html` can use the `html` group if it is configured.
Default value: true, so extension-based inference is enabled by default.
When false: Oxiplate stops trying to infer the escaper group from the file extension.
Special handling for `.oxip`: If the filename ends in `.oxip`, Oxiplate checks the preceding extension instead.
Special handling for `.raw`: The `raw` extension is ignored during escaper-group inference.

Important restriction: This inference only works when you use `oxiplate` rather than `oxiplate-derive` directly.

Example:
```toml 
infer_escaper_group_from_file_extension = false
```

## optimized_renderer
Purpose: Controls whether Oxiplate uses its alternate, optimized renderer implementation.
When true: Enables the optimized renderer.
When false: Disables the optimized renderer.
Default value: Depends on the build configuration:
When the internal `_oxiplate` feature is enabled, the default is true.
When that feature is not enabled, the default is false.

Important restriction: If you use `oxiplate-derive` directly, setting `optimized_renderer = true` causes a compile-time error. This option is supported when using the main oxiplate crate.

Example:
```toml 
optimized_renderer = true
```


## escaper_groups
Purpose: Allows users to define custom escaper groups when the built-in groups don't meet their needs.
Declaring a group: Use a TOML table header in the format `[escaper_groups.group_name]`. In the example, `your_group` is the name of the custom group.

Path requirement: The escaper path must start with `::`.
Required trait: The custom escaper type must implement the `oxiplate_traits::Escaper` trait. A Rust trait defines the behavior that a type must provide.

Example:
```toml 
[escaper_groups.your_group] 
escaper = "::your_crate::YourEscaper"
```


## Related documentation
[Escaping](templates/writs/escaping.md) explains how escaping works in Oxiplate, including the `fallback_escaper_group` and require_specifying_escaper settings.