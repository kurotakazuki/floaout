use crate::io::ReadBytes;
use mycrc::CRC;
use std::io::{Error, ErrorKind, Read, Result};

pub trait ReadExt: Read + Sized {
    fn read_be<T: ReadBytes>(&mut self) -> Result<T> {
        <T>::read_be_bytes(self)
    }
    fn read_le<T: ReadBytes>(&mut self) -> Result<T> {
        <T>::read_le_bytes(self)
    }

    fn read_be_and_calc_bytes<T: ReadBytes>(&mut self, crc: &mut CRC<u32>) -> Result<T> {
        <T>::read_be_bytes_and_calc_bytes(self, crc)
    }
    fn read_le_and_calc_bytes<T: ReadBytes>(&mut self, crc: &mut CRC<u32>) -> Result<T> {
        <T>::read_le_bytes_and_calc_bytes(self, crc)
    }

    fn read_vec_for(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn read_vec_for_and_calc_bytes(&mut self, n: usize, crc: &mut CRC<u32>) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf)?;
        crc.calc_bytes(&buf);
        Ok(buf)
    }

    fn read_string_for(&mut self, n: usize) -> Result<String> {
        let vec = self.read_vec_for(n)?;
        let s = String::from_utf8(vec);
        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
    fn read_string_for_and_calc_bytes(&mut self, n: usize, crc: &mut CRC<u32>) -> Result<String> {
        let vec = self.read_vec_for_and_calc_bytes(n, crc)?;
        let s = String::from_utf8(vec);
        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }

    fn read_array<const LEN: usize>(&mut self) -> Result<[u8; LEN]> {
        let mut buf = [0; LEN];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn read_array_and_calc_bytes<const LEN: usize>(
        &mut self,
        crc: &mut CRC<u32>,
    ) -> Result<[u8; LEN]> {
        let mut buf = [0; LEN];
        self.read_exact(&mut buf)?;
        crc.calc_bytes(&buf);
        Ok(buf)
    }

    fn read_string<const LEN: usize>(&mut self) -> Result<String> {
        let bytes = self.read_array::<LEN>()?;
        let s = String::from_utf8(bytes.to_vec());

        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
    fn read_stringy_and_calc_bytes<const LEN: usize>(
        &mut self,
        crc: &mut CRC<u32>,
    ) -> Result<String> {
        let bytes = self.read_array_and_calc_bytes::<LEN>(crc)?;
        let s = String::from_utf8(bytes.to_vec());

        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}

impl<R: Read> ReadExt for R {}

#[cfg(test)]
mod tests {
    use super::ReadExt;
    use std::io;

    #[test]
    fn read_str_for() {
        let mut v: &[u8] = &[111, 97, 111];
        let bytes = v.read_array::<3>().unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(s, "oao");
        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s = v.read_string::<15>().unwrap();
        assert_eq!(s, "びかびかび");

        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s = v.read_string_for(15).unwrap();
        assert_eq!(s, "びかびかび");
    }

    #[test]
    fn read_unsigned_integer_type() -> io::Result<()> {
        let mut v: &[u8] = &[
            1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5,
        ];

        let bytes_u8: u8 = v.read_le()?;
        assert_eq!(bytes_u8, 1);

        let bytes_u16: u16 = v.read_le()?;
        assert_eq!(bytes_u16, 2);

        let bytes_u32: u32 = v.read_le()?;
        assert_eq!(bytes_u32, 3);

        let bytes_u64: u64 = v.read_le()?;
        assert_eq!(bytes_u64, 4);

        let bytes_u128: u128 = v.read_be()?;
        assert_eq!(bytes_u128, 5);

        Ok(())
    }
}
