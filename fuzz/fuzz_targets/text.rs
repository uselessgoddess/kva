#![no_main]

use kva::text::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &str| {
    let mut parser = Parser::new(input);
    let _ = parser.parse();

    let mut parser = Parser::with_escape_sequences(input);
    let _ = parser.parse();
});
