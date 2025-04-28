#![no_main]
use libfuzzer_sys::fuzz_target;
use std::mem::MaybeUninit;

fuzz_target!(|data: &[u8]| {
    let mut headers: Vec<MaybeUninit<httparse::Header<'_>>> = vec![MaybeUninit::uninit(); 16];
    let mut resp = httparse::Response::new(&mut []);
    let _ = httparse::ParserConfig::default().parse_response_with_uninit_headers(
        &mut resp,
        data,
        &mut headers,
    );
});
