pub fn chunk2u64(chunk: u128) -> Option<u64> {
    let a: [u8; 16] = chunk.to_be_bytes();
    let u: &[u8] = &a;
    let s: Option<&str> = std::str::from_utf8(u).ok();
    s.and_then(|hex| u64::from_str_radix(hex, 16).ok())
}

/// Converts hex encoded str bytes into the buf.
///
/// This function assumes hex.len() % 16 === 0.
pub fn hex_str_bytes2buf(hex: &[u8], buf: &mut Vec<u8>) -> usize {
    buf.clear();
    let chunks = hex.chunks_exact(16);
    let u7s = chunks.flat_map(|s: &[u8]| s.try_into().ok().map(u128::from_be_bytes));
    let u6s = u7s.flat_map(chunk2u64);
    let mapd = u6s.map(u64::to_be_bytes);
    mapd.for_each(|u: [u8; 8]| {
        let s: &[u8] = &u;
        buf.extend(s);
    });
    buf.len()
}
