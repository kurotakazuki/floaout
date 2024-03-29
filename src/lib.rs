#![doc = include_str!("../README.md")]

use std::io::{ErrorKind, Read, Result, Write};

pub use crate::colors::{Rgb, Rgba};
pub use crate::coord::{BubFnsCoord, Coord};
pub use crate::crc::CRC_32K_4_2;
pub use crate::lpcm::{Frame, LpcmKind, Sample};
pub use crate::space::{OaoSpace, OaoSpaces};

pub mod bub;
pub mod colors;
pub mod coord;
pub mod crc;
pub mod io;
pub mod lpcm;
pub mod oao;
pub mod space;
pub mod utils;
pub mod wav;

/// Metadata
pub trait Metadata {}

/// FrameIOKind
pub enum FrameIOKind<F32LE, F64LE> {
    F32LE(F32LE),
    F64LE(F64LE),
}

impl<F32LE, F64LE> FrameIOKind<F32LE, F64LE> {
    pub fn into_f32_le(self) -> Result<F32LE> {
        match self {
            Self::F32LE(r) => Ok(r),
            Self::F64LE(_) => Err(utils::expected_and_found_error(
                ErrorKind::Other,
                LpcmKind::F32LE,
                LpcmKind::F64LE,
            )),
        }
    }

    pub fn into_f64_le(self) -> Result<F64LE> {
        match self {
            Self::F32LE(_) => Err(utils::expected_and_found_error(
                ErrorKind::Other,
                LpcmKind::F64LE,
                LpcmKind::F32LE,
            )),
            Self::F64LE(r) => Ok(r),
        }
    }
}

/// Frame Reader
pub trait FrameReader<R: Read, S: Sample>: Iterator<Item = Result<Frame<S>>> {
    fn get_ref(&self) -> &R;
    fn get_mut(&mut self) -> &mut R;
    fn into_inner(self) -> R;
    // From Metadata
    fn frames(&self) -> u64;
    fn samples_per_sec(&self) -> f64;

    /// Number of channels
    fn number_of_channels(&self) -> u32;
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
