use crate::bub::{
    function::{parse, FunctionVariable},
    BubbleMetadata, BubbleSampleKind, BubbleState,
};
use crate::io::ReadExt;
use crate::wav::{WavFrame, WavSample};
use crate::{FrameReader, SampleKind};
use std::io::{Error, ErrorKind, Read, Result};

pub type BubbleFrameReader<R, S> = FrameReader<R, BubbleMetadata, S>;

impl<R: Read, S: WavSample> BubbleFrameReader<R, S> {
    fn read_flags_and_function_size(&mut self) -> Result<u16> {
        let mut read_flags_and_function_size: u16 = self.inner.read_le()?;

        // connected
        self.metadata.connected = if read_flags_and_function_size & (1 << 15) != 0 {
            read_flags_and_function_size &= 0x7FFF;
            true
        } else {
            false
        };

        // ended
        self.metadata.ended = if read_flags_and_function_size & (1 << 14) != 0 {
            read_flags_and_function_size &= 0xBFFF;
            true
        } else {
            false
        };

        Ok(read_flags_and_function_size)
    }

    fn read_head_metadata(&mut self) -> Result<()> {
        let function_size = self.read_flags_and_function_size()?;

        let bubble_functions_vec = self.inner.read_vec_for(function_size as usize)?;

        self.metadata.bubble_functions =
            parse(&bubble_functions_vec, &FunctionVariable::BubbleFunctions)
                .unwrap()
                .into_original()
                .unwrap()
                .into_bubble_functions()
                .unwrap();

        self.metadata.tail_absolute_frame_plus_one = self.pos + self.inner.read_le::<u64>()?;

        if !(self.metadata.connected || self.metadata.ended) {
            self.metadata.next_head_frame = self.pos + self.inner.read_le::<u64>()? - 1;
        }

        Ok(())
    }
}

impl<R: Read, S: WavSample> Iterator for BubbleFrameReader<R, S> {
    type Item = Result<WavFrame<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        self.metadata.init_with_pos(self.pos);

        let channels = self.metadata.speakers_absolute_coordinates.len();

        let mut frame: WavFrame<S> = vec![S::default(); channels];

        match self.metadata.bubble_state {
            BubbleState::Head => {
                if let Err(e) = self.read_head_metadata() {
                    return Some(Err(e));
                }

                // Read Sample
                let sample: S = match self.metadata.bubble_sample_kind {
                    BubbleSampleKind::LPCM => match S::read(&mut self.inner) {
                        Ok(n) => n,
                        Err(e) => return Some(Err(e)),
                    },
                    BubbleSampleKind::Expression => todo!(),
                };
                if sample != S::default() {
                    for (i, speaker_absolute_coordinates) in self
                        .metadata
                        .speakers_absolute_coordinates
                        .iter()
                        .enumerate()
                    {
                        if let Some(volume) = self.metadata.bubble_functions.to_volume(
                            *speaker_absolute_coordinates,
                            self.pos as f64,
                            (self.pos - self.metadata.head_frame + 1) as f64,
                            self.metadata.frames as f64,
                            self.metadata.samples_per_sec,
                        ) {
                            frame[i] = sample * S::from_f64(volume);
                        }
                    }
                }
            }
            BubbleState::Normal => {
                // Read Sample
                let sample: S = match self.metadata.bubble_sample_kind {
                    BubbleSampleKind::LPCM => match S::read(&mut self.inner) {
                        Ok(n) => n,
                        Err(e) => return Some(Err(e)),
                    },
                    BubbleSampleKind::Expression => todo!(),
                };
                if sample != S::default() {
                    for (i, speaker_absolute_coordinates) in self
                        .metadata
                        .speakers_absolute_coordinates
                        .iter()
                        .enumerate()
                    {
                        if let Some(volume) = self.metadata.bubble_functions.to_volume(
                            *speaker_absolute_coordinates,
                            self.pos as f64,
                            (self.pos - self.metadata.head_frame + 1) as f64,
                            self.metadata.frames as f64,
                            self.metadata.samples_per_sec,
                        ) {
                            frame[i] = sample * S::from_f64(volume);
                        }
                    }
                }
            }
            BubbleState::Stopped => (),
            BubbleState::Ended => (),
        }

        Some(Ok(frame))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{
        function::BubbleFunctions, BubbleID, BubbleSampleKind, BubbleState, BubbleState::*,
    };

    #[test]
    fn read_frames() {
        let metadata = BubbleMetadata {
            starting_sample: 0,
            version: 0,
            bubble_id: BubbleID::new(0),
            frames: 8,
            samples_per_sec: 96000.0,
            sample_kind: SampleKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::LPCM,
            name: String::from("0.1*N"),

            speakers_absolute_coordinates: vec![(0.0, 0.0, 0.0), (3.0, 0.0, 0.0)],

            bubble_state: BubbleState::Stopped,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,
        };

        let data: &[u8] = &[
            // Frame 1
            &[15][..],
            &[0x80],
            b"1 2 3 X<3 0.1*N",
            &2u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            // Frame 2
            &1.0f32.to_le_bytes(),
            // Frame 3
            &[11],
            &[0],
            b"1 2 3 X<3 1",
            &1u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            &0.3f32.to_le_bytes(),
            // Frame 4

            // Frame 5
            &[12],
            &[0x80],
            b"0 0 0 0==0 1",
            &1u64.to_le_bytes(),
            &0.4f32.to_le_bytes(),
            // Frame 6
            &[13],
            &[0x40],
            b"0 0 n X>=3 -z",
            &1u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            // Frame 7

            // Frame 8
        ]
        .concat();

        let mut wav_frame_reader: BubbleFrameReader<&[u8], f32> =
            BubbleFrameReader::new(data, metadata);

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
            let frame = wav_frame_reader.next().unwrap().unwrap();
            assert_eq!(wav_frame_reader.metadata.bubble_state, expect.0);
            assert_eq!(frame, expect.1);
        }
    }
}
