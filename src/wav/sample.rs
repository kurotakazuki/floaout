use crate::io::{ReadExt, WriteExt};
use crate::Sample;
use std::io::{Read, Result, Write};

macro_rules! wav_le_sample_impl {
    ( $( $t:ty ),* ) => ($(
        impl Sample for $t {
            fn read<R: Read>(reader: &mut R) -> Result<Self> {
                reader.read_le()
            }

            fn write<W: Write>(self, writer: &mut W) -> Result<()> {
                writer.write_le(self)
            }
        }
    )*)
}

wav_le_sample_impl!(f32, f64);
