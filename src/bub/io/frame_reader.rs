use crate::bub::{BubbleMetadata, BubbleSample};
use crate::wav::{WavFrame, WavSample};
use crate::{FrameReader, SampleKind};
use std::io::{Error, ErrorKind, Read, Result};

pub type BubbleFrameReader<R, S> = FrameReader<R, BubbleMetadata, S>;

impl<R: Read, S: WavSample> Iterator for BubbleFrameReader<R, S> {
    type Item = Result<WavFrame<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        let channels = self.metadata.speakers_absolute_coordinates.len();

        let mut buf: WavFrame<S> = Vec::with_capacity(channels);

        for i in 0..channels as usize {
            let bub_sample =
                BubbleSample::read(self, self.metadata.speakers_absolute_coordinates[i]);

            match bub_sample {
                Ok(s) => buf.push(s),
                Err(e) => return Some(Err(e)),
            }
        }

        Some(Ok(buf))
    }
}

pub enum BubbleFrameReaderKind<R: Read> {
    F32LE(BubbleFrameReader<R, f32>),
    F64LE(BubbleFrameReader<R, f64>),
}

impl<R: Read> From<BubbleFrameReader<R, f32>> for BubbleFrameReaderKind<R> {
    fn from(r: BubbleFrameReader<R, f32>) -> Self {
        Self::F32LE(r)
    }
}

impl<R: Read> From<BubbleFrameReader<R, f64>> for BubbleFrameReaderKind<R> {
    fn from(r: BubbleFrameReader<R, f64>) -> Self {
        Self::F64LE(r)
    }
}

impl<R: Read> BubbleFrameReaderKind<R> {
    pub fn into_f32_le(self) -> Result<BubbleFrameReader<R, f32>> {
        match self {
            Self::F32LE(r) => Ok(r),
            Self::F64LE(r) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    SampleKind::F32LE,
                    r.metadata.sample_kind()
                ),
            )),
        }
    }

    pub fn into_f64_le(self) -> Result<BubbleFrameReader<R, f64>> {
        match self {
            Self::F32LE(r) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    SampleKind::F64LE,
                    r.metadata.sample_kind()
                ),
            )),
            Self::F64LE(r) => Ok(r),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn read() {
//         macro_rules! test_read_bub {
//             ( $( $t:ty ),* ) => ($(
//                 let bub_sample_kind = SampleKind::from_format_tag_and_bits_per_sample(3, (std::mem::size_of::<$t>() * 8) as u16);
//                 let channels = 1;
//                 let samples_per_sec = 44100;

//                 let data: Vec<u8> = Vec::new();
//                 let metadata = BubbleMetadata {
//                         frames: 0,
//                         bub_sample_kind,
//                         channels,
//                         samples_per_sec,
//                 };
//                 let mut bub_frame_reader: BubbleFrameReader<&[u8], $t> = BubbleFrameReader::new(&data[..], metadata);
//                 assert!(bub_frame_reader.next().is_none());
//                 assert!(bub_frame_reader.next().is_none());

//                 let data: Vec<u8> = vec![<$t>::to_le_bytes(0.5)]
//                     .into_iter()
//                     .flatten()
//                     .collect();
//                 let metadata = BubbleMetadata {
//                     frames: 1,
//                     bub_sample_kind,
//                     channels,
//                     samples_per_sec,
//                 };
//                 let mut bub_frame_reader: BubbleFrameReader<&[u8], $t> = BubbleFrameReader::new(&data[..], metadata);
//                 assert_eq!(bub_frame_reader.next().unwrap().unwrap(), vec![0.5]);
//                 assert!(bub_frame_reader.next().is_none());
//                 assert!(bub_frame_reader.next().is_none());

//                 let data: Vec<u8> = vec![<$t>::to_le_bytes(0.0), <$t>::to_le_bytes(1.0)]
//                     .into_iter()
//                     .flatten()
//                     .collect();
//                 let metadata = BubbleMetadata {
//                     frames: 2,
//                     bub_sample_kind,
//                     channels,
//                     samples_per_sec,
//                 };
//                 let mut bub_frame_reader: BubbleFrameReader<&[u8], $t> = BubbleFrameReader::new(&data[..], metadata);
//                 assert_eq!(bub_frame_reader.next().unwrap().unwrap(), vec![0.0]);
//                 assert_eq!(bub_frame_reader.next().unwrap().unwrap(), vec![1.0]);
//                 assert!(bub_frame_reader.next().is_none());
//                 assert!(bub_frame_reader.next().is_none());

//                 let channels = 2;

//                 let metadata = BubbleMetadata {
//                     frames: 2,
//                     bub_sample_kind,
//                     channels,
//                     samples_per_sec,
//                 };
//                 let mut bub_frame_reader: BubbleFrameReader<&[u8], $t> = BubbleFrameReader::new(&data[..], metadata);
//                 assert_eq!(bub_frame_reader.next().unwrap().unwrap(), vec![0.0, 1.0]);
//                 assert!(bub_frame_reader.next().unwrap().is_err());
//                 assert!(bub_frame_reader.next().is_none());
//                 assert!(bub_frame_reader.next().is_none());

//                 let data: Vec<u8> = vec![
//                     <$t>::to_le_bytes(0.0),
//                     <$t>::to_le_bytes(1.0),
//                     <$t>::to_le_bytes(1.0),
//                 ]
//                 .into_iter()
//                 .flatten()
//                 .collect();
//                 let metadata = BubbleMetadata {
//                     frames: 2,
//                     bub_sample_kind,
//                     channels,
//                     samples_per_sec,
//                 };
//                 let mut bub_frame_reader: BubbleFrameReader<&[u8], $t> = BubbleFrameReader::new(&data[..], metadata);
//                 assert_eq!(bub_frame_reader.next().unwrap().unwrap(), vec![0.0, 1.0]);
//                 assert!(bub_frame_reader.next().unwrap().is_err());
//                 assert!(bub_frame_reader.next().is_none());
//                 assert!(bub_frame_reader.next().is_none());
//             )*)
//         }

//         test_read_bub!(f32, f64);
//     }
// }
