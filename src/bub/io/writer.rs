use crate::bub::{BubFrameWriter, BubFrameWriterKind, BubMetadata};
use crate::{LpcmKind, Sample};
use mycrc::CRC;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub struct BubWriter<W: Write> {
    pub inner: W,
    pub metadata: BubMetadata,
    /// CRC
    pub crc: CRC<u32>,
}

impl<W: Write> BubWriter<W> {
    pub fn new(mut inner: W, metadata: BubMetadata) -> Result<Self> {
        let crc = metadata.write(&mut inner)?;

        Ok(Self {
            inner,
            metadata,
            crc,
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`LpcmKind`]
    pub unsafe fn into_bub_frame_writer<S: Sample>(self) -> BubFrameWriter<W, S> {
        BubFrameWriter::new(self.inner, (self.metadata, self.crc))
    }

    pub fn into_bub_frame_writer_kind(self) -> BubFrameWriterKind<W> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => BubFrameWriterKind::F32LE(BubFrameWriter::<W, f32>::new(
                self.inner,
                (self.metadata, self.crc),
            )),
            LpcmKind::F64LE => BubFrameWriterKind::F64LE(BubFrameWriter::<W, f64>::new(
                self.inner,
                (self.metadata, self.crc),
            )),
        }
    }
}

impl BubWriter<BufWriter<File>> {
    pub fn create<P: AsRef<Path>>(filename: P, metadata: BubMetadata) -> Result<Self> {
        let file = File::create(filename)?;
        let buf_writer = BufWriter::new(file);
        Self::new(buf_writer, metadata)
    }
}
