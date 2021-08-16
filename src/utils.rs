use mycrc::CRC;
use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub(crate) fn expected_and_found_error<T: Debug>(
    error_kind: ErrorKind,
    expected: T,
    found: T,
) -> Error {
    Error::new(
        error_kind,
        format!("expected `{:?}`, found `{:?}`", expected, found),
    )
}

pub(crate) fn is_equal<T: Debug + Eq>(error_kind: ErrorKind, expected: T, found: T) -> Result<()> {
    if found != expected {
        return Err(expected_and_found_error(error_kind, expected, found));
    }
    Ok(())
}

pub(crate) fn return_invalid_data_if_not_equal<T: Debug + Eq>(val: T, expect: T) -> Result<()> {
    is_equal(ErrorKind::InvalidData, expect, val)
}

pub(crate) fn read_crc<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<()> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    crc.calc_bytes(&buf);
    // TODO: Return Error
    assert!(crc.is_error_free());

    crc.initialize().calc_bytes(&buf);

    Ok(())
}
pub(crate) fn write_crc<W: Write>(writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
    let checksum_bytes = crc.finalize_to_endian_bytes();
    writer.write_all(&checksum_bytes)?;

    crc.initialize().calc_bytes(&checksum_bytes);

    Ok(())
}
