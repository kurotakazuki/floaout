use crate::oao::{OaoFrameReader, OaoFrameReaderKind, OaoMetadata};
use crate::{Coord, LpcmKind, Sample};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct OaoReader<R: Read> {
    pub inner: R,
    pub metadata: OaoMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coord: Vec<Coord>,
}

impl<R: Read> OaoReader<R> {
    pub fn new(mut inner: R, speakers_absolute_coord: Vec<Coord>) -> Result<Self> {
        let metadata = OaoMetadata::read(&mut inner)?;

        Ok(Self {
            inner,
            metadata,
            speakers_absolute_coord,
        })
    }

    pub fn into_oao_frame_reader<S: Sample>(self) -> OaoFrameReader<R, S> {
        OaoFrameReader::new(self.inner, self.metadata, self.speakers_absolute_coord)
    }

    pub fn into_oao_frame_reader_kind(self) -> OaoFrameReaderKind<R> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => OaoFrameReaderKind::F32LE(OaoFrameReader::<R, f32>::new(
                self.inner,
                self.metadata,
                self.speakers_absolute_coord,
            )),
            LpcmKind::F64LE => OaoFrameReaderKind::F64LE(OaoFrameReader::<R, f64>::new(
                self.inner,
                self.metadata,
                self.speakers_absolute_coord,
            )),
        }
    }
}

impl OaoReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(filename: P, speakers_absolute_coord: Vec<Coord>) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader, speakers_absolute_coord)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
