use crate::wav::WavMetadata;
use crate::{Frame, FrameIOKind, FrameReader, Sample};
use std::io::{Read, Result};
use std::marker::PhantomData;

pub struct WavFrameReader<R: Read, S: Sample> {
    pub inner: R,
    pub metadata: WavMetadata,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
}

impl<R: Read, S: Sample> FrameReader<R> for WavFrameReader<R, S> {
    fn get_ref(&self) -> &R {
        &self.inner
    }
    fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
    fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read, S: Sample> WavFrameReader<R, S> {
    pub fn new(inner: R, metadata: WavMetadata) -> Self {
        Self {
            inner,
            metadata,
            pos: 0,
            _phantom_sample: PhantomData,
        }
    }
}

impl<R: Read, S: Sample> Iterator for WavFrameReader<R, S> {
    type Item = Result<Frame<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        let mut buf: Vec<S> = Vec::with_capacity(self.metadata.channels() as usize);

        for _ in 0..self.metadata.channels() as usize {
            let wav_sample = S::read(self.get_mut());
            match wav_sample {
                Ok(s) => buf.push(s),
                Err(e) => return Some(Err(e)),
            }
        }

        Some(Ok(buf.into()))
    }
}

pub type WavFrameReaderKind<R> = FrameIOKind<WavFrameReader<R, f32>, WavFrameReader<R, f64>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LpcmKind;

    #[test]
    fn read() {
        macro_rules! test_read_wav {
            ( $( $t:ty ),* ) => ($(
                let lpcm_kind =LpcmKind::from_format_tag_and_bits_per_sample(3, (std::mem::size_of::<$t>() * 8) as u16);
                let channels = 1;
                let samples_per_sec = 44100;


                let data: Vec<u8> = Vec::new();
                let metadata = WavMetadata {
                        frames: 0,
                       lpcm_kind,
                        channels,
                        samples_per_sec,
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
                   lpcm_kind,
                    channels,
                    samples_per_sec,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap().0, vec![0.5]);
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());

                let data: Vec<u8> = vec![<$t>::to_le_bytes(0.0), <$t>::to_le_bytes(1.0)]
                    .into_iter()
                    .flatten()
                    .collect();
                let metadata = WavMetadata {
                    frames: 2,
                   lpcm_kind,
                    channels,
                    samples_per_sec,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap().0, vec![0.0]);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap().0, vec![1.0]);
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());


                let channels = 2;

                let metadata = WavMetadata {
                    frames: 2,
                   lpcm_kind,
                    channels,
                    samples_per_sec,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap().0, vec![0.0, 1.0]);
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
                   lpcm_kind,
                    channels,
                    samples_per_sec,
                };
                let mut wav_frame_reader: WavFrameReader<&[u8], $t> = WavFrameReader::new(&data[..], metadata);
                assert_eq!(wav_frame_reader.next().unwrap().unwrap().0, vec![0.0, 1.0]);
                assert!(wav_frame_reader.next().unwrap().is_err());
                assert!(wav_frame_reader.next().is_none());
                assert!(wav_frame_reader.next().is_none());
            )*)
        }

        test_read_wav!(f32, f64);
    }
}
