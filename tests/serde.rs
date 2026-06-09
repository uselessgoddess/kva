use std::collections::BTreeMap;

use kva::{binary, text};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct BorrowedConfig<'a> {
    name: &'a str,
    code: &'a str,
    count: i32,
    enabled: bool,
    id: u64,
    tags: Vec<&'a str>,
    attrs: BTreeMap<String, i32>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct OwnedConfig {
    name: String,
    count: i32,
    enabled: bool,
    id: u64,
    tags: Vec<String>,
}

#[test]
fn keep_numeric_strings() {
    use text::{Deserializer, Parser};

    let input = r#"
"weapon"
{
    "name" "ak47"
    "code" "42"
    "count" "30"
    "enabled" "1"
    "id" "18446744073709551615"
    "tags"
    {
        "0" "rifle"
        "1" "primary"
    }
    "attrs"
    {
        "damage" "36"
        "price" "2700"
    }
}
"#;

    let parsed = Parser::new(input).numeric_inference(false).parse().unwrap();

    let value = BorrowedConfig::deserialize(&mut Deserializer::new(parsed)).unwrap();

    assert_eq!(value.name, "ak47");
    assert_eq!(value.code, "42");
    assert_eq!(value.count, 30);
    assert!(value.enabled);
    assert_eq!(value.id, u64::MAX);
    assert_eq!(value.tags, ["rifle", "primary"]);
    assert_eq!(value.attrs["damage"], 36);
}

#[test]
fn text_roundtrip() {
    use text::{Options, Serializer};

    let mut attrs = BTreeMap::new();
    attrs.insert("damage".to_owned(), 36);
    attrs.insert("price".to_owned(), 2700);

    let value = BorrowedConfig {
        name: "ak47",
        code: "42",
        count: 30,
        enabled: true,
        id: u64::MAX,
        tags: vec!["rifle", "primary"],
        attrs,
    };

    let text = value.serialize(Serializer::new("weapon")).unwrap();
    let decoded: BorrowedConfig = text::from_str_options(
        &text,
        Options {
            escape_sequences: false,
            numeric_inference: false,
        },
    )
    .unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn binary_roundtrip() {
    use binary::Serializer;

    let value = OwnedConfig {
        name: "ak47".to_owned(),
        count: 30,
        enabled: true,
        id: u64::MAX,
        tags: vec!["rifle".to_owned(), "primary".to_owned()],
    };

    let bytes = value.serialize(Serializer::new("weapon")).unwrap();
    let decoded: OwnedConfig = binary::from_slice(&bytes).unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn serde_entry_points() {
    use text::{Deserializer, Parser};

    let input = r#""weapon" { "name" "ak47" "count" "30" "enabled" "true" "id" "42" "tags" { "0" "rifle" } }"#;
    let parsed = Parser::new(input).parse().unwrap();

    let mut deserializer = Deserializer::new(parsed);
    let decoded = OwnedConfig::deserialize(&mut deserializer).unwrap();

    assert_eq!(decoded.name, "ak47");
    assert_eq!(decoded.tags, ["rifle"]);
}
