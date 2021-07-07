use crate::wav::{WavMetadata, WavSample};
use crate::FrameReader;
use std::io::{Read, Result};

pub type WavFrameReader<R, S> = FrameReader<R, WavMetadata, S>;

impl<R: Read, S: WavSample> Iterator for WavFrameReader<R, S> {
    type Item = Result<Vec<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Vec<S> = Vec::with_capacity(self.metadata.channels() as usize);

        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        for _ in 0..self.metadata.channels() as usize {
            let wav_sample = S::read(self.get_mut());
            match wav_sample {
                Ok(s) => buf.push(s),
                Err(e) => return Some(Err(e)),
            }
        }

        Some(Ok(buf))
    }
}

pub enum WavFrames<R: Read> {
    F32(WavFrameReader<R, f32>),
    F64(WavFrameReader<R, f64>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wav::FormatTag;
    use std::mem::size_of;

    #[test]
    fn read() {
        macro_rules! test_read_wav {
            ( $( $t:ty ),* ) => ($(
                let format_tag = FormatTag::IEEEFloatingPoint;
                let channels = 1;
                let samples_per_sec = 44100;
                let bits_per_sample = (size_of::<$t>() * 8) as u16;


                let data: Vec<u8> = Vec::new();
                let metadata = WavMetadata {
                        frames: 0,
                        format_tag,
                        channels,
                        samples_per_sec,
                        bits_per_sample,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());

                let data: Vec<u8> = vec![<$t>::to_le_bytes(0.5)]
                    .into_iter()
                    .flatten()
                    .collect();
                let metadata = WavMetadata {
                    frames: 1,
                    format_tag,
                    channels,
                    samples_per_sec,
                    bits_per_sample,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.5]);
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());

                let data: Vec<u8> = vec![<$t>::to_le_bytes(0.0), <$t>::to_le_bytes(1.0)]
                    .into_iter()
                    .flatten()
                    .collect();
                let metadata = WavMetadata {
                    frames: 2,
                    format_tag,
                    channels,
                    samples_per_sec,
                    bits_per_sample,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.0]);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![1.0]);
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());


                let channels = 2;

                let metadata = WavMetadata {
                    frames: 2,
                    format_tag,
                    channels,
                    samples_per_sec,
                    bits_per_sample,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.0, 1.0]);
                assert!(wav_frame_reader.next().unwrap().is_err());
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());

                let data: Vec<u8> = vec![
                    <$t>::to_le_bytes(0.0),
                    <$t>::to_le_bytes(1.0),
                    <$t>::to_le_bytes(1.0),
                ]
                .into_iter()
                .flatten()
                .collect();
                let metadata = WavMetadata {
                    frames: 2,
                    format_tag,
                    channels,
                    samples_per_sec,
                    bits_per_sample,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.0, 1.0]);
                assert!(wav_frame_reader.next().unwrap().is_err());
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());
            )*)
        }

        test_read_wav!(f32, f64);
    }
}