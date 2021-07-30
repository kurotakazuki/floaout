use crate::bub::{BubbleFrameReader, BubbleFrameReaderKind, BubbleMetadata};
use crate::{LPCMKind, Metadata, Sample};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct BubbleReader<R: Read> {
    pub inner: R,
    pub metadata: BubbleMetadata,
}

impl<R: Read> BubbleReader<R> {
    pub fn new(mut inner: R) -> Result<Self> {
        let metadata = BubbleMetadata::read(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`SampleKind`]
    pub unsafe fn into_bub_frame_reader<S: Sample>(self) -> BubbleFrameReader<R, S> {
        BubbleFrameReader::new(self.inner, self.metadata)
    }

    pub fn into_bub_frame_reader_kind(self) -> BubbleFrameReaderKind<R> {
        match self.metadata.lpcm_kind() {
            LPCMKind::F32LE => BubbleFrameReader::<R, f32>::new(self.inner, self.metadata).into(),
            LPCMKind::F64LE => BubbleFrameReader::<R, f64>::new(self.inner, self.metadata).into(),
        }
    }
}

impl BubbleReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader)
    }
}

// TODO: Add tests
