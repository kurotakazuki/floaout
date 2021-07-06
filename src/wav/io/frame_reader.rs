use crate::io::ReadExt;
use crate::wav::{WavMetadata, WavSample};
use std::io::{Read, Result};
use std::marker::PhantomData;

pub struct WavFrameReader<R: Read, S: WavSample> {
    pub inner: R,
    pub metadata: WavMetadata<S>,
    pub pos: u32,
    phantom_sample: PhantomData<S>,
}

impl<R: Read, S: WavSample> WavFrameReader<R, S> {
    pub fn new(inner: R, metadata: WavMetadata<S>) -> Self {
        Self {
            inner,
            metadata,
            pos: 0,
            phantom_sample: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: ReadExt, S: WavSample> Iterator for WavFrameReader<R, S> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_data_by_wav_frame_reader() {
        macro_rules! test_read_wav {
            ( $( $t:ty ),* ) => ($(
            let data: Vec<u8> = Vec::new();
            let metadata = WavMetadata::<$t>::new(0, 1, 44100);
            let mut wav_frame_reader = WavFrameReader::new(&data[..], metadata);
            assert!(wav_frame_reader.next().is_none());
            assert!(wav_frame_reader.next().is_none());

            let data: Vec<u8> = vec![<$t>::to_le_bytes(0.5)]
                .into_iter()
                .flatten()
                .collect();
            let metadata = WavMetadata::<$t>::new(1, 1, 44100);
            let mut wav_frame_reader = WavFrameReader::new(&data[..], metadata);
            assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.5]);
            assert!(wav_frame_reader.next().is_none());
            assert!(wav_frame_reader.next().is_none());

            let data: Vec<u8> = vec![<$t>::to_le_bytes(0.0), <$t>::to_le_bytes(1.0)]
                .into_iter()
                .flatten()
                .collect();
            let metadata = WavMetadata::<$t>::new(2, 1, 44100);
            let mut wav_frame_reader = WavFrameReader::new(&data[..], metadata);
            assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.0]);
            assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![1.0]);
            assert!(wav_frame_reader.next().is_none());
            assert!(wav_frame_reader.next().is_none());

            let metadata = WavMetadata::<$t>::new(2, 2, 44100);
            let mut wav_frame_reader = WavFrameReader::new(&data[..], metadata);
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
            let metadata = WavMetadata::<$t>::new(2, 2, 44100);
            let mut wav_frame_reader = WavFrameReader::new(&data[..], metadata);
            assert_eq!(wav_frame_reader.next().unwrap().unwrap(), vec![0.0, 1.0]);
            assert!(wav_frame_reader.next().unwrap().is_err());
            assert!(wav_frame_reader.next().is_none());
            assert!(wav_frame_reader.next().is_none());
            )*)
        }

        test_read_wav!(f32, f64);
    }
}
