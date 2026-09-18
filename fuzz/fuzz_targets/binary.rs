#![no_main]

use kva::binary::{Dialect, Parser};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    for dialect in [Dialect::Vdf, Dialect::Source] {
        let _ = Parser::new(input).dialect(dialect).parse();
    }
});
