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

pub type WavFrame<S: WavSample> = Vec<S>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WavSampleKind {
    F32LE,
    F64LE,
}

impl WavSampleKind {
    pub const fn format_tag(&self) -> u16 {
        match self {
            Self::F32LE => 3,
            Self::F64LE => 3,
        }
    }

    pub const fn bits_per_sample(&self) -> u16 {
        match self {
            Self::F32LE => 32,
            Self::F64LE => 64,
        }
    }

    pub fn from_format_tag_and_bits_per_sample(format_tag: u16, bits_per_sample: u16) -> Self {
        match format_tag {
            1 => {
                todo!()
            }
            3 => match bits_per_sample {
                32 => Self::F32LE,
                64 => Self::F64LE,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
