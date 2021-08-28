use crate::bub::{BubFrameReader, BubFrameReaderKind, BubMetadata};
use crate::{Coord, LpcmKind, OaoSpaces, Sample};
use mycrc::CRC;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct BubReader<R: Read> {
    pub inner: R,
    pub metadata: BubMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coord: Vec<Coord>,
    /// CRC
    pub crc: CRC<u32>,
}

impl<R: Read> BubReader<R> {
    pub fn new(mut inner: R, speakers_absolute_coord: Vec<Coord>) -> Result<Self> {
        let metadata_and_crc = BubMetadata::read(&mut inner)?;

        Ok(Self {
            inner,
            metadata: metadata_and_crc.0,
            speakers_absolute_coord,
            crc: metadata_and_crc.1,
        })
    }

    /// # Safety
    ///
    /// This is unsafe, due to the type of sample isn’t checked:
    /// - type of sample must follow [`LpcmKind`]
    pub unsafe fn into_bub_frame_reader<S: Sample>(
        self,
        oao_spaces: Option<OaoSpaces>,
    ) -> BubFrameReader<R, S> {
        BubFrameReader::new(
            self.inner,
            (self.metadata, self.crc),
            self.speakers_absolute_coord,
            oao_spaces,
        )
    }

    pub fn into_bub_frame_reader_kind(
        self,
        oao_spaces: Option<OaoSpaces>,
    ) -> BubFrameReaderKind<R> {
        match self.metadata.lpcm_kind() {
            LpcmKind::F32LE => BubFrameReaderKind::F32LE(BubFrameReader::<R, f32>::new(
                self.inner,
                (self.metadata, self.crc),
                self.speakers_absolute_coord,
                oao_spaces,
            )),
            LpcmKind::F64LE => BubFrameReaderKind::F64LE(BubFrameReader::<R, f64>::new(
                self.inner,
                (self.metadata, self.crc),
                self.speakers_absolute_coord,
                oao_spaces,
            )),
        }
    }
}

impl BubReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(filename: P, speakers_absolute_coord: Vec<Coord>) -> Result<Self> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader, speakers_absolute_coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{BubSampleKind, BubState::*};

    #[test]
    fn open() {
        let bub_reader = BubReader::open("tests/lpcm_test.bub", Vec::new()).unwrap();

        let metadata = BubMetadata::new(
            8,
            1,
            96000.0,
            LpcmKind::F32LE,
            BubSampleKind::Lpcm,
            String::from("0.1*N"),
        );

        assert_eq!(bub_reader.metadata, metadata);
    }

    #[test]
    fn read_lpcm_frames() -> std::io::Result<()> {
        let speakers_absolute_coord = vec![(0.0, 0.0, 0.0).into(), (3.0, 0.0, 0.0).into()];

        let bub_reader =
            BubReader::open("tests/lpcm_test.bub", speakers_absolute_coord.clone()).unwrap();
        assert!(bub_reader
            .into_bub_frame_reader_kind(None)
            .into_f64_le()
            .is_err());
        let bub_reader = BubReader::open("tests/lpcm_test.bub", speakers_absolute_coord).unwrap();
        let mut bub_frame_reader = bub_reader.into_bub_frame_reader_kind(None).into_f32_le()?;

        let expects = vec![
            (Head, [0.1, 0.0]),
            (Body, [0.2, 0.0]),
            (Head, [0.3, 0.0]),
            (Stopped, [0.0, 0.0]),
            (Head, [0.4, 0.4]),
            (Head, [0.0, 1.0]),
            (Ended, [0.0, 0.0]),
            (Ended, [0.0, 0.0]),
        ];

        for expect in expects {
            let frame = bub_frame_reader.next().unwrap().unwrap();
            assert_eq!(bub_frame_reader.metadata.bub_state, expect.0);
            assert_eq!(frame.0, expect.1);
        }

        assert!(bub_frame_reader.next().is_none());

        Ok(())
    }

    #[test]
    fn read_expr_frames() -> std::io::Result<()> {
        let speakers_absolute_coord = vec![(0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into()];

        let bub_reader =
            BubReader::open("tests/expr_test.bub", speakers_absolute_coord.clone()).unwrap();
        assert!(bub_reader
            .into_bub_frame_reader_kind(None)
            .into_f32_le()
            .is_err());
        let bub_reader = BubReader::open("tests/expr_test.bub", speakers_absolute_coord).unwrap();
        let mut bub_frame_reader = bub_reader.into_bub_frame_reader_kind(None).into_f64_le()?;

        let expects = vec![
            (Stopped, [0.0, 0.0]),
            (Head, [0.0, 0.1]),
            (Stopped, [0.0, 0.0]),
            (Head, [0.0, 1.0]),
            (Body, [0.0, 0.5]),
            (Head, [0.1, 0.0]),
            (Ended, [0.0, 0.0]),
            (Ended, [0.0, 0.0]),
        ];

        for expect in expects {
            let frame = bub_frame_reader.next().unwrap().unwrap();
            assert_eq!(bub_frame_reader.metadata.bub_state, expect.0);
            assert_eq!(frame.0, expect.1);
        }

        assert!(bub_frame_reader.next().is_none());

        Ok(())
    }
}
