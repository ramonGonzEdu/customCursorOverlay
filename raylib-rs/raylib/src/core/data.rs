//! Data manipulation functions. Compress and Decompress with DEFLATE
use crate::ffi;

/// Compress data (DEFLATE algorythm)
/// ```rust
/// use raylib::prelude::*;
/// let data = compress_data(b"1111111111");
/// let expected: &[u8] = &[61, 193, 33, 1, 0, 0, 0, 128, 160, 77, 254, 63, 103, 3, 98];
/// assert_eq!(data, Ok(expected));
/// ```
pub fn compress_data(data: &[u8]) -> Result<&'static [u8], String> {
    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::CompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err("could not compress data".to_string());
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    return Ok(buffer);
}

/// Decompress data (DEFLATE algorythm)
/// ```rust
/// use raylib::prelude::*;
/// let input: &[u8] = &[61, 193, 33, 1, 0, 0, 0, 128, 160, 77, 254, 63, 103, 3, 98];
/// let expected: &[u8] = b"1111111111";
/// let data = decompress_data(input);
/// assert_eq!(data, Ok(expected));
/// ```
pub fn decompress_data(data: &[u8]) -> Result<&'static [u8], String> {
    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::DecompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err("could not compress data".to_string());
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    return Ok(buffer);
}
