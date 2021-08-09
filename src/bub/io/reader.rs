use crate::bub::{BubbleFrameReader, BubbleFrameReaderKind, BubbleMetadata};
use crate::{LpcmKind, Metadata, Sample};
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
            LpcmKind::F32LE => BubbleFrameReader::<R, f32>::new(self.inner, self.metadata).into(),
            LpcmKind::F64LE => BubbleFrameReader::<R, f64>::new(self.inner, self.metadata).into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{BubbleFunctions, BubbleID, BubbleSampleKind, BubbleState};

    #[test]
    fn open() {
        let bub_reader = BubbleReader::open("tests/test.bub").unwrap();

        let metadata = BubbleMetadata {
            version: 0,
            bubble_id: BubbleID::new(0),
            frames: 8,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bubble_lpcm_kind: BubbleSampleKind::LPCM,
            name: String::from("0.1*N"),

            speakers_absolute_coordinates: vec![],

            bubble_state: BubbleState::Stopped,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,
        };

        assert_eq!(bub_reader.metadata, metadata);
    }

    #[test]
    fn read_bub_frames() -> std::io::Result<()> {
        let bub_reader = BubbleReader::open("tests/test.bub").unwrap();
        assert!(bub_reader
            .into_bub_frame_reader_kind()
            .into_f64_le()
            .is_err());
        let bub_reader = BubbleReader::open("tests/test.bub").unwrap();
        let mut bub_frame_reader = bub_reader.into_bub_frame_reader_kind().into_f32_le()?;

        for _ in 0..bub_frame_reader.metadata.frames() {
            if let Some(frame) = bub_frame_reader.next() {
                frame?;
            } else {
                panic!();
            }
        }

        assert!(bub_frame_reader.next().is_none());

        Ok(())
    }
}
