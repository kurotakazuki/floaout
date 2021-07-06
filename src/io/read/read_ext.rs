use crate::io::ReadBytes;
use std::io::{Error, ErrorKind, Read, Result};

pub trait ReadExt: Read + Sized {
    fn read_be<T: ReadBytes>(&mut self) -> Result<T> {
        <T>::read_be_bytes(self)
    }

    fn read_le<T: ReadBytes>(&mut self) -> Result<T> {
        <T>::read_le_bytes(self)
    }

    fn read_bytes_for<const LEN: usize>(&mut self) -> Result<[u8; LEN]> {
        let mut buf = [0; LEN];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_string_for<const LEN: usize>(&mut self) -> Result<String> {
        let bytes = self.read_bytes_for::<3>()?;
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
        let bytes = v.read_bytes_for::<3>().unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(s, "oao");
        let mut v: &[u8] = &[111, 97, 111];
        let bytes = v.read_string_for::<3>().unwrap();
        assert_eq!(bytes, "oao");
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
