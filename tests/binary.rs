use kva::{Error, binary::Parser};

#[test]
fn binary_compound() {
    let mut data = Vec::new();
    data.push(0);
    data.extend_from_slice(b"root\0");
    data.push(1);
    data.extend_from_slice(b"name\0value\0");
    data.push(2);
    data.extend_from_slice(b"count\0");
    data.extend_from_slice(&42_i32.to_le_bytes());
    data.push(3);
    data.extend_from_slice(b"scale\0");
    data.extend_from_slice(&1.5_f32.to_le_bytes());
    data.push(7);
    data.extend_from_slice(b"wide\0");
    data.extend_from_slice(&42_u64.to_le_bytes());
    data.push(8);

    let mut parser = Parser::new(&data);
    let root = parser.parse().unwrap().unwrap();

    assert_eq!(root.name, "root");
    assert_eq!(root.get_str("name"), Some("value"));
    assert_eq!(root.get_int("count"), Some(42));
    assert_eq!(root.get_float("scale"), Some(1.5));
    assert_eq!(root.get_uint64("wide"), Some(42));
}

#[test]
fn extended_binary_values() {
    let mut binary = Vec::new();
    binary.push(9);
    binary.extend_from_slice(b"blob\0");
    binary.extend_from_slice(&3_u32.to_le_bytes());
    binary.extend_from_slice(b"abc");

    let mut parser = Parser::new(&binary);
    let entry = parser.parse().unwrap().unwrap();

    assert_eq!(entry.name, "blob");
    assert_eq!(entry.data.as_binary(), Some(&b"abc"[..]));

    let mut int64 = Vec::new();
    int64.push(10);
    int64.extend_from_slice(b"value\0");
    int64.extend_from_slice(&(-42_i64).to_le_bytes());

    let mut parser = Parser::new(&int64);
    let entry = parser.parse().unwrap().unwrap();

    assert_eq!(entry.name, "value");
    assert_eq!(entry.data.as_int64(), Some(-42));
}

#[test]
fn wide_and_ptr() {
    let mut wide = Vec::new();
    wide.push(5);
    wide.extend_from_slice(b"text\0");
    wide.extend_from_slice(&2_u16.to_le_bytes());
    wide.extend_from_slice(&u16::from(b'h').to_le_bytes());
    wide.extend_from_slice(&u16::from(b'i').to_le_bytes());

    let mut parser = Parser::new(&wide);
    let entry = parser.parse().unwrap().unwrap();
    assert_eq!(entry.data.as_wide_string(), Some(&[104, 105][..]));

    let mut ptr = Vec::new();
    ptr.push(4);
    ptr.extend_from_slice(b"ptr\0");
    ptr.extend_from_slice(&31337_u32.to_le_bytes());

    let mut parser = Parser::new(&ptr);
    let entry = parser.parse().unwrap().unwrap();
    assert_eq!(entry.data.as_pointer(), Some(31337));
}

#[test]
fn truncated_instead_of_panic() {
    let mut parser = Parser::new(&[]);
    assert_eq!(parser.parse(), Err(Error::UnexpectedEof));

    let mut parser = Parser::new(&[1, b'n', b'a', b'm', b'e']);
    assert_eq!(parser.parse(), Err(Error::UnterminatedCString));

    let mut parser = Parser::new(&[255]);
    assert_eq!(parser.parse(), Err(Error::InvalidType(255)));

    let mut parser = Parser::new(&[11]);
    assert_eq!(parser.parse(), Err(Error::InvalidType(11)));
}

#[test]
fn end_marker_returns_none() {
    let mut parser = Parser::new(&[8]);
    assert_eq!(parser.parse().unwrap(), None);
}

#[test]
fn parse_fixture() {
    let fixture = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x35, 0x34, 0x30, 0x32, 0x39, 0x00, 0x02, 0x70, 0x61, 0x63,
        0x6b, 0x61, 0x67, 0x65, 0x69, 0x64, 0x00, 0x0d, 0xd3, 0x00, 0x00, 0x02, 0x62, 0x69, 0x6c,
        0x6c, 0x69, 0x6e, 0x67, 0x74, 0x79, 0x70, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x6c,
        0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x02, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x78,
        0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x00, 0x01, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x63, 0x72,
        0x6f, 0x73, 0x73, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x64, 0x69, 0x6e,
        0x67, 0x61, 0x6e, 0x64, 0x67, 0x69, 0x66, 0x74, 0x69, 0x6e, 0x67, 0x00, 0x66, 0x61, 0x6c,
        0x73, 0x65, 0x00, 0x08, 0x00, 0x61, 0x70, 0x70, 0x69, 0x64, 0x73, 0x00, 0x02, 0x30, 0x00,
        0xda, 0x02, 0x00, 0x00, 0x02, 0x31, 0x00, 0xe4, 0x02, 0x00, 0x00, 0x02, 0x32, 0x00, 0xe9,
        0x02, 0x00, 0x00, 0x08, 0x00, 0x64, 0x65, 0x70, 0x6f, 0x74, 0x69, 0x64, 0x73, 0x00, 0x02,
        0x30, 0x00, 0xdb, 0x02, 0x00, 0x00, 0x02, 0x31, 0x00, 0xdc, 0x02, 0x00, 0x00, 0x02, 0x32,
        0x00, 0xdd, 0x02, 0x00, 0x00, 0x02, 0x33, 0x00, 0xde, 0x02, 0x00, 0x00, 0x08, 0x00, 0x61,
        0x70, 0x70, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x00, 0x08, 0x08, 0x08,
    ];

    let data = &fixture[4..]; // skip steam header

    let mut parser = kva::binary::Parser::new(data);
    let root = parser.parse().unwrap().unwrap();

    assert_eq!(root.name, "54029");

    assert_eq!(root.get_int("packageid"), Some(54029));
    assert_eq!(root.get_int("billingtype"), Some(1));
    assert_eq!(root.get_int("status"), Some(0));

    let extended = root.get("extended").unwrap();
    assert_eq!(
        extended.get_str("allowcrossregiontradingandgifting"),
        Some("false")
    );

    let appids = root.get("appids").unwrap();
    assert_eq!(appids.get_int("0"), Some(730));
    assert_eq!(appids.get_int("1"), Some(740));
    assert_eq!(appids.get_int("2"), Some(745));

    let depotids = root.get("depotids").unwrap();
    assert_eq!(depotids.get_int("0"), Some(731));
    assert_eq!(depotids.get_int("3"), Some(734));

    let appitems = root.get("appitems").unwrap();
    assert!(appitems.is_empty());
}
