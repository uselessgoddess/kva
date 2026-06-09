mod util;

use std::collections::BTreeMap;

use kva::text::{Deserializer, Parser};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameInfo {
    first_valid_class: i32,
    last_valid_class: i32,
    first_valid_item_slot: i32,
    last_valid_item_slot: i32,
    num_item_presets: u32,
    max_num_stickers: u32,
    max_num_patches: u32,
}

#[derive(Debug, Deserialize)]
struct PaintKit {
    name: String,
    #[serde(default)]
    description_string: Option<String>,
    #[serde(default)]
    wear_default: Option<f32>,
    #[serde(default)]
    wear_remap_min: Option<f32>,
    #[serde(default)]
    wear_remap_max: Option<f32>,
    #[serde(default)]
    style: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct Rarity {
    value: u32,
    loc_key: String,
    #[serde(default)]
    next_rarity: Option<String>,
}

#[test]
fn deserialize_game_info() {
    let input = util::fixture();
    let root = Parser::new(input).parse().unwrap();

    let entry = root.get("game_info").expect("game_info present").clone();
    let info = GameInfo::deserialize(&mut Deserializer::new(entry)).unwrap();

    assert_eq!(info.first_valid_class, 2);
    assert_eq!(info.last_valid_class, 3);
    assert_eq!(info.first_valid_item_slot, 0);
    assert_eq!(info.last_valid_item_slot, 54);
    assert_eq!(info.num_item_presets, 4);
    assert_eq!(info.max_num_stickers, 5);
    assert_eq!(info.max_num_patches, 3);
}

#[test]
fn deserialize_rarities_map() {
    let input = util::fixture();
    let root = Parser::new(input).parse().unwrap();

    let entry = root.get("rarities").expect("rarities present").clone();
    let rarities: BTreeMap<String, Rarity> =
        BTreeMap::deserialize(&mut Deserializer::new(entry)).unwrap();

    assert_eq!(rarities["default"].value, 0);
    assert_eq!(rarities["default"].loc_key, "Rarity_Default");
    assert_eq!(rarities["common"].value, 1);
    assert_eq!(rarities["common"].next_rarity.as_deref(), Some("uncommon"));
}

#[test]
fn merge_repeated_paint_kits() {
    let input = util::fixture();
    let root = Parser::new(input).parse().unwrap();

    let blocks = root.find_all_keys("paint_kits").count();
    assert!(blocks > 1, "expected repeated paint_kits, got {blocks}");

    let merged = root.get_all("paint_kits");
    let single = root.get("paint_kits").unwrap().iter().count();
    let total = merged.iter().count();
    assert!(
        total > single,
        "merge should gather more entries ({total}) than one block ({single})"
    );

    let kits: BTreeMap<u32, PaintKit> =
        BTreeMap::deserialize(&mut Deserializer::new(merged)).unwrap();

    let default = kits.get(&0).expect("paint kit 0 present");
    assert_eq!(default.name, "default");
    assert_eq!(
        default.description_string.as_deref(),
        Some("#PaintKit_Default")
    );
    assert_eq!(default.wear_default, Some(0.1));
    assert_eq!(default.wear_remap_min, Some(0.06));
    assert_eq!(default.wear_remap_max, Some(0.8));
    assert_eq!(default.style, Some(0));
}
