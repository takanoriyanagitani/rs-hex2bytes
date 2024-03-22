use core::ptr::{addr_of, addr_of_mut};

static mut I_HEX_BYTES: Vec<u8> = vec![];
static mut O_BYTES: Vec<u8> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn input_resize(sz: i32) -> i32 {
    let mvp: *mut Vec<u8> = unsafe { addr_of_mut!(I_HEX_BYTES) };
    let omv: Option<&mut Vec<u8>> = unsafe { mvp.as_mut() };
    omv.and_then(|mv: &mut Vec<u8>| {
        mv.resize(sz as usize, 0);
        mv.capacity().try_into().ok()
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn input_ptr() -> *mut u8 {
    let mv: *mut Vec<u8> = unsafe { addr_of_mut!(I_HEX_BYTES) };
    let omv: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
    omv.map(|m| m.as_mut_ptr())
        .unwrap_or_else(core::ptr::null_mut)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn output_ptr() -> *mut u8 {
    let mv: *mut Vec<u8> = unsafe { addr_of_mut!(O_BYTES) };
    let omv: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
    omv.map(|m| m.as_mut_ptr())
        .unwrap_or_else(core::ptr::null_mut)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn output_reset(sz: i32) -> i32 {
    let mv: *mut Vec<u8> = unsafe { addr_of_mut!(O_BYTES) };
    let omv: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
    omv.and_then(|mv: &mut Vec<u8>| {
        let cap: usize = mv.capacity();
        let tgt: usize = sz as usize;
        let add: usize = tgt.saturating_sub(cap);
        mv.try_reserve(add)
            .ok()
            .and_then(|_| mv.capacity().try_into().ok())
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn hex2bytes_std() -> i32 {
    let cv: *const Vec<u8> = unsafe { addr_of!(I_HEX_BYTES) };
    let ov: Option<&Vec<u8>> = unsafe { cv.as_ref() };
    ov.and_then(|v: &Vec<u8>| {
        let i: &[u8] = v;
        let mv: *mut Vec<u8> = unsafe { addr_of_mut!(O_BYTES) };
        let ov: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
        ov.and_then(|o: &mut Vec<u8>| crate::hex_bytes2bytes_std(i, o).try_into().ok())
    })
    .unwrap_or(-1)
}

#[cfg(feature = "chunk8")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn hex2bytes_std_chunk8() -> i32 {
    let cv: *const Vec<u8> = unsafe { addr_of!(I_HEX_BYTES) };
    let ov: Option<&Vec<u8>> = unsafe { cv.as_ref() };
    ov.and_then(|v: &Vec<u8>| {
        let i: &[u8] = v;
        let mv: *mut Vec<u8> = unsafe { addr_of_mut!(O_BYTES) };
        let ov: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
        ov.and_then(|o: &mut Vec<u8>| crate::chunk8::hex_str_bytes2buf(i, o).try_into().ok())
    })
    .unwrap_or(-1)
}

#[cfg(feature = "chunk8simd")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn hex2bytes_chunk8simd() -> i32 {
    let cv: *const Vec<u8> = unsafe { addr_of!(I_HEX_BYTES) };
    let ov: Option<&Vec<u8>> = unsafe { cv.as_ref() };
    ov.and_then(|v: &Vec<u8>| {
        let i: &[u8] = v;
        let mv: *mut Vec<u8> = unsafe { addr_of_mut!(O_BYTES) };
        let ov: Option<&mut Vec<u8>> = unsafe { mv.as_mut() };
        ov.and_then(|o: &mut Vec<u8>| crate::chunk8simd::hex_str_bytes2buf(i, o).try_into().ok())
    })
    .unwrap_or(-1)
}
