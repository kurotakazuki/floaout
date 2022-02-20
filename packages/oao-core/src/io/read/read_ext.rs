use crate::io::ReadBytes;
use std::io::{Error, ErrorKind, Read, Result};

pub trait ReadExt {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

    fn read_be<T: ReadBytes<Self, F>, F>(&mut self, f: &mut F) -> Result<T> {
        <T>::read_be_bytes(self, f)
    }
    fn read_le<T: ReadBytes<Self, F>, F>(&mut self, f: &mut F) -> Result<T> {
        <T>::read_le_bytes(self, f)
    }

    fn read_boxed_slice_exact<F>(&mut self, n: usize, f: &mut F) -> Result<Box<[u8]>>
    where
        F: FnMut(&mut [u8]) -> Result<()>,
    {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf)?;
        f(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }

    fn read_vec_exact<F>(&mut self, n: usize, f: &mut F) -> Result<Vec<u8>>
    where
        F: FnMut(&mut [u8]) -> Result<()>,
    {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf)?;
        f(&mut buf)?;
        Ok(buf)
    }

    fn read_string_exact<F>(&mut self, n: usize, f: &mut F) -> Result<String>
    where
        F: FnMut(&mut [u8]) -> Result<()>,
    {
        let vec = self.read_vec_exact(n, f)?;
        let s = String::from_utf8(vec);
        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }

    fn read_array<F, const N: usize>(&mut self, f: &mut F) -> Result<[u8; N]>
    where
        F: FnMut(&mut [u8]) -> Result<()>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        f(&mut buf)?;
        Ok(buf)
    }

    fn read_string<F, const N: usize>(&mut self, f: &mut F) -> Result<String>
    where
        F: FnMut(&mut [u8]) -> Result<()>,
    {
        let bytes = self.read_array::<F, N>(f)?;
        let s = String::from_utf8(bytes.to_vec());

        match s {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}

impl<R: Read> ReadExt for R {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::cb_ok;
    use std::io;

    #[test]
    fn read_unsigned_integer() -> io::Result<()> {
        let mut v: &[u8] = &[
            1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5,
        ];

        let bytes_u8: u8 = v.read_le(&mut cb_ok)?;
        assert_eq!(bytes_u8, 1);

        let bytes_u16: u16 = v.read_le(&mut cb_ok)?;
        assert_eq!(bytes_u16, 2);

        let bytes_u32: u32 = v.read_le(&mut cb_ok)?;
        assert_eq!(bytes_u32, 3);

        let bytes_u64: u64 = v.read_le(&mut cb_ok)?;
        assert_eq!(bytes_u64, 4);

        let bytes_u128: u128 = v.read_be(&mut cb_ok)?;
        assert_eq!(bytes_u128, 5);

        Ok(())
    }

    #[test]
    fn read_string() {
        let mut v: &[u8] = &[111, 97, 111];
        let bytes: [u8; 3] = v.read_array(&mut cb_ok).unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(s, "oao");
        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s = v.read_string::<_, 15>(&mut cb_ok).unwrap();
        assert_eq!(s, "びかびかび");

        let mut v: &[u8] = &[
            0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81, 0xB3, 0xE3, 0x81, 0x8B, 0xE3, 0x81,
            0xB3,
        ];
        let s = v.read_string_exact(15, &mut cb_ok).unwrap();
        assert_eq!(s, "びかびかび");
    }
}
