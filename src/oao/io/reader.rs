use crate::bub::BubFrameReader;
use crate::oao::{OaoFrameReader, OaoMetadata};
use crate::{BubFnsCoord, OaoSpaces, Sample};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct OaoReader<R: Read> {
    pub inner: R,
    pub metadata: OaoMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coord: Vec<BubFnsCoord>,
}

impl<R: Read> OaoReader<R> {
    pub fn new(mut inner: R, speakers_absolute_coord: Vec<BubFnsCoord>) -> Result<Self> {
        let metadata = OaoMetadata::read(&mut inner)?;

        Ok(Self {
            inner,
            metadata,
            speakers_absolute_coord,
        })
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`LpcmKind`]
    // TODO: Is metadata.bubs.len() equals to bub_frame_readers.len()?
    pub unsafe fn into_oao_frame_reader<B: Read + Clone, S: Sample>(
        self,
        bub_frame_readers: Vec<BubFrameReader<B, S>>,
        oao_spaces: Option<OaoSpaces>,
    ) -> OaoFrameReader<R, B, S> {
        OaoFrameReader::new(
            self.inner,
            self.metadata,
            self.speakers_absolute_coord,
            bub_frame_readers,
            oao_spaces,
        )
    }

    // pub fn into_oao_frame_reader_kind<B: Read + Clone, S: Sample>(self, bub_frame_readers: Vec<BubFrameReader<B, S>>) -> OaoFrameReaderKind<R, B> {
    //     match self.metadata.lpcm_kind() {
    //         LpcmKind::F32LE => OaoFrameReaderKind::F32LE(OaoFrameReader::<R, f32>::new(
    //             self.inner,
    //             self.metadata,
    //             self.speakers_absolute_coord,
    //         )),
    //         LpcmKind::F64LE => OaoFrameReaderKind::F64LE(OaoFrameReader::<R, f64>::new(
    //             self.inner,
    //             self.metadata,
    //             self.speakers_absolute_coord,
    //         )),
    //     }
    // }
}

impl OaoReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(
        filename: P,
        speakers_absolute_coord: Vec<BubFnsCoord>,
    ) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader, speakers_absolute_coord)
    }
    // TODO: Add open_and_into_oao_frame_reader_kind method
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
