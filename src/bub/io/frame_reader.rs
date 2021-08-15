use crate::bub::{
    function::{parse, FunctionAST, FunctionInterpreter, FunctionVariable},
    BubbleMetadata, BubbleSampleKind, BubbleState,
};
use crate::io::ReadExt;
use crate::{Frame, FrameReader, LpcmKind, Sample};
use std::io::{Error, ErrorKind, Read, Result};
use std::marker::PhantomData;

pub struct BubbleFrameReader<R: Read, S: Sample> {
    pub inner: R,
    pub metadata: BubbleMetadata,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
}

impl<R: Read, S: Sample> FrameReader<R> for BubbleFrameReader<R, S> {
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

impl<R: Read, S: Sample> BubbleFrameReader<R, S> {
    pub fn new(inner: R, metadata: BubbleMetadata) -> Self {
        Self {
            inner,
            metadata,
            pos: 0,
            _phantom_sample: PhantomData,
        }
    }

    fn read_head_metadata_and_calc_bytes(&mut self) -> Result<()> {
        let functions_size: u16 = self.inner.read_le_and_calc_bytes(&mut self.metadata.crc)?;

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
        self.metadata
            .read_next_head_absolute_frame_from_relative(&mut self.inner, self.pos)?;

        Ok(())
    }

    fn read_lpcm_and_crc(&mut self) -> Result<S> {
        let sample = S::read_and_calc_bytes(&mut self.inner, &mut self.metadata.crc)?;
        // Read CRC
        if self.metadata.tail_absolute_frame_plus_one - 1 == self.pos {
            self.metadata.read_crc(&mut self.inner)?;
        }

        Ok(sample)
    }

    fn read_expression_and_crc(&mut self) -> Result<Vec<u8>> {
        let expr_size: u16 = self.inner.read_le_and_calc_bytes(&mut self.metadata.crc)?;
        let expr = self
            .inner
            .read_vec_for_and_calc_bytes(expr_size as usize, &mut self.metadata.crc)?;
        // CRC
        self.metadata.read_crc(&mut self.inner)?;
        Ok(expr)
    }

    fn get_volume_and_interpreter(
        &self,
        speaker_absolute_coordinates: (f64, f64, f64),
    ) -> Option<(f64, FunctionInterpreter)> {
        self.metadata.bubble_functions.to_volume(
            speaker_absolute_coordinates,
            self.pos as f64,
            (self.pos - self.metadata.head_absolute_frame + 1) as f64,
            self.metadata.frames as f64,
            self.metadata.samples_per_sec,
        )
    }

    fn read_lpcm_frame(&mut self, frame: &mut Frame<S>) -> Result<()> {
        let sample = self.read_lpcm_and_crc()?;

        if sample != S::default() {
            // TODO: Create method
            for (i, speaker_absolute_coordinates) in self
                .metadata
                .speakers_absolute_coordinates
                .iter()
                .enumerate()
            {
                if let Some((volume, _)) =
                    self.get_volume_and_interpreter(*speaker_absolute_coordinates)
                {
                    frame.0[i] = sample * S::from_f64(volume);
                }
            }
        }

        Ok(())
    }

