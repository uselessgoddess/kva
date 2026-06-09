use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use kva::text::Parser;
use valve_keyvalue::{Parse, ValveKeyValue};
use vdf_reader::entry::Table as VdfTable;

#[cfg(feature = "full-fixtures")]
fn items_game() -> &'static str {
    include_str!("../tests/fixtures/items_game.txt")
}

#[cfg(not(feature = "full-fixtures"))]
fn items_game() -> &'static str {
    include_str!("../tests/fixtures/items_min.txt")
}

fn text_items_game(c: &mut Criterion) {
    let input = items_game().to_owned();
    let keyvalues_parser = keyvalues_parser::Parser::new().literal_special_chars(true);

    let mut group = c.benchmark_group("text_items_game");
    group.bench_function("kva", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(input.as_str()));
            black_box(parser.parse()).unwrap()
        });
    });
    group.bench_function("keyvalues_parser", |b| {
        b.iter(|| black_box(keyvalues_parser.parse(black_box(input.as_str()))).unwrap());
    });
    group.bench_function("valve_keyvalue", |b| {
        b.iter(|| black_box(ValveKeyValue::parse(black_box(input.clone()), false)).unwrap());
    });
    group.bench_function("vdf_reader", |b| {
        b.iter(|| black_box(VdfTable::load_from_str(black_box(input.as_str()))).unwrap());
    });
    group.finish();
}

criterion_group!(benches, text_items_game);
criterion_main!(benches);
