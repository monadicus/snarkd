use anyhow::Result;
use bip_bencode::{BDecodeOpt, BRefAccess, BencodeRef, BencodeRefKind};

/// Converts bencoded bytes to JSON string
pub fn bencode_bytes_to_json(bytes: &[u8]) -> Result<String> {
    let decoded =
        BencodeRef::decode(&bytes, BDecodeOpt::default()).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    Ok(bencode_to_json(&decoded))
}

/// Converts decoded bencode data into JSON string
pub fn bencode_to_json(decoded: &BencodeRef) -> String {
    match decoded.kind() {
        // ints are... ints
        BencodeRefKind::Int(n) => format!("{}", n),

        // bytes are displayed as an array
        BencodeRefKind::Bytes(n) => format!(
            "[{}]",
            n.iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(",")
        ),
        BencodeRefKind::List(n) => format!(
            "[{}]",
            // json stringify all items in the list
            n.into_iter()
                .map(|r| bencode_to_json(r))
                .collect::<Vec<String>>()
                .join(",")
        ),
        BencodeRefKind::Dict(n) => format!(
            "{{{}}}",
            n.to_list()
                .iter()
                .map(|(&k, v)| format!(
                    "\"{}\": {}",
                    // For keys, try utf-8 strings. fallback to hex (infohash)
                    std::str::from_utf8(k).unwrap_or(&hex::encode_upper(k)),
                    // Values are bencoded
                    bencode_to_json(v)
                ))
                .collect::<Vec<String>>()
                .join(",")
        ),
    }
}
