#![doc = include_str!("../README.md")]

use std::io::{Read, Result, Write};

pub use crate::coord::Coord;
pub use crate::crc::CRC;
pub use crate::lpcm::{Frame, LpcmKind, Sample};

pub mod bub;
pub mod coord;
pub mod crc;
pub mod io;
pub mod lpcm;
pub mod oao;
pub mod utils;
pub mod wav;

/// Metadata
pub trait Metadata {}

/// Frame Reader
pub trait FrameReader<R: Read> {
    fn get_ref(&self) -> &R;

    fn get_mut(&mut self) -> &mut R;

    fn into_inner(self) -> R;
}
/// Reader
pub trait Reader {}

/// Frame Writer
pub trait FrameWriter<W: Write> {
    fn flush(&mut self) -> Result<()>;

    fn get_ref(&self) -> &W;

    fn get_mut(&mut self) -> &mut W;

    fn into_inner(self) -> W;
}
/// Writer
pub trait Writer {}
