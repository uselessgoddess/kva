mod util;

use kva::{KvData, text::Parser};
use proptest::prelude::*;

#[test]
fn parses_source_style_text() {
    let input = r#"
// leading comment
"root"
{
    bare unquoted
    "quoted { key }" "quoted value"
}
"#;

    let mut parser = Parser::new(input);
    let root = parser.parse().unwrap();

    assert_eq!(root.name, "root");
    assert_eq!(root.get_str("bare"), Some("unquoted"));
    assert_eq!(root.get_str("quoted { key }"), Some("quoted value"));
}

#[test]
fn escape_sequences_are_opt_in() {
    let input = r#""root" { "path" "C:\Games\Steam" }"#;
    let mut parser = Parser::new(input);
    let root = parser.parse().unwrap();
    assert_eq!(root.get_str("path"), Some(r"C:\Games\Steam"));

    let input = r#""root" { "quote" "a\"b" "line" "a\nb" }"#;
    let mut parser = Parser::with_escape_sequences(input);
    let root = parser.parse().unwrap();
    assert_eq!(root.get_str("quote"), Some("a\"b"));
    assert_eq!(root.get_str("line"), Some("a\nb"));
}

#[test]
fn infers_source_numeric_values() {
    let input = r#"
"root"
{
    "int" "42"
    "float" "1e2"
    "uint64" "0x000000000000002a"
    "big_integer_string" "999999999999"
}
"#;

    let mut parser = Parser::new(input);
    let root = parser.parse().unwrap();

    assert_eq!(root.get_int("int"), Some(42));
    assert_eq!(root.get_float("float"), Some(100.0));
    assert_eq!(root.get_uint64("uint64"), Some(42));
    assert_eq!(root.get_str("big_integer_string"), Some("999999999999"));
}

#[test]
fn skip_conditionals() {
    let input = r#"
"root"
{
    "before_value" [$WIN32] "yes"
    "after_value" "yes" [$LINUX]
}
"#;

    let mut parser = Parser::new(input);
    let root = parser.parse().unwrap();

    assert_eq!(root.get_str("before_value"), Some("yes"));
    assert_eq!(root.get_str("after_value"), Some("yes"));
}

#[test]
fn items_game() {
    let input = util::fixture();
    let mut parser = Parser::new(input);
    let root = parser.parse().unwrap();

    assert_eq!(root.name, "items_game");
    assert!(matches!(
        root.get("game_info").map(|entry| &entry.data),
        Some(KvData::Compound(_))
    ));
}

proptest! {
    #[test]
    fn parser_does_not_panic(input in ".*") {
        let mut parser = Parser::new(&input);
        let _ = parser.parse();
    }

    #[test]
    fn parses_generated_quoted_leaf(key in "[A-Za-z0-9_]{1,16}", value in "[A-Za-z_][A-Za-z_ .-]{0,31}") {
        let input = format!(r#""root" {{ "{key}" "{value}" }}"#);
        let mut parser = Parser::new(&input);
        let root = parser.parse().unwrap();
        prop_assert_eq!(root.get_str(&key), Some(value.as_str()));
    }
}
