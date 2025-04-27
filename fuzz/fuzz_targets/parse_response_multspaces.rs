#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut resp = httparse::Response::new(&mut headers);
    let _ = httparse::ParserConfig::default()
        .allow_spaces_after_header_name_in_responses(true)
        .allow_multiple_spaces_in_response_status_delimiters(true)
        .allow_obsolete_multiline_headers_in_responses(true)
        .allow_space_before_first_header_name(true)
        .parse_response(&mut resp, data);
});
