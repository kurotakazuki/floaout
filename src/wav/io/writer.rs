use crate::wav::{WavFrameWriter, WavFrameWriterKind, WavMetadata};
use crate::{LpcmKind, Sample};
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub struct WavWriter<W: Write> {
    pub inner: W,
    pub metadata: WavMetadata,
}

impl<W: Write> WavWriter<W> {
    pub fn new(mut inner: W, metadata: WavMetadata) -> Result<Self> {
        metadata.write(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`LpcmKind`]
    pub unsafe fn into_wav_frame_writer<S: Sample>(self) -> WavFrameWriter<W, S> {
        WavFrameWriter::new(self.inner, self.metadata)
    }

    pub fn into_wav_frame_writer_kind(self) -> WavFrameWriterKind<W> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => {
                WavFrameWriterKind::F32LE(WavFrameWriter::<W, f32>::new(self.inner, self.metadata))
            }
            LpcmKind::F64LE => {
                WavFrameWriterKind::F64LE(WavFrameWriter::<W, f64>::new(self.inner, self.metadata))
            }
            _ => unimplemented!(),
        }
    }
}

impl WavWriter<BufWriter<File>> {
    pub fn create<P: AsRef<Path>>(filename: P, metadata: WavMetadata) -> Result<Self> {
        let file = File::create(filename)?;
        let buf_writer = BufWriter::new(file);
        Self::new(buf_writer, metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wav::WavReader;

    #[test]
    fn read_and_write() -> std::io::Result<()> {
        let v = Vec::new();

        let wav_reader = WavReader::open("tests/test.wav")?;
        let wav_frame_reader = wav_reader.into_wav_frame_reader_kind().into_f32_le()?;

        let wav_writer = WavWriter::new(v, wav_frame_reader.metadata.clone())?;
        let mut wav_frame_writer = wav_writer.into_wav_frame_writer_kind().into_f32_le()?;

        for frame in wav_frame_reader {
            wav_frame_writer.write_frame(frame?)?;
        }

        assert_eq!(
            wav_frame_writer.inner,
            include_bytes!("./../../../tests/test.wav")
        );

        Ok(())
    }
}
