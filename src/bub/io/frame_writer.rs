use crate::bub::{BubbleFunctionsBlock, BubbleMetadata, BubbleSample};
use crate::io::WriteExt;
use crate::{FrameWriter, LpcmKind, Sample};
use std::io::{Error, ErrorKind, Result, Write};

pub type BubbleFrameWriter<W, S> = FrameWriter<W, BubbleMetadata, S>;

impl<W: Write, S: Sample> BubbleFrameWriter<W, S> {
    fn write_head_metadata_and_calc_bytes(
        &mut self,
        head_absolute_frame: u64,
        bubble_functions: &[u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
    ) -> Result<()> {
        self.metadata.set_as_head(head_absolute_frame);
        self.metadata.tail_absolute_frame_plus_one = head_absolute_frame + tail_relative_frame;
        // functions size
        self.inner
            .write_le_and_calc_bytes(bubble_functions.len() as u16, &mut self.metadata.crc)?;
        // Bubble Functions
        self.inner.write_all(bubble_functions)?;
        self.metadata.crc.calc_bytes(bubble_functions);
        // Tail Relative Frame
        self.inner
            .write_le_and_calc_bytes(tail_relative_frame, &mut self.metadata.crc)?;
        // Next head relative frame
        self.inner
            .write_le_and_calc_bytes(next_head_relative_frame, &mut self.metadata.crc)?;
        self.metadata.set_next_head_absolute_frame_from_relative(
            next_head_relative_frame,
            head_absolute_frame,
        );

        Ok(())
    }

    fn write_sample_and_calc_bytes(&mut self, bubble_sample: BubbleSample<S>) -> Result<()> {
        match bubble_sample {
            BubbleSample::LpcmHead {
                head_absolute_frame,
                bubble_functions,
                tail_relative_frame,
                next_head_relative_frame,
                sample,
            } => {
                self.write_head_metadata_and_calc_bytes(
                    head_absolute_frame,
                    bubble_functions,
                    tail_relative_frame,
                    next_head_relative_frame.unwrap_or(0),
                )?;

                sample.write_and_calc_bytes(&mut self.inner, &mut self.metadata.crc)?;
            }
            BubbleSample::LpcmNormal(sample) => {
                self.metadata.set_as_normal();
                sample.write_and_calc_bytes(&mut self.inner, &mut self.metadata.crc)?;
            }
            BubbleSample::Expression {
                head_absolute_frame,
                bubble_functions,
                tail_relative_frame,
                next_head_relative_frame,
                expression,
            } => {
                self.write_head_metadata_and_calc_bytes(
                    head_absolute_frame,
                    bubble_functions,
                    tail_relative_frame,
                    next_head_relative_frame.unwrap_or(0),
                )?;
                // Write Expression
                self.inner
                    .write_le_and_calc_bytes(expression.len() as u16, &mut self.metadata.crc)?;
                self.inner.write_all(expression)?;
                self.metadata.crc.calc_bytes(expression);
            }
        }

        Ok(())
    }

    fn add_pos_to_less_than_next_head_or_ended(
        &mut self,
        head_absolute_frame: u64,
        tail_relative_frame: u64,
        next_head_relative_frame: Option<u64>,
    ) -> Result<()> {
        match next_head_relative_frame {
            Some(next_head_relative_frame) => {
                if tail_relative_frame < next_head_relative_frame {
                    let next_head_relative_frame_minus_one = next_head_relative_frame - 1;
                    if self.metadata.frames()
                        < head_absolute_frame + next_head_relative_frame_minus_one
                    {
                        return Err(ErrorKind::InvalidData.into());
                    }
                    self.pos += next_head_relative_frame_minus_one;
                } else {
                    return Err(ErrorKind::InvalidData.into());
                }
            }
            // Ended
            None => {
                if self.metadata.frames() <= head_absolute_frame + tail_relative_frame {
                    return Err(ErrorKind::InvalidData.into());
                }
                self.pos = self.metadata.frames();
            }
        }

        Ok(())
    }

    pub fn write_head_to_less_than_next_head_or_ended(
        &mut self,
        bubble_functions_block: BubbleFunctionsBlock<S>,
    ) -> Result<()> {
        if self.metadata.frames() <= self.pos {
            return Err(ErrorKind::InvalidData.into());
        }

        let head_absolute_frame = self.pos + 1;

        match bubble_functions_block {
            BubbleFunctionsBlock::Lpcm {
                bubble_functions,
                next_head_relative_frame,
                samples,
            } => {
                // Check if samples have Head frame sample.
                if samples.is_empty() {
                    return Err(ErrorKind::InvalidData.into());
                }

                let tail_relative_frame = samples.len() as u64;

                self.add_pos_to_less_than_next_head_or_ended(
                    head_absolute_frame,
                    tail_relative_frame,
                    next_head_relative_frame,
                )?;

                // Write Head
                self.write_sample_and_calc_bytes(BubbleSample::LpcmHead {
                    head_absolute_frame,
                    bubble_functions,
                    tail_relative_frame,
                    next_head_relative_frame,
                    sample: samples[0],
                })?;
                // Write Normal
                for sample in samples.into_iter().skip(1) {
                    self.write_sample_and_calc_bytes(BubbleSample::LpcmNormal(sample))?;
                }
                // Write CRC
                self.metadata.write_crc(&mut self.inner)?;
                // Finalize
                self.metadata
                    .set_bubble_state_from_connected_and_ended(self.pos);
            }
            BubbleFunctionsBlock::Expression {
                bubble_functions,
                tail_relative_frame,
                next_head_relative_frame,
                expression,
            } => {
                self.add_pos_to_less_than_next_head_or_ended(
                    head_absolute_frame,
                    tail_relative_frame,
                    next_head_relative_frame,
                )?;
                // Write Expression
                self.write_sample_and_calc_bytes(BubbleSample::Expression {
                    head_absolute_frame,
                    bubble_functions,
                    tail_relative_frame,
                    next_head_relative_frame,
                    expression,
                })?;

                // Write CRC
                self.metadata.write_crc(&mut self.inner)?;
                // Finalize
                self.metadata
                    .set_bubble_state_from_connected_and_ended(self.pos);
            }
        }

        Ok(())
    }

    // pub fn write_frame(&mut self, bubble_sample: BubbleSample<S>) -> Result<()> {
    //     let bubble_state = self.metadata.next_pos_bubble_state(self.pos);
    //     match bubble_sample {
    //         BubbleSample::Head { .. } => {
    //             if bubble_state != BubbleState::Head {
    //                 return Err(ErrorKind::InvalidData.into());
    //             }
    //         }
    //         BubbleSample::Normal(_) => {
    //             if bubble_state != BubbleState::Normal {
    //                 return Err(ErrorKind::InvalidData.into());
    //             }
    //         }
    //         _ => todo!(),
    //     }

    //     if self.metadata.frames() <= self.pos {
    //         return Err(ErrorKind::InvalidData.into());
    //     } else {
    //         self.pos += 1;
    //     }

    //     self.write_sample_and_calc_bytes(bubble_sample)?;

    //     Ok(())
    // }
}

pub enum BubbleFrameWriterKind<W: Write> {
    F32LE(BubbleFrameWriter<W, f32>),
    F64LE(BubbleFrameWriter<W, f64>),
}

impl<W: Write> From<BubbleFrameWriter<W, f32>> for BubbleFrameWriterKind<W> {
    fn from(w: BubbleFrameWriter<W, f32>) -> Self {
        Self::F32LE(w)
    }
}

impl<W: Write> From<BubbleFrameWriter<W, f64>> for BubbleFrameWriterKind<W> {
    fn from(w: BubbleFrameWriter<W, f64>) -> Self {
        Self::F64LE(w)
    }
}

impl<W: Write> BubbleFrameWriterKind<W> {
    pub fn into_f32_le(self) -> Result<BubbleFrameWriter<W, f32>> {
        match self {
            Self::F32LE(w) => Ok(w),
            Self::F64LE(w) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    LpcmKind::F32LE,
                    w.metadata.lpcm_kind()
                ),
            )),
        }
    }

    pub fn into_f64_le(self) -> Result<BubbleFrameWriter<W, f64>> {
        match self {
            Self::F32LE(w) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    LpcmKind::F64LE,
                    w.metadata.lpcm_kind()
                ),
            )),
            Self::F64LE(w) => Ok(w),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{function::BubbleFunctions, BubbleID, BubbleSampleKind, BubbleState};

    #[test]
    fn write_lpcm_frames() -> Result<()> {
        let mut metadata = BubbleMetadata {
            spec_version: 0,
            bubble_id: BubbleID::new(0),
            bubble_version: 0,
            frames: 8,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::Lpcm,
            name: String::from("0.1*N"),

            speakers_absolute_coordinates: vec![],

            bubble_state: BubbleState::Stopped,
            head_absolute_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            tail_absolute_frame_plus_one: 0,
            next_head_absolute_frame: Some(1),

            crc: crate::crc::CRC,
        };

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
        // Write Metadata
        let mut skip = Vec::new();
        metadata.write(&mut skip).unwrap();

        let vec = Vec::new();

        let mut bub_frame_writer = BubbleFrameWriter::<Vec<u8>, f32>::new(vec, metadata);

        // BubbleFunctionsBlock 1
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"1 2 3 X<3 0.1*N",
            next_head_relative_frame: Some(3),
            samples: vec![1.0, 1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 2
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"1 2 3 X<3 1",
            next_head_relative_frame: Some(3),
            samples: vec![0.3],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 3
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 0 0==0 1",
            next_head_relative_frame: Some(2),
            samples: vec![0.4],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(2),
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 4
        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: None,
            samples: vec![1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        assert_eq!(&bub_frame_writer.inner[..], data);

        let lpcm = BubbleFunctionsBlock::Lpcm {
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        Ok(())
    }
}
