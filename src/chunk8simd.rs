use core::arch::wasm32::v128;

use core::arch::wasm32::{u8x16, u8x16_ge, u8x16_shl, u8x16_splat, u8x16_sub, u8x16_swizzle};

use core::arch::wasm32::{u16x8_shl, u16x8_splat};

use core::arch::wasm32::{u64x2, u64x2_extract_lane};

use core::arch::wasm32::{v128_and, v128_bitselect, v128_or};

/// Converts hex chars([0-9A-F]) to numbers.
pub fn hex2num_upper(hex: v128) -> v128 {
    let th: v128 = u8x16_splat(b'A');
    let b7: v128 = u8x16_ge(hex, th);
    let sn: v128 = u8x16_splat(b'0');
    let sa: v128 = u8x16_splat(b'A' - 10);
    let s: v128 = v128_bitselect(sa, sn, b7);
    u8x16_sub(hex, s)
}

/// Converts hex chars to numbers(assuming chars are [0-9A-F])
pub fn hex2num(hex: v128) -> v128 {
    hex2num_upper(hex)
}

pub fn hex_str2u64(hex: u128) -> u64 {
    let hi: u128 = hex >> 64;
    let lo: u128 = hex & 0xffff_ffff_ffff_ffff;
    let h: u64 = hi as u64;
    let l: u64 = lo as u64;
    let raw: v128 = u64x2(l, h);

    let num: v128 = hex2num(raw); // 0x00,0x01,0x02, ..., 0x0F

    let cev: v128 = u16x8_splat(0xff00); // 0,2,4,6,8,a,c,e
    let cod: v128 = u16x8_splat(0x00ff); // 1,3,5,7,9,b,d,f
    let ev: v128 = v128_and(num, cev); // 0x0*, 0x00, 0x0*, 0x00, ...
    let od: v128 = v128_and(num, cod); // 0x00, 0x0*, 0x00, 0x0*, ...

    let e4: v128 = u8x16_shl(ev, 4); // 0x*0, 0x00, 0x*0, 0x00, ...
    let o8: v128 = u16x8_shl(od, 8); // 0x0*00,     0x0*00, ...
    let eo: v128 = v128_or(e4, o8); //  0x**00,     0x**00, ...

    let c8: v128 = u8x16(
        0x00, 0x02, 0x04, 0x06, 0x08, 0x0a, 0x0c, 0x0e, 0x01, 0x03, 0x05, 0x07, 0x09, 0x0b, 0x0d,
        0x0f,
    );
    let v8: v128 = u8x16_swizzle(eo, c8);
    u64x2_extract_lane::<1>(v8)
}

pub fn hex_str_bytes2buf(hex: &[u8], buf: &mut Vec<u8>) -> usize {
    buf.clear();
    let chunks = hex.chunks_exact(16);
    let u7s = chunks.flat_map(|s: &[u8]| s.try_into().ok().map(u128::from_be_bytes));
    let u6s = u7s.map(hex_str2u64);
    let mapd = u6s.map(u64::to_be_bytes);
    mapd.for_each(|u: [u8; 8]| {
        let s: &[u8] = &u;
        buf.extend(s);
    });
    buf.len()
}
