//! Converts hex string bytes to bytes.
//!
//! ```
//! "00" -> 0x00(16 bits string -> 8 bits)
//! "01" -> 0x01
//! "02" -> 0x02
//! ...
//! "0F" -> 0x0F
//! ...
//! "FF" -> 0xFF
//! ```
//!
//! ```
//! "0123" -> 0x0123(32 bits string -> 16 bits)
//! "01234567" -> 0x01234567(64 bits string -> 32 bits)
//! "0123456789abcdef" -> 0x0123456789abcdef(128 bits string -> 64 bits)
//! ```

/// Converts a hex str(e.g, `"ff"`) to a u8(e.g, `0xff`).
pub fn hex_str2byte_std(s: &str) -> Option<u8> {
    u8::from_str_radix(s, 16).ok()
}

/// Converts a hex bytes pair(u16) to a u8.
pub fn u16str2byte_std(u4: u16) -> Option<u8> {
    let b: [u8; 2] = u4.to_be_bytes();
    let s: &[u8] = &b;
    let o: Option<&str> = std::str::from_utf8(s).ok();
    o.and_then(hex_str2byte_std)
}

/// Converts a hex bytes(`&[u8]`) into a bytes buffer(`&mut Vec<u8>`).
pub fn hex_bytes2bytes_std(hex: &[u8], dst: &mut Vec<u8>) -> usize {
    dst.clear();
    let chunks = hex.chunks_exact(2);
    let mapd = chunks.flat_map(|s| <[u8; 2]>::try_from(s).ok());
    let u4s = mapd.map(u16::from_be_bytes);
    let converted = u4s.flat_map(u16str2byte_std);
    dst.extend(converted);
    dst.len()
}

#[cfg(test)]
mod test_hex_str2byte_std {
    use super::hex_str2byte_std;

    #[test]
    fn test_num() {
        assert_eq!(hex_str2byte_std("00"), Some(0x00));
        assert_eq!(hex_str2byte_std("01"), Some(0x01));
        assert_eq!(hex_str2byte_std("02"), Some(0x02));
        assert_eq!(hex_str2byte_std("09"), Some(0x09));
        assert_eq!(hex_str2byte_std("19"), Some(0x19));
        assert_eq!(hex_str2byte_std("29"), Some(0x29));
        assert_eq!(hex_str2byte_std("39"), Some(0x39));
        assert_eq!(hex_str2byte_std("99"), Some(0x99));
    }

    #[test]
    fn test_lo() {
        assert_eq!(hex_str2byte_std("0a"), Some(0x0a));
        assert_eq!(hex_str2byte_std("0b"), Some(0x0b));
        assert_eq!(hex_str2byte_std("0c"), Some(0x0c));
        assert_eq!(hex_str2byte_std("0d"), Some(0x0d));
        assert_eq!(hex_str2byte_std("0e"), Some(0x0e));
        assert_eq!(hex_str2byte_std("0f"), Some(0x0f));

        assert_eq!(hex_str2byte_std("a0"), Some(0xa0));
        assert_eq!(hex_str2byte_std("b0"), Some(0xb0));
        assert_eq!(hex_str2byte_std("c0"), Some(0xc0));
        assert_eq!(hex_str2byte_std("d0"), Some(0xd0));
        assert_eq!(hex_str2byte_std("e0"), Some(0xe0));
        assert_eq!(hex_str2byte_std("f0"), Some(0xf0));
    }

    #[test]
    fn test_hi() {
        assert_eq!(hex_str2byte_std("0A"), Some(0x0A));
        assert_eq!(hex_str2byte_std("0B"), Some(0x0B));
        assert_eq!(hex_str2byte_std("0C"), Some(0x0C));
        assert_eq!(hex_str2byte_std("0D"), Some(0x0D));
        assert_eq!(hex_str2byte_std("0E"), Some(0x0E));
        assert_eq!(hex_str2byte_std("0F"), Some(0x0F));

        assert_eq!(hex_str2byte_std("A0"), Some(0xA0));
        assert_eq!(hex_str2byte_std("B0"), Some(0xB0));
        assert_eq!(hex_str2byte_std("C0"), Some(0xC0));
        assert_eq!(hex_str2byte_std("D0"), Some(0xD0));
        assert_eq!(hex_str2byte_std("E0"), Some(0xE0));
        assert_eq!(hex_str2byte_std("F0"), Some(0xF0));
    }
}

#[cfg(test)]
mod test_u16str2byte_std {
    use super::u16str2byte_std;