    fn expr_frame(&self, expr: &FunctionAST, frame: &mut Frame<S>) {
        for (i, speaker_absolute_coordinates) in self
            .metadata
            .speakers_absolute_coordinates
            .iter()
            .enumerate()
        {
            if let Some((volume, interpreter)) =
                self.get_volume_and_interpreter(*speaker_absolute_coordinates)
            {
                let sample = interpreter.eval_sum(expr).unwrap();
                frame.0[i] = S::from_f64(sample * volume);
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
                match self.metadata.bubble_sample_kind {
                    BubbleSampleKind::Lpcm => {
                        if let Err(e) = self.read_lpcm_frame(&mut frame) {
                            return Some(Err(e));
                        }
                    }
                    BubbleSampleKind::Expression(_) => {
                        let expr = match self.read_expression_and_crc() {
                            Ok(v) => v,
                            Err(e) => return Some(Err(e)),
                        };
                        let expr = parse(&expr, &FunctionVariable::Sum).unwrap();
                        self.expr_frame(&expr, &mut frame);
                        self.metadata.bubble_sample_kind = expr.into();
                    }
                }
            }
            BubbleState::Normal => {
                // Read Sample
                match &self.metadata.bubble_sample_kind {
                    BubbleSampleKind::Lpcm => {
                        if let Err(e) = self.read_lpcm_frame(&mut frame) {
                            return Some(Err(e));
                        }
                    }
                    BubbleSampleKind::Expression(expr) => self.expr_frame(expr, &mut frame),
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
    fn read_lpcm_frames() {
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
            head_absolute_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            tail_absolute_frame_plus_one: 0,
            next_head_absolute_frame: Some(1),

            crc: crate::crc::CRC,
        };

        // Write Metadata and get CRC
        let mut skip: Vec<u8> = Vec::new();
        metadata.write(&mut skip).unwrap();

        let data: &[u8] = &[
            // Frame 1
            &15u16.to_le_bytes()[..],
            b"1 2 3 X<3 0.1*N",
            &2u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            // Frame 2
            &1.0f32.to_le_bytes(),
            &[253, 24, 123, 85], // crc
            // Frame 3
            &11u16.to_le_bytes()[..],
            b"1 2 3 X<3 1",
            &1u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            &0.3f32.to_le_bytes(),
            &[250, 147, 10, 142], // crc
            // Frame 4

            // Frame 5
            &12u16.to_le_bytes()[..],
            b"0 0 0 0==0 1",
            &1u64.to_le_bytes(),
            &2u64.to_le_bytes(),
            &0.4f32.to_le_bytes(),
            &[84, 232, 255, 6], // crc
            // Frame 6
            &13u16.to_le_bytes()[..],
            b"0 0 n X>=3 -z",
            &1u64.to_le_bytes(),
            &0u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            &[227, 183, 22, 42], // crc
                                 // Frame 7

                                 // Frame 8
        ]
        .concat();

        let mut bub_frame_reader: BubbleFrameReader<&[u8], f32> =
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
            let frame = bub_frame_reader.next().unwrap().unwrap();
            assert_eq!(bub_frame_reader.metadata.bubble_state, expect.0);
            assert_eq!(frame.0, expect.1);
        }
    }

    #[test]
    fn read_expr_frames() {
        let mut metadata = BubbleMetadata {
            spec_version: 0,
            bubble_id: BubbleID::new(0),
            bubble_version: 0,
            frames: 8,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F64LE,
            bubble_sample_kind: BubbleSampleKind::default_expr(),
            name: String::from("Expression"),

            speakers_absolute_coordinates: vec![(0.0, 0.0, 0.0), (0.0, 0.0, 1.0)],

            bubble_state: BubbleState::Stopped,
            head_absolute_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            tail_absolute_frame_plus_one: 0,
            next_head_absolute_frame: Some(2),

            crc: crate::crc::CRC,
        };

        // Write Metadata and get CRC
        let mut skip: Vec<u8> = Vec::new();
        metadata.write(&mut skip).unwrap();

        let data: &[u8] = &[
            // Frame 1

            // Frame 2
            &14u16.to_le_bytes()[..],
            b"1 2 3 Z==1 0.1",
            &1u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            // Expr
            &1u16.to_le_bytes(),
            b"1",
            &[17, 247, 225, 70], // crc
            // Frame 3

            // Frame 4
            &12u16.to_le_bytes()[..],
            b"1 2 3 Z==1 1",
            &2u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            // Expr
            &3u16.to_le_bytes(),
            b"1/n",
            &[48, 94, 190, 151], // crc
            // Frame 5

            // Frame 6
            &11u16.to_le_bytes()[..],
            b"1 2 3 Z<1 n",
            &1u64.to_le_bytes(),
            &0u64.to_le_bytes(),
            // Expr
            &3u16.to_le_bytes(),
            b"0.1",
            &[248, 137, 58, 64], // crc

                                 // Frame 7

                                 // Frame 8
        ]
        .concat();
        let mut bub_frame_reader: BubbleFrameReader<&[u8], f32> =
            BubbleFrameReader::new(data, metadata);

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
    }
}
