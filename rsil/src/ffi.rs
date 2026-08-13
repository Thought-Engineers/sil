use std::slice;

#[unsafe(no_mangle)]
pub extern "C" fn sil_compress(
    src: *const u8,
    srclen: usize,
    dst: *mut u8,
    dstlen: *mut usize,
) -> i32 {
    if src.is_null() || dst.is_null() || dstlen.is_null() {
        return -1;
    }
    
    let input = unsafe { slice::from_raw_parts(src, srclen) };
    let text = String::from_utf8_lossy(input);
    
    let fz = crate::fractalzip::FractalZip::new();
    let stream = fz.compress(&text);
    let packed = crate::pack::pack(&stream);
    
    let total_len = 8 + packed.len();
    
    let out_len = unsafe { *dstlen };
    if out_len < total_len {
        return -1;
    }
    
    let dst_slice = unsafe { slice::from_raw_parts_mut(dst, total_len) };
    
    let stream_len = stream.len() as u64;
    dst_slice[0..8].copy_from_slice(&stream_len.to_le_bytes());
    dst_slice[8..total_len].copy_from_slice(&packed);
    
    unsafe { *dstlen = total_len };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn sil_decompress(
    src: *const u8,
    srclen: usize,
    dst: *mut u8,
    dstlen: *mut usize,
) -> i32 {
    if src.is_null() || dst.is_null() || dstlen.is_null() || srclen < 8 {
        return -1;
    }
    
    let input = unsafe { slice::from_raw_parts(src, srclen) };
    
    let mut length_buf = [0u8; 8];
    length_buf.copy_from_slice(&input[0..8]);
    let stream_len = u64::from_le_bytes(length_buf) as usize;
    
    let packed = &input[8..];
    
    let stream = crate::pack::unpack(packed, stream_len);
    let fz = crate::fractalzip::FractalZip::new();
    let recovered = fz.decompress(&stream);
    
    let out_bytes = recovered.as_bytes();
    let out_len = unsafe { *dstlen };
    if out_len < out_bytes.len() {
        return -1;
    }
    
    let dst_slice = unsafe { slice::from_raw_parts_mut(dst, out_bytes.len()) };
    dst_slice.copy_from_slice(out_bytes);
    
    unsafe { *dstlen = out_bytes.len() };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_compress_decompress() {
        let input = b"hello ffi world hello ffi";
        let mut compressed = vec![0u8; 1024];
        let mut comp_len = compressed.len();

        let res = sil_compress(
            input.as_ptr(),
            input.len(),
            compressed.as_mut_ptr(),
            &mut comp_len,
        );
        assert_eq!(res, 0, "Compression should succeed");

        let mut decompressed = vec![0u8; 1024];
        let mut decomp_len = decompressed.len();

        let res2 = sil_decompress(
            compressed.as_ptr(),
            comp_len,
            decompressed.as_mut_ptr(),
            &mut decomp_len,
        );
        assert_eq!(res2, 0, "Decompression should succeed");
        assert_eq!(decomp_len, input.len());
        
        let out_slice = unsafe { std::slice::from_raw_parts(decompressed.as_ptr(), decomp_len) };
        assert_eq!(out_slice, input);
    }
}
