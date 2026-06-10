use std::collections::BTreeMap;

use kva::text::{self, Options, Parser};
use serde::Deserialize;

// escaped quotes inside a quoted value.
const HUD_LINE: &str = r#"
"Resource/UI/HudSpectator.res"
{
    "HudSpecPlayer_WeaponName"        "<span class=\"possessive-player-name\">{s:possessive_player_name}</span><br><span class=\"weapon-name\">{s:weapon_name}</span><br><font color=\"{s:rarity_color}\"><span class=\"weapon-kit-name\">{s:weapon_kit_name}</span><br><span class=\"weapon-name-custom\">{s:weapon_name_custom}</span></font>"
}
"#;

#[test]
fn escaped_quotes() {
    let root = Parser::with_escape_sequences(HUD_LINE).parse().unwrap();
    let value = root.get_str("HudSpecPlayer_WeaponName").unwrap();

    let expected = concat!(
        r#"<span class="possessive-player-name">{s:possessive_player_name}</span><br>"#,
        r#"<span class="weapon-name">{s:weapon_name}</span><br>"#,
        r#"<font color="{s:rarity_color}">"#,
        r#"<span class="weapon-kit-name">{s:weapon_kit_name}</span><br>"#,
        r#"<span class="weapon-name-custom">{s:weapon_name_custom}</span></font>"#,
    );
    assert_eq!(value, expected);
}

#[test]
fn escapes_is_preserved() {
    let root = Parser::with_escape_sequences(r#""root" { "k" "a\"BCD\"e" }"#)
        .parse()
        .unwrap();
    assert_eq!(root.get_str("k"), Some(r#"a"BCD"e"#));
}

#[test]
fn escapes_remain_opt_in() {
    let root = Parser::new(r#""root" { "path" "C:\Games\Steam" }"#)
        .parse()
        .unwrap();
    assert_eq!(root.get_str("path"), Some(r#"C:\Games\Steam"#));
}

// #[serde(flatten)]`.
#[derive(Debug, PartialEq, Deserialize)]
struct Inner {
    b: String,
    c: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Outer {
    a: String,
    #[serde(flatten)]
    inner: Inner,
}

#[test]
fn flatten_into_struct() {
    let input = r#"
"root"
{
    "a" "aa"
    "b" "bb"
    "c" "cc"
}
"#;
    let decoded: Outer = text::from_str(input).unwrap();
    assert_eq!(
        decoded,
        Outer {
            a: "aa".into(),
            inner: Inner {
                b: "bb".into(),
                c: "cc".into(),
            },
        }
    );
}

#[derive(Debug, PartialEq, Deserialize)]
struct Collected {
    name: String,
    #[serde(flatten)]
    rest: BTreeMap<String, String>,
}

#[test]
fn flatten_into_catch_all_map() {
    let input = r#"
"weapon"
{
    "name"  "ak47"
    "count" "30"
    "clip"  "1"
}
"#;
    let decoded: Collected = text::from_str(input).unwrap();
    assert_eq!(decoded.name, "ak47");
    assert_eq!(decoded.rest.get("count").map(String::as_str), Some("30"));
    assert_eq!(decoded.rest.get("clip").map(String::as_str), Some("1"));
}

#[test]
fn independent_of_num_infer() {
    let input = r#"
"weapon"
{
    "name"  "ak47"
    "count" "30"
}
"#;
    let opts = Options {
        escape_sequences: false,
        numeric_inference: false,
    };
    let decoded: Collected = text::from_str_options(input, opts).unwrap();
    assert_eq!(decoded.rest.get("count").map(String::as_str), Some("30"));
}
