use std::io::{Read, Result, Write};
use std::marker::PhantomData;

pub mod bub;
pub mod io;
pub mod oao;
pub mod wav;

/// Metadata
pub trait Metadata: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(self, writer: &mut W) -> Result<()>;
}

/// Sample
pub trait Sample: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
    fn write<W: Write>(self, writer: &mut W) -> Result<()>;
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
pub trait FrameWriter {}

/// Writer
pub trait Writer {}
