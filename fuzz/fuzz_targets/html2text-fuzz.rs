#![no_main]

use libfuzzer_sys::fuzz_target;
use html2text::*;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if data.len() > 2 {
        let width: usize = usize::from(data[0]);
        from_read(&(data[1..]), width);
    }
});
