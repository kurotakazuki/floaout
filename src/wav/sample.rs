use crate::io::{ReadExt, WriteExt};
use crate::Sample;
use std::io::{Read, Write};

macro_rules! wav_le_sample_impl {
    ( $( $t:ty ),* ) => ($(
        impl Sample for $t {
            fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
                reader.read_le()
            }

            fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()> {
                writer.write_le(self)
            }
        }
    )*)
}

wav_le_sample_impl!(f32, f64);

pub trait WavSample: Sample {}

impl WavSample for f32 {}
impl WavSample for f64 {}

/// This size is equal to block align.
pub type WavFrame<S> = Vec<S>;