    #[test]
    fn test_num() {
        assert_eq!(u16str2byte_std(0x3030), Some(0x00));
        assert_eq!(u16str2byte_std(0x3130), Some(0x10));
        assert_eq!(u16str2byte_std(0x3230), Some(0x20));
        assert_eq!(u16str2byte_std(0x3330), Some(0x30));
        assert_eq!(u16str2byte_std(0x3430), Some(0x40));
        assert_eq!(u16str2byte_std(0x3530), Some(0x50));
        assert_eq!(u16str2byte_std(0x3630), Some(0x60));
        assert_eq!(u16str2byte_std(0x3730), Some(0x70));
        assert_eq!(u16str2byte_std(0x3830), Some(0x80));
        assert_eq!(u16str2byte_std(0x3930), Some(0x90));
        assert_eq!(u16str2byte_std(0x3939), Some(0x99));
    }

    #[test]
    fn test_lo() {
        assert_eq!(u16str2byte_std(0x6130), Some(0xa0));
        assert_eq!(u16str2byte_std(0x6230), Some(0xb0));
        assert_eq!(u16str2byte_std(0x6330), Some(0xc0));
        assert_eq!(u16str2byte_std(0x6430), Some(0xd0));
        assert_eq!(u16str2byte_std(0x6530), Some(0xe0));
        assert_eq!(u16str2byte_std(0x6630), Some(0xf0));

        assert_eq!(u16str2byte_std(0x3061), Some(0x0a));
        assert_eq!(u16str2byte_std(0x3062), Some(0x0b));
        assert_eq!(u16str2byte_std(0x3063), Some(0x0c));
        assert_eq!(u16str2byte_std(0x3064), Some(0x0d));
        assert_eq!(u16str2byte_std(0x3065), Some(0x0e));
        assert_eq!(u16str2byte_std(0x3066), Some(0x0f));
    }

    #[test]
    fn test_hi() {
        assert_eq!(u16str2byte_std(0x4130), Some(0xA0));
        assert_eq!(u16str2byte_std(0x4230), Some(0xB0));
        assert_eq!(u16str2byte_std(0x4330), Some(0xC0));
        assert_eq!(u16str2byte_std(0x4430), Some(0xD0));
        assert_eq!(u16str2byte_std(0x4530), Some(0xE0));
        assert_eq!(u16str2byte_std(0x4630), Some(0xF0));

        assert_eq!(u16str2byte_std(0x3041), Some(0x0A));
        assert_eq!(u16str2byte_std(0x3042), Some(0x0B));
        assert_eq!(u16str2byte_std(0x3043), Some(0x0C));
        assert_eq!(u16str2byte_std(0x3044), Some(0x0D));
        assert_eq!(u16str2byte_std(0x3045), Some(0x0E));
        assert_eq!(u16str2byte_std(0x3046), Some(0x0F));
    }
}

#[cfg(test)]
mod test_hex_bytes2bytes_std {
    use super::hex_bytes2bytes_std;

    #[test]
    fn test_single() {
        let mut buf: Vec<u8> = Vec::with_capacity(1);

        hex_bytes2bytes_std(b"Ff", &mut buf);
        assert_eq!(vec![0xff], buf);

        hex_bytes2bytes_std(b"1f", &mut buf);
        assert_eq!(vec![0x1f], buf);

        hex_bytes2bytes_std(b"30", &mut buf);
        assert_eq!(vec![0x30], buf);
    }

    #[test]
    fn test_double() {
        let mut buf: Vec<u8> = Vec::with_capacity(1);

        hex_bytes2bytes_std(b"3776", &mut buf);
        assert_eq!(vec![0x37, 0x76], buf);
    }

    #[test]
    fn test_iii() {
        let mut buf: Vec<u8> = Vec::with_capacity(1);

        hex_bytes2bytes_std(b"101325", &mut buf);
        assert_eq!(vec![0x10, 0x13, 0x25], buf);
    }

    #[test]
    fn test_f32_little() {
        let mut buf: Vec<u8> = Vec::with_capacity(1);

        hex_bytes2bytes_std(b"00001242", &mut buf);
        assert_eq!(buf.len(), 4);
        let s: &[u8] = &buf;
        let b4: [u8; 4] = s.try_into().unwrap();
        let f: f32 = f32::from_le_bytes(b4);
        assert_eq!(36.5, f);
    }
}

#[cfg(feature = "ext_wasm")]
pub mod ext_wasm;
