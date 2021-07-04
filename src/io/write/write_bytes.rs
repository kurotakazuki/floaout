use std::io::{Result, Write};

pub trait WriteBytes {
    /// This method writes bytes in big-endian byte order.
    fn write_be_bytes<W: Write>(&self, writer: &mut W) -> Result<()>;
    /// This method writes bytes in little-endian byte order.
    fn write_le_bytes<W: Write>(&self, writer: &mut W) -> Result<()>;
}

macro_rules! write_bytes_impl {
    ( $( $t:ty ),* ) => ($(
        impl WriteBytes for $t {
            fn write_be_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
                writer.write_all(&self.to_be_bytes())
            }

            fn write_le_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
                writer.write_all(&self.to_le_bytes())
            }
        }
    )*)
}

write_bytes_impl!(f32, f64);
write_bytes_impl!(isize, i8, i16, i32, i64, i128);
write_bytes_impl!(usize, u8, u16, u32, u64, u128);
