pub mod bub;
pub mod io;
pub mod oao;
pub mod wav;

/// Metadata
pub trait Metadata {}

/// Sample
pub trait Sample: Sized {
    fn read<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>;

    fn write<W: std::io::Write>(self, writer: &mut W) -> std::io::Result<()>;
}

/// Frame Reader
pub struct FrameReader<R, M> {
    pub inner: R,
    pub metadata: M,
    pub pos: u32,
}

impl<R, M> FrameReader<R, M> {
    pub fn new(inner: R, metadata: M) -> Self {
        Self {
            inner,
            metadata,
            pos: 0,
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
