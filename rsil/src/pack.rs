pub fn pack(stream: &[u8]) -> Vec<u8> {
    let mut packed = Vec::new();
    let mut current_byte: u8 = 0;
    let mut bit_count: usize = 0;

    for &symbol in stream {
        current_byte = (current_byte << 2) | (symbol & 3);
        bit_count += 2;

        if bit_count == 8 {
            packed.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        current_byte <<= 8 - bit_count;
        packed.push(current_byte);
    }

    packed
}

pub fn unpack(packed: &[u8], expected_len: usize) -> Vec<u8> {
    let mut stream = Vec::with_capacity(expected_len);
    let mut total_unpacked = 0;

    for &b in packed {
        let mut i: i32 = 6;
        while i >= 0 {
            if total_unpacked >= expected_len {
                return stream;
            }
            let symbol = (b >> i) & 3;
            stream.push(symbol);
            total_unpacked += 1;
            i -= 2;
        }
    }

    stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack() {
        let stream: Vec<u8> = vec![0, 1, 2, 3, 3, 2, 1, 0, 1, 2];
        let packed = pack(&stream);
        
        // 0(00), 1(01), 2(10), 3(11) -> 00 01 10 11 -> 0x1B
        // 3(11), 2(10), 1(01), 0(00) -> 11 10 01 00 -> 0xE4
        // 1(01), 2(10), p(00), p(00) -> 01 10 00 00 -> 0x60
        assert_eq!(packed, vec![0x1B, 0xE4, 0x60]);

        let unpacked = unpack(&packed, stream.len());
        assert_eq!(stream.len(), unpacked.len(), "Length mismatch");
        assert_eq!(stream, unpacked, "Content mismatch");
    }
}
