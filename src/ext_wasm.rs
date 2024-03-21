static mut I_HEX_BYTES: Vec<u8> = vec![];
static mut O_BYTES: Vec<u8> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn input_resize(sz: i32) -> i32 {
    let mv: &mut Vec<u8> = unsafe { &mut I_HEX_BYTES };
    mv.resize(sz as usize, 0);
    mv.capacity().try_into().ok().unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn input_ptr() -> *mut u8 {
    let mv: &mut Vec<u8> = unsafe { &mut I_HEX_BYTES };
    mv.as_mut_ptr()
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn output_ptr() -> *mut u8 {
    let mv: &mut Vec<u8> = unsafe { &mut O_BYTES };
    mv.as_mut_ptr()
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn output_reset(sz: i32) -> i32 {
    let mv: &mut Vec<u8> = unsafe { &mut O_BYTES };
    let cap: usize = mv.capacity();
    let tgt: usize = sz as usize;
    let add: usize = tgt.saturating_sub(cap);
    mv.try_reserve(add)
        .ok()
        .and_then(|_| mv.capacity().try_into().ok())
        .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn hex2bytes_std() -> i32 {
    let i: &[u8] = unsafe { &I_HEX_BYTES };
    let o: &mut Vec<u8> = unsafe { &mut O_BYTES };
    crate::hex_bytes2bytes_std(i, o)
        .try_into()
        .ok()
        .unwrap_or(-1)
}

#[cfg(feature = "chunk8")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn hex2bytes_std_chunk8() -> i32 {
    let i: &[u8] = unsafe { &I_HEX_BYTES };
    let o: &mut Vec<u8> = unsafe { &mut O_BYTES };
    crate::chunk8::hex_str_bytes2buf(i, o)
        .try_into()
        .ok()
        .unwrap_or(-1)
}
