use crate::samples::Sample;
use std::io::{Read, Result};

pub trait ReadBytes<R, F>: Sized
where
    R: ?Sized,
{
    /// This method reads bytes in big-endian byte order.
    fn read_be_bytes(reader: &mut R, f: &mut F) -> Result<Self>;
    /// This method reads bytes in little-endian byte order.
    fn read_le_bytes(reader: &mut R, f: &mut F) -> Result<Self>;
}

macro_rules! read_bytes_impl {
    ( $( $t:ty ),* ) => ($(

        impl<R, F> ReadBytes<R, F> for $t where R: Read, F: FnMut(&mut [u8]) -> Result<()> {
#[doc = concat!("Read bytes from a reader as big-endian byte order.
# Examples
```
use std::io::Read;
use oao_core::io::{ReadBytes, cb_ok};

let mut buf: &[u8] = &[0; 16];
let num: ", stringify!($t), " = ", stringify!($t), "::read_be_bytes(&mut buf, &mut cb_ok).unwrap();
assert_eq!(num, 0_", stringify!($t), ");
```")]
            fn read_be_bytes(reader: &mut R, f: &mut F) -> Result<Self> {
                let mut buf = [0; Self::FORMAT.sample_size()];
                reader.read_exact(&mut buf)?;
                f(&mut buf)?;
                Ok(<$t>::from_be_bytes(buf))
            }
#[doc = concat!("Read bytes from a reader as little-endian byte order.
# Examples
```
use std::io::Read;
use oao_core::io::{ReadBytes, cb_ok};

let mut buf: &[u8] = &[0; 16];
let num = ", stringify!($t), "::read_le_bytes(&mut buf, &mut cb_ok).unwrap();
assert_eq!(num, 0_", stringify!($t), ");
```")]
            fn read_le_bytes(reader: &mut R, f: &mut F) -> Result<Self> {
                let mut buf = [0; Self::FORMAT.sample_size()];
                reader.read_exact(&mut buf)?;
                f(&mut buf)?;
                Ok(<$t>::from_le_bytes(buf))
            }
        }
    )*)
}

read_bytes_impl!(u8, u16, u32, u64, u128);
read_bytes_impl!(i8, i16, i32, i64, i128);
read_bytes_impl!(f32, f64);
