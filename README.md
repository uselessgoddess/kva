# kva

A small parser for Valve KeyValues text and binary data.

The crate keeps the default parser path dependency-free. Text and binary parsing
are enabled by default and can be disabled independently. Serde format support is
available behind the `serde` feature.

## Serde

```rust
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Weapon {
    name: String,
    // flattened and `#[serde(untagged)]` values are always
    // seen as strings — collect them as `String`, or parse them yourself
    #[serde(flatten)]
    attrs: BTreeMap<String, String>,
}

let weapon: Weapon = kva::text::from_str(r#"
"weapon"
{
    "name"   "ak47"
    "damage" "36"
}
"#).unwrap();

assert_eq!(weapon.name, "ak47");
assert_eq!(weapon.attrs["damage"], "36");
```

## Binary dialects

Valve ships two binary writers whose tags diverge past `7`. `Parser::new` reads
Steam's binary VDF (`appinfo.vdf`, `shortcuts.vdf`, microtransaction payloads);
`Parser::source` reads Source's `KeyValues::WriteAsBinary`, which spends tags
`8`–`10` on compiled small ints and ends a compound with `11`:

```rust
use kva::binary::Parser;

let sheet = b"\x00store\x00\x00prices\x00\x08USD\x00\xc7\x0b\x0b";
let root = Parser::source(sheet).parse().unwrap().unwrap();

assert_eq!(root.get_path("prices").unwrap().get_int("USD"), Some(199));
```

`to_vec_dialect` writes either one back, compiling small ints into the tag the
way Source does.

## Escape sequences

Escape handling is opt-in so that literal backslashes (e.g. Windows paths like
`C:\Games\Steam`) survive by default. Enable it to decode values that embed
escaped quotes, such as Source HUD / localization strings:

```rust
use kva::text::Parser;

let root = Parser::with_escape_sequences(r#""k" "say \"hi\"""#).parse().unwrap();
assert_eq!(root.data.as_str(), Some(r#"say "hi""#));
```

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
