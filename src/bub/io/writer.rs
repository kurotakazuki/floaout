use crate::bub::{BubbleFrameWriter, BubbleFrameWriterKind, BubbleMetadata};
use crate::{LPCMKind, Metadata, Sample};
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub struct BubbleWriter<W: Write> {
    pub inner: W,
    pub metadata: BubbleMetadata,
}

impl<W: Write> BubbleWriter<W> {
    pub fn new(mut inner: W, metadata: BubbleMetadata) -> Result<Self> {
        metadata.clone().write(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`SampleKind`]
    pub unsafe fn into_bub_frame_writer<S: Sample>(self) -> BubbleFrameWriter<W, S> {
        BubbleFrameWriter::new(self.inner, self.metadata)
    }

    pub fn into_bub_frame_writer_kind(self) -> BubbleFrameWriterKind<W> {
        match self.metadata.lpcm_kind() {
            LPCMKind::F32LE => BubbleFrameWriter::<W, f32>::new(self.inner, self.metadata).into(),
            LPCMKind::F64LE => BubbleFrameWriter::<W, f64>::new(self.inner, self.metadata).into(),
        }
    }
}

impl BubbleWriter<BufWriter<File>> {
    pub fn create<P: AsRef<Path>>(filename: P, metadata: BubbleMetadata) -> Result<Self> {
        let file = File::create(filename)?;
        let buf_writer = BufWriter::new(file);
        Self::new(buf_writer, metadata)
    }
}
