use crate::bub::{BubbleFunctionsBlock, BubbleMetadata, BubbleSample};
use crate::io::WriteExt;
use crate::{FrameWriter, LpcmKind, Sample};
use std::io::{Error, ErrorKind, Result, Write};

pub type BubbleFrameWriter<W, S> = FrameWriter<W, BubbleMetadata, S>;

impl<W: Write, S: Sample> BubbleFrameWriter<W, S> {
    fn write_flags_functions_size_and_calc_bytes(&mut self, functions_size: u16) -> Result<()> {
        let mut read_flags_and_functions_size = functions_size;

        // connected
        if self.metadata.connected {
            read_flags_and_functions_size |= 1 << 15;
        }
        // ended
        if self.metadata.ended {
            read_flags_and_functions_size |= 1 << 14;
        }

        self.inner
            .write_le_and_calc_bytes(read_flags_and_functions_size, &mut self.metadata.crc)?;

        Ok(())
    }

    fn write_sample_and_calc_bytes(&mut self, bubble_sample: BubbleSample<S>) -> Result<()> {
        match bubble_sample {
            BubbleSample::LpcmHead {
                head_absolute_frame,
                connected,
                ended,
                bubble_functions,
                tail_relative_frame,
                next_head_relative_frame,
                sample,
            } => {
                self.metadata.set_as_head(head_absolute_frame);
                self.metadata.connected = connected;
                self.metadata.ended = ended;
                self.metadata.tail_absolute_frame_plus_one =
                    head_absolute_frame + tail_relative_frame;
                // Flags and functions
                self.write_flags_functions_size_and_calc_bytes(bubble_functions.len() as u16)?;
                // Bubble Functions
                self.inner.write_all(bubble_functions)?;
                self.metadata.crc.calc_bytes(bubble_functions);
                // Tail Relative Frame
                self.inner
                    .write_le_and_calc_bytes(tail_relative_frame, &mut self.metadata.crc)?;
                // Next head relative frame
                if !(connected || ended) {
                    self.inner.write_le_and_calc_bytes(
                        next_head_relative_frame,
                        &mut self.metadata.crc,
                    )?;
                    self.metadata.next_head_frame =
                        head_absolute_frame + next_head_relative_frame - 1;
                }

                sample.write_and_calc_bytes(&mut self.inner, &mut self.metadata.crc)?;
            }
            BubbleSample::LpcmNormal(sample) => {
                self.metadata.set_as_normal();
                sample.write_and_calc_bytes(&mut self.inner, &mut self.metadata.crc)?;
            }
            BubbleSample::Expression { .. } => todo!(),
        }

        Ok(())
    }

    pub fn write_head_to_less_than_next_head_or_ended(
        &mut self,
        bubble_functions_block: BubbleFunctionsBlock<S>,
    ) -> Result<()> {
        match bubble_functions_block {
            BubbleFunctionsBlock::Lpcm {
                connected,
                ended,
                bubble_functions,
                next_head_relative_frame,
                samples,
            } => {
                let tail_relative_frame = samples.len() as u64;

                // Check if samples have Head frame sample.
                if samples.is_empty() {
                    return Err(ErrorKind::InvalidData.into());
                }

                if self.metadata.frames() <= self.pos {
                    return Err(ErrorKind::InvalidData.into());
                }

                let head_absolute_frame = self.pos + 1;

                if ended {
                    // Ended
                    if self.metadata.frames() <= head_absolute_frame + tail_relative_frame {
                        return Err(ErrorKind::InvalidData.into());
                    }
                    self.pos = self.metadata.frames();
                } else if connected {
                    if self.metadata.frames() < head_absolute_frame + tail_relative_frame {
                        return Err(ErrorKind::InvalidData.into());
                    }
                    self.pos += tail_relative_frame;
                } else {
                    let next_head_relative_frame_minus_one = next_head_relative_frame - 1;
                    // Check if next_head_relative_frame is valid.
                    if tail_relative_frame >= next_head_relative_frame_minus_one {
                        return Err(ErrorKind::InvalidData.into());
                    }
                    // Stopped
                    if self.metadata.frames()
                        < head_absolute_frame + next_head_relative_frame_minus_one
                    {
                        return Err(ErrorKind::InvalidData.into());
                    }
                    self.pos += next_head_relative_frame_minus_one;
                }

                // Write Head
                self.write_sample_and_calc_bytes(BubbleSample::LpcmHead {
                    head_absolute_frame,
                    connected,
                    ended,
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
                if ended {
                    self.metadata.set_as_ended();
                } else if !connected {
                    self.metadata.set_as_stopped();
                }
            }
            _ => todo!(),
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
    fn write_frames() -> Result<()> {
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
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,

            crc: crate::crc::CRC,
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
            &[194, 54, 74, 24], // crc
            // Frame 3
            &[11],
            &[0],
            b"1 2 3 X<3 1",
            &1u64.to_le_bytes(),
            &3u64.to_le_bytes(),
            &0.3f32.to_le_bytes(),
            &[142, 47, 230, 233], // crc
            // Frame 4

            // Frame 5
            &[12],
            &[0x80],
            b"0 0 0 0==0 1",
            &1u64.to_le_bytes(),
            &0.4f32.to_le_bytes(),
            &[16, 85, 195, 62], // crc
            // Frame 6
            &[13],
            &[0x40],
            b"0 0 n X>=3 -z",
            &1u64.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            &[175, 72, 19, 72], // crc
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
            connected: true,
            ended: false,
            bubble_functions: b"1 2 3 X<3 0.1*N",
            next_head_relative_frame: 0,
            samples: vec![1.0, 1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: false,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 5,
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: false,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 5,
            samples: vec![],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 2
        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: false,
            bubble_functions: b"1 2 3 X<3 1",
            next_head_relative_frame: 3,
            samples: vec![0.3],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: false,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 5,
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 3
        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: true,
            ended: false,
            bubble_functions: b"0 0 0 0==0 1",
            next_head_relative_frame: 0,
            samples: vec![0.4],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: true,
            ended: false,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 0,
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: true,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 0,
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubbleFunctionsBlock 4
        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: true,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 0,
            samples: vec![1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        assert_eq!(&bub_frame_writer.inner[..], data);

        let lpcm = BubbleFunctionsBlock::Lpcm {
            connected: false,
            ended: false,
            bubble_functions: b"0 0 n X>=3 -z",
            next_head_relative_frame: 5,
            samples: vec![1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        Ok(())
    }
}
