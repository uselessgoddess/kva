use kva::text::Parser;
use kva::{KvData, binary};
use serde::Serialize;

#[derive(Serialize)]
struct Weapon {
    name: String,
    clip: i32,
    id: u64,
}

#[test]
fn binary_layout_is_byte_exact() {
    let weapon = Weapon {
        name: "ak47".into(),
        clip: 30,
        id: 0x2a,
    };

    let bytes = binary::to_vec("weapon", &weapon).unwrap();

    #[rustfmt::skip]
    let expected = [
        0x00, b'w', b'e', b'a', b'p', b'o', b'n', 0x00,     // compound "weapon"
        0x01, b'n', b'a', b'm', b'e', 0x00, b'a', b'k', b'4', b'7', 0x00, // string "name" = "ak47"
        0x02, b'c', b'l', b'i', b'p', 0x00, 0x1e, 0x00, 0x00, 0x00,       // int "clip" = 30
        0x07, b'i', b'd', 0x00, 0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // uint64 "id" = 42
        0x08,                                              // end of compound
    ];

    assert_eq!(bytes, expected);
}

#[test]
fn text_layout_is_byte_exact() {
    let weapon = Weapon {
        name: "ak47".into(),
        clip: 30,
        id: 0x2a,
    };

    let text = kva::text::to_string("weapon", &weapon).unwrap();

    let expected = "\"weapon\"\n{\n\t\"name\"\t\"ak47\"\n\t\"clip\"\t\"30\"\n\t\"id\"\t\"0x000000000000002A\"\n}\n";
    assert_eq!(text, expected);
}

fn infer(raw: &str) -> KvData<'static> {
    let input = format!("\"root\" {{ \"k\" \"{raw}\" }}");
    let root = Parser::new(&input).parse().unwrap();
    root.get("k").unwrap().data.clone().into_owned()
}

#[test]
fn num_infer_valve_rules() {
    assert_eq!(infer("42"), KvData::Int(42));
    assert_eq!(infer("-7"), KvData::Int(-7));
    assert_eq!(infer("+5"), KvData::Int(5));

    assert_eq!(infer("1.5"), KvData::Float(1.5));
    assert_eq!(infer("1e2"), KvData::Float(100.0));
    assert_eq!(infer("0.100000"), KvData::Float(0.1));

    assert_eq!(infer("0x000000000000002a"), KvData::UInt64(0x2a));
    assert_eq!(infer("0xFFFFFFFFFFFFFFFF"), KvData::UInt64(u64::MAX));

    assert!(matches!(infer(""), KvData::String(s) if s.is_empty()));
    assert!(matches!(infer("999999999999"), KvData::String(_)));
    assert!(matches!(infer("0x1234"), KvData::String(_)));

    assert!(matches!(infer("1.2.3"), KvData::String(_)));
    assert!(matches!(infer("12abc"), KvData::String(_)));
    assert!(matches!(infer("inf"), KvData::String(_)));
    assert!(matches!(infer("nan"), KvData::String(_)));
}

#[test]
fn binary_roundtrips() {
    let weapon = Weapon {
        name: "ak47".into(),
        clip: 30,
        id: 0x2a,
    };

    let bytes = binary::to_vec("weapon", &weapon).unwrap();
    let root = binary::Parser::new(&bytes).parse().unwrap().unwrap();

    assert_eq!(root.name, "weapon");
    assert_eq!(root.get_str("name"), Some("ak47"));
    assert_eq!(root.get_int("clip"), Some(30));
    assert_eq!(root.get_uint64("id"), Some(0x2a));
}
