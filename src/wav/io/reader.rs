use crate::wav::{WavFrameReader, WavFrames, WavMetadata, WavSample};
use crate::{Metadata, Sample};
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
        match self.metadata.bits_per_sample() {
            32 => WavFrames::F32(WavFrameReader::<R, f32>::new(self.inner, self.metadata)),
            64 => WavFrames::F64(WavFrameReader::<R, f64>::new(self.inner, self.metadata)),
            _ => unreachable!(),
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
    use crate::wav::FormatTag;

    #[test]
    fn open_wav_reader() {
        let wav_reader = WavReader::open("sample.wav").unwrap();

        let metadata = WavMetadata {
            frames: 176400,
            format_tag: FormatTag::IEEEFloatingPoint,
            channels: 2,
            samples_per_sec: 44100,
            bits_per_sample: 32,
        };

        assert_eq!(wav_reader.metadata, metadata);
    }
}
