use crate::io::{ReadExt, WriteExt};
use mycrc::CRC;
use std::io::{ErrorKind, Read, Result, Write};
use std::ops::Mul;

/// LPCM Sample
pub trait Sample: Sized + Default + Mul<Output = Self> + Clone + Copy + PartialEq {
    fn from_f64(n: f64) -> Self;
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(self, writer: &mut W) -> Result<()>;
}

macro_rules! le_sample_impl {
    ( $( $t:ty ),* ) => ($(
        impl Sample for $t {
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

le_sample_impl!(f32, f64);

/// LPCM Frame
#[derive(Clone, Debug, PartialEq)]
pub struct Frame<S: Sample>(pub Vec<S>);

impl<S: Sample> From<Frame<S>> for Vec<S> {
    fn from(value: Frame<S>) -> Self {
        value.0
    }
}

impl<S: Sample> From<Vec<S>> for Frame<S> {
    fn from(value: Vec<S>) -> Self {
        Frame(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LpcmKind {
    F32LE,
    F64LE,
}

impl LpcmKind {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value: u8 = reader.read_le()?;
        Ok(match value {
            0 => Self::F32LE,
            1 => Self::F64LE,
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }
    pub fn read_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
        let value: u8 = reader.read_le_and_calc_bytes(crc)?;
        Ok(match value {
            0 => Self::F32LE,
            1 => Self::F64LE,
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.to_u8())
    }
    pub fn write_and_calc_bytes<W: Write>(self, writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
        writer.write_le_and_calc_bytes(self.to_u8(), crc)
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::F32LE,
            1 => Self::F64LE,
            _ => unimplemented!(),
        }
    }

    pub const fn to_u8(self) -> u8 {
        match self {
            Self::F32LE => 0,
            Self::F64LE => 1,
        }
    }

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
