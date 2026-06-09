# kva

A small parser for Valve KeyValues text and binary data.

The crate keeps the default parser path dependency-free. Text and binary parsing
are enabled by default and can be disabled independently. Serde format support is
available behind the `serde` feature.

## Features

- `std` *(default)* — enables `std`; disable it for `no_std`.
- `text` *(default)* — the text (`.vdf`) tokenizer/parser and writer.
- `binary` *(default)* — the binary KeyValues parser and writer.
- `serde` *(default)* — `serde` `Serialize`/`Deserialize` support.

## `no_std`

The crate is `no_std` + `alloc`. Disable default features to drop `std`:

```toml
[dependencies]
kva = { version = "0.1", default-features = false, features = ["text", "binary", "serde"] }
```

Fuzz targets live under `fuzz/` and can be run with `cargo fuzz`:

```sh
cargo fuzz run text
cargo fuzz run binary
```
