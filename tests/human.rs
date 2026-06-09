use std::net::IpAddr;

use kva::binary;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Debug, PartialEq)]
struct ReadableChecker;

impl Serialize for ReadableChecker {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str("human")
        } else {
            serializer.serialize_bytes(b"machine")
        }
    }
}

impl<'de> Deserialize<'de> for ReadableChecker {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ReadableChecker;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("string or bytes depending on is_human_readable")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                assert_eq!(v, "human");
                Ok(ReadableChecker)
            }

            fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
                assert_eq!(v, "human");
                Ok(ReadableChecker)
            }

            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                assert_eq!(v, b"machine");
                Ok(ReadableChecker)
            }

            fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                assert_eq!(v, b"machine");
                Ok(ReadableChecker)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CheckerConfig {
    flag: ReadableChecker,
}

#[test]
fn text_true() {
    let config = CheckerConfig {
        flag: ReadableChecker,
    };

    let text = kva::text::to_string("root", &config).unwrap();
    assert_eq!(text, "\"root\"\n{\n\t\"flag\"\t\"human\"\n}\n");

    let decoded: CheckerConfig = kva::text::from_str(&text).unwrap();
    assert_eq!(decoded, config);
}

#[test]
fn binary_false() {
    let config = CheckerConfig {
        flag: ReadableChecker,
    };

    let bytes = kva::binary::to_vec("root", &config).unwrap();
    assert!(bytes.windows(7).any(|w| w == b"machine"));

    let decoded: CheckerConfig = kva::binary::from_slice(&bytes).unwrap();
    assert_eq!(decoded, config);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NetworkConfig {
    single_ip: IpAddr,
    nested_ips: Vec<IpAddr>,
}

#[test]
fn nested_types() {
    let config = NetworkConfig {
        single_ip: "127.0.0.1".parse().unwrap(),
        nested_ips: vec!["192.168.0.1".parse().unwrap(), "10.0.0.1".parse().unwrap()],
    };

    let bytes = kva::binary::to_vec("net", &config).unwrap();

    let string_repr = b"192.168.0.1";
    assert!(!bytes.windows(string_repr.len()).any(|w| w == string_repr));

    let decoded: NetworkConfig = binary::from_slice(&bytes).unwrap();
    assert_eq!(decoded, config);
}

#[test]
fn ipaddr_human() {
    let config = NetworkConfig {
        single_ip: "127.0.0.1".parse().unwrap(),
        nested_ips: vec!["192.168.0.1".parse().unwrap()],
    };

    let text = kva::text::to_string("net", &config).unwrap();

    assert!(text.contains("\"127.0.0.1\""));
    assert!(text.contains("\"192.168.0.1\""));

    let decoded: NetworkConfig = kva::text::from_str(&text).unwrap();
    assert_eq!(decoded, config);
}
