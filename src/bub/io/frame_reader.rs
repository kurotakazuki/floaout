use crate::bub::{
    function::{parse, FunctionVariable},
    BubbleMetadata, BubbleSampleKind, BubbleState,
};
use crate::io::ReadExt;
use crate::{Frame, FrameReader, LpcmKind, Sample};
use std::io::{Error, ErrorKind, Read, Result};

pub type BubbleFrameReader<R, S> = FrameReader<R, BubbleMetadata, S>;

impl<R: Read, S: Sample> BubbleFrameReader<R, S> {
    fn read_flags_functions_size_and_calc_bytes(&mut self) -> Result<u16> {
        let mut read_flags_and_functions_size: u16 =
            self.inner.read_le_and_calc_bytes(&mut self.metadata.crc)?;

        // connected
        self.metadata.connected = if read_flags_and_functions_size & (1 << 15) != 0 {
            read_flags_and_functions_size &= 0x7FFF;
            true
        } else {
            false
        };

        // ended
        self.metadata.ended = if read_flags_and_functions_size & (1 << 14) != 0 {
            read_flags_and_functions_size &= 0xBFFF;
            true
        } else {
            false
        };

        Ok(read_flags_and_functions_size)
    }

    fn read_head_metadata_and_calc_bytes(&mut self) -> Result<()> {
        let functions_size = self.read_flags_functions_size_and_calc_bytes()?;

        let bubble_functions_vec = self
            .inner
            .read_vec_for_and_calc_bytes(functions_size as usize, &mut self.metadata.crc)?;

        self.metadata.bubble_functions =
            parse(&bubble_functions_vec, &FunctionVariable::BubbleFunctions)
                .unwrap()
                .into_original()
                .unwrap()
                .into_bubble_functions()
                .unwrap();
        // Tail relative frame
        self.metadata.tail_absolute_frame_plus_one = self.pos
            + self
                .inner
                .read_le_and_calc_bytes::<u64>(&mut self.metadata.crc)?;
        // Next head relative frame
        if !(self.metadata.connected || self.metadata.ended) {
            self.metadata.next_head_frame = self.pos
                + self
                    .inner
                    .read_le_and_calc_bytes::<u64>(&mut self.metadata.crc)?
                - 1;
        }

        Ok(())
    }

    fn multiply_volume(&self, frame: &mut Frame<S>, sample: S) {
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
                    frame.0[i] = sample * S::from_f64(volume);
                }
            }
        }
    }
}

impl<R: Read, S: Sample> Iterator for BubbleFrameReader<R, S> {
    type Item = Result<Frame<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        self.metadata.init_with_pos(self.pos);

        let channels = self.metadata.speakers_absolute_coordinates.len();

        let mut frame: Frame<S> = vec![S::default(); channels].into();

        match self.metadata.bubble_state {
            BubbleState::Head => {
                if let Err(e) = self.read_head_metadata_and_calc_bytes() {
                    return Some(Err(e));
                }

                // Read Sample
                let sample: S = match self.metadata.bubble_sample_kind {
                    BubbleSampleKind::Lpcm => {
                        match S::read_and_calc_bytes(&mut self.inner, &mut self.metadata.crc) {
                            Ok(n) => n,
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    BubbleSampleKind::Expression => todo!(),
                };
                self.multiply_volume(&mut frame, sample);
            }
            BubbleState::Normal => {
                // Read Sample
                let sample: S = match self.metadata.bubble_sample_kind {
                    BubbleSampleKind::Lpcm => {
                        match S::read_and_calc_bytes(&mut self.inner, &mut self.metadata.crc) {
                            Ok(n) => n,
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    BubbleSampleKind::Expression => todo!(),
                };
                self.multiply_volume(&mut frame, sample);
            }
            BubbleState::Stopped => (),
            BubbleState::Ended => (),
        }
        // Read CRC
        if self.metadata.tail_absolute_frame_plus_one - 1 == self.pos {
            if let Err(e) = self.metadata.read_crc(&mut self.inner) {
                return Some(Err(e));
            }
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
                    LpcmKind::F32LE,
                    r.metadata.lpcm_kind()
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
                    LpcmKind::F64LE,
                    r.metadata.lpcm_kind()
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
        let mut metadata = BubbleMetadata {
            spec_version: 0,
            bubble_id: BubbleID::new(0),
            bubble_version: 0,
            frames: 8,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::Lpcm,
            name: String::from("0.1*N"),

            speakers_absolute_coordinates: vec![(0.0, 0.0, 0.0), (3.0, 0.0, 0.0)],

            bubble_state: BubbleState::Stopped,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,

            crc: crate::crc::CRC,
        };

        // Write Metadata and get CRC
        let mut skip: Vec<u8> = Vec::new();
        metadata.write(&mut skip).unwrap();

        let data: &[u8] = &[
            // Frame 1
            &[15][..],
            &[0x80],
            b"1 2 3 X<3 0.1*N",
            &2u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            // Frame 2
            &1.0f32.to_le_bytes(),
            &[36, 239, 251, 84], // crc
            // Frame 3
            &[11],
            &[0],
            b"1 2 3 X<3 1",
            &1u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            &0.3f32.to_le_bytes(),
            &[111, 186, 119, 179], // crc
            // Frame 4

            // Frame 5
            &[12],
            &[0x80],
            b"0 0 0 0==0 1",
            &1u64.to_le_bytes(),
            &0.4f32.to_le_bytes(),
            &[56, 203, 203, 92], // crc
            // Frame 6
            &[13],
            &[0x40],
            b"0 0 n X>=3 -z",
            &1u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            &[249, 6, 139, 129], // crc
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
            assert_eq!(frame.0, expect.1);
        }
    }
}
