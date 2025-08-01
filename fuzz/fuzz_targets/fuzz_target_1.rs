#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code: try to parse as UTF-8
    if let Ok(s) = std::str::from_utf8(data) {
        // Use s if needed for further fuzzing logic
    }
});
