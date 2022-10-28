/// Given a 40 length hash (20 byte info_hash), URI encode it (%AA%BB%CC...)
pub fn uri_encode_hash(hash: &[u8]) -> Option<String> {
    if hash.len() != 40 {
        None
    } else {
        Some(
            hash.chunks(2)
                .map(|chunk| format!("%{}", std::str::from_utf8(chunk).unwrap()))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
