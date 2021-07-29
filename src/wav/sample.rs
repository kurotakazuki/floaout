use crate::io::{ReadExt, WriteExt};
use crate::Sample;
use std::io::{Read, Result, Write};
use std::ops::Mul;

impl Sample for f32 {}
impl Sample for f64 {}

pub trait WavSample: Sample + Default + Mul<Output = Self> + Clone + Copy + PartialEq {
    fn from_f64(n: f64) -> Self;
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(self, writer: &mut W) -> Result<()>;
}

macro_rules! wav_le_sample_impl {
    ( $( $t:ty ),* ) => ($(
        impl WavSample for $t {
            fn from_f64(n: f64) -> Self {
                n as $t
            }

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

/// This size is equal to block align.
pub type WavFrame<S> = Vec<S>;
