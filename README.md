# kva

A small parser for Valve KeyValues text and binary data.

The crate keeps the default parser path dependency-free. Text and binary parsing
are enabled by default and can be disabled independently. Serde format support is
available behind the `serde` feature.

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
