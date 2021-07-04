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
pub trait FrameReader {}

/// Reader
pub trait Reader {}

/// Frame Writer
pub trait FrameWriter {}

/// Writer
pub trait Writer {}
