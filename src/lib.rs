use crate::io::{ReadExt, WriteExt};
use std::io::{ErrorKind, Read, Result, Write};
use std::marker::PhantomData;

pub mod bub;
pub mod io;
pub mod oao;
pub mod utils;
pub mod wav;

/// Metadata
pub trait Metadata: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(self, writer: &mut W) -> Result<()>;
}

/// Sample
pub trait Sample: Sized {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SampleKind {
    F32LE,
    F64LE,
}

impl SampleKind {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value: u8 = reader.read_le()?;
        Ok(match value {
            0 => Self::F32LE,
            1 => Self::F64LE,
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.to_u8())
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

/// Frame Reader
pub struct FrameReader<R: Read, M: Metadata, S: Sample> {
    pub inner: R,
    pub metadata: M,
    pub pos: u32,
    _phantom_sample: PhantomData<S>,
}

impl<R: Read, M: Metadata, S: Sample> FrameReader<R, M, S> {
    pub fn new(inner: R, metadata: M) -> Self {
        Self {
            inner,
            metadata,
            pos: 0,
            _phantom_sample: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

/// Reader
pub trait Reader {}

/// Frame Writer
pub struct FrameWriter<W: Write, M: Metadata, S: Sample> {
    pub inner: W,
    pub metadata: M,
    pub pos: u32,
    _phantom_sample: PhantomData<S>,
}

impl<W: Write, M: Metadata, S: Sample> FrameWriter<W, M, S> {
    pub fn new(inner: W, metadata: M) -> Self {
        let pos = 0;

        Self {
            inner,
            metadata,
            pos,
            _phantom_sample: PhantomData,
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

/// Writer
pub trait Writer {}
