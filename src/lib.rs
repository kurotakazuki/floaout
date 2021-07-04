pub mod bub;
pub mod io;
pub mod oao;
pub mod wav;

/// Metadata
pub trait Metadata {}

/// Sample
pub trait Sample {}

/// Frame Reader
pub trait FrameReader {}

/// Reader
pub trait Reader {}

/// Frame Writer
pub trait FrameWriter {}

/// Writer
pub trait Writer {}
