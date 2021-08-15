use crate::bub::{BubFrameWriter, BubFrameWriterKind, BubbleMetadata};
use crate::{LpcmKind, Sample};
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub struct BubWriter<W: Write> {
    pub inner: W,
    pub metadata: BubbleMetadata,
}

impl<W: Write> BubWriter<W> {
    pub fn new(mut inner: W, mut metadata: BubbleMetadata) -> Result<Self> {
        metadata.write(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`SampleKind`]
    pub unsafe fn into_bub_frame_writer<S: Sample>(self) -> BubFrameWriter<W, S> {
        BubFrameWriter::new(self.inner, self.metadata)
    }

    pub fn into_bub_frame_writer_kind(self) -> BubFrameWriterKind<W> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => {
                BubFrameWriterKind::F32LE(BubFrameWriter::<W, f32>::new(self.inner, self.metadata))
            }
            LpcmKind::F64LE => {
                BubFrameWriterKind::F64LE(BubFrameWriter::<W, f64>::new(self.inner, self.metadata))
            }
        }
    }
}

impl BubWriter<BufWriter<File>> {
    pub fn create<P: AsRef<Path>>(filename: P, metadata: BubbleMetadata) -> Result<Self> {
        let file = File::create(filename)?;
        let buf_writer = BufWriter::new(file);
        Self::new(buf_writer, metadata)
    }
}
