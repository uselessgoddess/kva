#[cfg(feature = "full-fixtures")]
pub fn fixture() -> &'static str {
    include_str!("fixtures/items_game.txt")
}

#[cfg(not(feature = "full-fixtures"))]
pub fn fixture() -> &'static str {
    include_str!("fixtures/items_min.txt")
}
