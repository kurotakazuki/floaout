use crate::wav::{WavFrameReader, WavFrames, WavMetadata, WavSampleKind};
use crate::Metadata;
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

    pub fn into_wav_frames(self) -> WavFrames<R> {
        match self.metadata.wav_sample_kind() {
            WavSampleKind::F32LE => {
                WavFrames::F32(WavFrameReader::<R, f32>::new(self.inner, self.metadata))
            }
            WavSampleKind::F64LE => {
                WavFrames::F64(WavFrameReader::<R, f64>::new(self.inner, self.metadata))
            }
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
    fn open_wav_reader() {
        let wav_reader = WavReader::open("sample.wav").unwrap();

        let metadata = WavMetadata {
            frames: 176400,
            wav_sample_kind: WavSampleKind::F32LE,
            channels: 2,
            samples_per_sec: 44100,
        };

        assert_eq!(wav_reader.metadata, metadata);
    }
}
