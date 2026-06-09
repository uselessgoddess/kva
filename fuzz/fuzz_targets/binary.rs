#![no_main]

use kva::binary::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    let mut parser = Parser::new(input);
    let _ = parser.parse();
});
