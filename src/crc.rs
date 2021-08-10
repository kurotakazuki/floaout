use mycrc::{Endian, CRC};

/// CRC
pub const CRC: CRC<u32> = CRC::<u32>::new(
    Endian::Little, // endian
    0x93a409eb,     // poly CRC-32K/4.2
    0xffffffff,     // init
    true,           // refin
    true,           // refout
    0xffffffff,     // xorout
);

#[cfg(test)]
mod tests {
    use super::*;
    use mycrc::Algorithm;

    // check: 0xeea8baa4
    const CHECK_BYTES: &[u8] = b"123456789";

    #[test]
    fn crc() {
        let algo = Algorithm::<u32> {
            endian: Endian::Little,
            poly: 0x93a409eb, // CRC-32K/4.2
            init: 0xffffffff,
            refin: true,
            refout: true,
            xorout: 0xffffffff,
            residue: 0x76e908ce,
        };
        let mut crc = CRC;
        // Is algorithm same?
        assert_eq!(crc.algorithm, algo);
        // Is check same?
        assert_eq!(crc.checksum(CHECK_BYTES), 0xeea8baa4);
        // Is error-free?
        let checksum = crc.checksum_to_endian_bytes(CHECK_BYTES);
        let bytes = [CHECK_BYTES, &checksum].concat();
        assert!(crc.is_error_free(&bytes));
    }
}
