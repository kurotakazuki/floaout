use crate::wav::{WavFrameReader, WavFrameReaderKind, WavMetadata, WavSample};
use crate::{Metadata, SampleKind};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct WavReader<R: Read> {
    pub inner: R,
    pub metadata: WavMetadata,
}

impl<R: Read> WavReader<R> {
    pub fn new(mut inner: R) -> Result<Self> {
        let metadata = WavMetadata::read(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`SampleKind`]
    pub unsafe fn into_wav_frame_reader<S: WavSample>(self) -> WavFrameReader<R, S> {
        WavFrameReader::new(self.inner, self.metadata)
    }

    pub fn into_wav_frame_reader_kind(self) -> WavFrameReaderKind<R> {
        match self.metadata.wav_sample_kind() {
            SampleKind::F32LE => WavFrameReader::<R, f32>::new(self.inner, self.metadata).into(),
            SampleKind::F64LE => WavFrameReader::<R, f64>::new(self.inner, self.metadata).into(),
        }
    }
}

impl WavReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open() {
        let wav_reader = WavReader::open("tests/sample.wav").unwrap();

        let metadata = WavMetadata {
            frames: 176400,
            wav_sample_kind: SampleKind::F32LE,
            channels: 2,
            samples_per_sec: 44100,
        };

        assert_eq!(wav_reader.metadata, metadata);
    }

    #[test]
    fn read_wav_frames() -> std::io::Result<()> {
        let wav_reader = WavReader::open("tests/sample.wav").unwrap();
        assert!(wav_reader
            .into_wav_frame_reader_kind()
            .into_f64_le()
            .is_err());
        let wav_reader = WavReader::open("tests/sample.wav").unwrap();
        let mut wav_frame_reader = wav_reader.into_wav_frame_reader_kind().into_f32_le()?;

        for _ in 0..176400 {
            if let Some(frame) = wav_frame_reader.next() {
                frame?;
            } else {
                panic!();
            }
        }

        Ok(())
    }
}
