use crate::bub::{BubbleFrameReader, BubbleFrameReaderKind, BubbleMetadata};
use crate::{Coord, LpcmKind, Sample};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct BubbleReader<R: Read> {
    pub inner: R,
    pub metadata: BubbleMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coordinates: Vec<Coord>,
}

impl<R: Read> BubbleReader<R> {
    pub fn new(mut inner: R, speakers_absolute_coordinates: Vec<Coord>) -> Result<Self> {
        let metadata = BubbleMetadata::read(&mut inner)?;

        Ok(Self {
            inner,
            metadata,
            speakers_absolute_coordinates,
        })
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`SampleKind`]
    pub unsafe fn into_bub_frame_reader<S: Sample>(self) -> BubbleFrameReader<R, S> {
        BubbleFrameReader::new(
            self.inner,
            self.metadata,
            self.speakers_absolute_coordinates,
        )
    }

    pub fn into_bub_frame_reader_kind(self) -> BubbleFrameReaderKind<R> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => BubbleFrameReaderKind::F32LE(BubbleFrameReader::<R, f32>::new(
                self.inner,
                self.metadata,
                self.speakers_absolute_coordinates,
            )),
            LpcmKind::F64LE => BubbleFrameReaderKind::F64LE(BubbleFrameReader::<R, f64>::new(
                self.inner,
                self.metadata,
                self.speakers_absolute_coordinates,
            )),
        }
    }
}

impl BubbleReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(
        filename: P,
        speakers_absolute_coordinates: Vec<Coord>,
    ) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader, speakers_absolute_coordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{BubbleFunctions, BubbleID, BubbleSampleKind, BubbleState, BubbleState::*};

    #[test]
    fn open() {
        let mut bub_reader = BubbleReader::open("tests/lpcm_test.bub", Vec::new()).unwrap();

        let metadata = BubbleMetadata {
            spec_version: 0,
            bubble_id: BubbleID::new(0),
            bubble_version: 0,
            frames: 8,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::Lpcm,
            name: String::from("0.1*N"),

            bubble_state: BubbleState::Stopped,
            head_absolute_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            tail_absolute_frame_plus_one: 0,
            next_head_absolute_frame: Some(1),

            crc: crate::crc::CRC,
        };

        bub_reader.metadata.crc = crate::crc::CRC;

        assert_eq!(bub_reader.metadata, metadata);
    }

    #[test]
    fn read_lpcm_frames() -> std::io::Result<()> {
        let speakers_absolute_coordinates = vec![(0.0, 0.0, 0.0).into(), (3.0, 0.0, 0.0).into()];

        let bub_reader =
            BubbleReader::open("tests/lpcm_test.bub", speakers_absolute_coordinates.clone())
                .unwrap();
        assert!(bub_reader
            .into_bub_frame_reader_kind()
            .into_f64_le()
            .is_err());
        let bub_reader =
            BubbleReader::open("tests/lpcm_test.bub", speakers_absolute_coordinates).unwrap();
        let mut bub_frame_reader = bub_reader.into_bub_frame_reader_kind().into_f32_le()?;

        let expects = vec![
            (Head, [0.1, 0.0]),
            (Normal, [0.2, 0.0]),
            (Head, [0.3, 0.0]),
            (Stopped, [0.0, 0.0]),
            (Head, [0.4, 0.4]),
            (Head, [0.0, 1.0]),
            (Ended, [0.0, 0.0]),
            (Ended, [0.0, 0.0]),
        ];

        for expect in expects {
            let frame = bub_frame_reader.next().unwrap().unwrap();
            assert_eq!(bub_frame_reader.metadata.bubble_state, expect.0);
            assert_eq!(frame.0, expect.1);
        }

        assert!(bub_frame_reader.next().is_none());

        Ok(())
    }

    #[test]
    fn read_expr_frames() -> std::io::Result<()> {
        let speakers_absolute_coordinates = vec![(0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into()];

        let bub_reader =
            BubbleReader::open("tests/expr_test.bub", speakers_absolute_coordinates.clone())
                .unwrap();
        assert!(bub_reader
            .into_bub_frame_reader_kind()
            .into_f32_le()
            .is_err());
        let bub_reader =
            BubbleReader::open("tests/expr_test.bub", speakers_absolute_coordinates).unwrap();
        let mut bub_frame_reader = bub_reader.into_bub_frame_reader_kind().into_f64_le()?;

        let expects = vec![
            (Stopped, [0.0, 0.0]),
            (Head, [0.0, 0.1]),
            (Stopped, [0.0, 0.0]),
            (Head, [0.0, 1.0]),
            (Normal, [0.0, 0.5]),
            (Head, [0.1, 0.0]),
            (Ended, [0.0, 0.0]),
            (Ended, [0.0, 0.0]),
        ];

        for expect in expects {
            let frame = bub_frame_reader.next().unwrap().unwrap();
            assert_eq!(bub_frame_reader.metadata.bubble_state, expect.0);
            assert_eq!(frame.0, expect.1);
        }

        assert!(bub_frame_reader.next().is_none());

        Ok(())
    }
}
