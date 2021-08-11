use crate::io::write::write_bytes::WriteBytes;
use mycrc::CRC;
use std::io::{Result, Write};

pub trait WriteExt: Write + Sized {
    fn write_be<T: WriteBytes>(&mut self, n: T) -> Result<()> {
        n.write_be_bytes(self)
    }
    fn write_le<T: WriteBytes>(&mut self, n: T) -> Result<()> {
        n.write_le_bytes(self)
    }

    fn write_be_and_calc_bytes<T: WriteBytes>(&mut self, n: T, crc: &mut CRC<u32>) -> Result<()> {
        n.write_be_bytes_and_calc_bytes(self, crc)
    }
    fn write_le_and_calc_bytes<T: WriteBytes>(&mut self, n: T, crc: &mut CRC<u32>) -> Result<()> {
        n.write_le_bytes_and_calc_bytes(self, crc)
    }

    fn write_str(&mut self, s: &str) -> Result<()> {
        self.write_all(s.as_bytes())
    }
    fn write_str_and_calc_bytes(&mut self, s: &str, crc: &mut CRC<u32>) -> Result<()> {
        let bytes = s.as_bytes();
        self.write_all(bytes)?;
        crc.calc_bytes(bytes);
        Ok(())
    }
}

impl<W: Write> WriteExt for W {}

#[cfg(test)]
mod tests {
    use super::WriteExt;
    use std::io;

    #[test]
    fn write_str() -> io::Result<()> {
        let mut v = Vec::new();
        v.write_str("oao")?;

        assert_eq!(v, [111, 97, 111]);

        Ok(())
    }

    #[test]
    fn write_unsigned_integer_type() -> io::Result<()> {
        let mut v = Vec::new();

        v.write_le(1_u8)?;
        v.write_le(2_u16)?;
        v.write_le(3_u32)?;
        v.write_le(4_u64)?;
        v.write_be(5_u128)?;

        assert_eq!(
            v,
            [
                1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 5,
            ]
        );

        Ok(())
    }
}
