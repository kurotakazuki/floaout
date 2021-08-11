use mycrc::CRC;
use std::io::{Read, Result};
use std::mem;

pub trait ReadBytes: Sized {
    /// This method reads bytes in big-endian byte order.
    fn read_be_bytes<R: Read>(reader: &mut R) -> Result<Self>;
    /// This method reads bytes in little-endian byte order.
    fn read_le_bytes<R: Read>(reader: &mut R) -> Result<Self>;

    fn read_be_bytes_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self>;
    fn read_le_bytes_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self>;
}

macro_rules! read_bytes_impl {
    ( $( $t:ty ),* ) => ($(
        impl ReadBytes for $t {
            fn read_be_bytes<R: Read>(reader: &mut R) -> Result<Self> {
                let mut buf = [0; mem::size_of::<$t>()];
                reader.read_exact(&mut buf)?;
                Ok(<$t>::from_be_bytes(buf))
            }

            fn read_le_bytes<R: Read>(reader: &mut R) -> Result<Self> {
                let mut buf = [0; mem::size_of::<$t>()];
                reader.read_exact(&mut buf)?;
                Ok(<$t>::from_le_bytes(buf))
            }

            fn read_be_bytes_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
                let mut buf = [0; mem::size_of::<$t>()];
                reader.read_exact(&mut buf)?;
                crc.calc_bytes(&buf);
                Ok(<$t>::from_be_bytes(buf))
            }

            fn read_le_bytes_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
                let mut buf = [0; mem::size_of::<$t>()];
                reader.read_exact(&mut buf)?;
                crc.calc_bytes(&buf);
                Ok(<$t>::from_le_bytes(buf))
            }
        }
    )*)
}

read_bytes_impl!(f32, f64);
read_bytes_impl!(isize, i8, i16, i32, i64, i128);
read_bytes_impl!(usize, u8, u16, u32, u64, u128);
