use crate::bub::{BubFnsBlock, BubMetadata, BubbleSample};
use crate::io::WriteExt;
use crate::utils::write_crc;
use crate::{FrameIOKind, FrameWriter, Sample};
use mycrc::CRC;
use std::io::{Error, ErrorKind, Result, Write};
use std::marker::PhantomData;

pub struct BubFrameWriter<W: Write, S: Sample> {
    pub inner: W,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
    pub metadata: BubMetadata,
    /// CRC
    pub crc: CRC<u32>,
}

impl<W: Write, S: Sample> FrameWriter<W> for BubFrameWriter<W, S> {
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
    fn get_ref(&self) -> &W {
        &self.inner
    }
    fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
    fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write, S: Sample> BubFrameWriter<W, S> {
    pub fn new(inner: W, metadata_and_crc: (BubMetadata, CRC<u32>)) -> Self {
        let pos = 0;

        Self {
            inner,
            pos,
            _phantom_sample: PhantomData,
            metadata: metadata_and_crc.0,
            crc: metadata_and_crc.1,
        }
    }

    fn write_crc(&mut self) -> Result<()> {
        write_crc(&mut self.inner, &mut self.crc)
    }

    fn write_head_metadata_and_calc_bytes(
        &mut self,
        head_absolute_frame: u64,
        bub_fns: &[u8],
        foot_relative_frame: u64,
        next_head_relative_frame: u64,
    ) -> Result<()> {
        self.metadata.set_as_head(head_absolute_frame);
        self.metadata.foot_absolute_frame_plus_one = head_absolute_frame + foot_relative_frame;
        // functions size
        self.inner
            .write_le_and_calc_bytes(bub_fns.len() as u16, &mut self.crc)?;
        // Bubble Functions
        self.inner.write_all(bub_fns)?;
        self.crc.calc_bytes(bub_fns);
        // Foot Relative Frame
        self.inner
            .write_le_and_calc_bytes(foot_relative_frame, &mut self.crc)?;
        // Next head relative frame
        self.inner
            .write_le_and_calc_bytes(next_head_relative_frame, &mut self.crc)?;
        self.metadata.set_next_head_absolute_frame_from_relative(
            next_head_relative_frame,
            head_absolute_frame,
        );

        Ok(())
    }

    fn write_sample_and_calc_bytes(&mut self, bub_sample: BubbleSample<S>) -> Result<()> {
        match bub_sample {
            BubbleSample::LpcmHead {
                head_absolute_frame,
                bub_fns,
                foot_relative_frame,
                next_head_relative_frame,
                sample,
            } => {
                self.write_head_metadata_and_calc_bytes(
                    head_absolute_frame,
                    bub_fns,
                    foot_relative_frame,
                    next_head_relative_frame.unwrap_or(0),
                )?;

                sample.write_and_calc_bytes(&mut self.inner, &mut self.crc)?;
            }
            BubbleSample::LpcmBody(sample) => {
                self.metadata.set_as_body();
                sample.write_and_calc_bytes(&mut self.inner, &mut self.crc)?;
            }
            BubbleSample::Expr {
                head_absolute_frame,
                bub_fns,
                foot_relative_frame,
                next_head_relative_frame,
                expression,
            } => {
                self.write_head_metadata_and_calc_bytes(
                    head_absolute_frame,
                    bub_fns,
                    foot_relative_frame,
                    next_head_relative_frame.unwrap_or(0),
                )?;
                // Write Expr
                self.inner
                    .write_le_and_calc_bytes(expression.len() as u16, &mut self.crc)?;
                self.inner.write_all(expression)?;
                self.crc.calc_bytes(expression);
            }
        }

        Ok(())
    }

    fn add_pos_to_less_than_next_head_or_ended(
        &mut self,
        head_absolute_frame: u64,
        foot_relative_frame: u64,
        next_head_relative_frame: Option<u64>,
    ) -> Result<()> {
        match next_head_relative_frame {
            Some(next_head_relative_frame) => {
                if foot_relative_frame < next_head_relative_frame {
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
                if self.metadata.frames() <= head_absolute_frame + foot_relative_frame {
                    return Err(ErrorKind::InvalidData.into());
                }
                self.pos = self.metadata.frames();
            }
        }

        Ok(())
    }

    pub fn write_head_to_less_than_next_head_or_ended(
        &mut self,
        bub_fns_block: BubFnsBlock<S>,
    ) -> Result<()> {
        if self.metadata.frames() <= self.pos {
            return Err(ErrorKind::InvalidData.into());
        }

        let head_absolute_frame = self.pos + 1;

        match bub_fns_block {
            BubFnsBlock::Lpcm {
                bub_fns,
                next_head_relative_frame,
                samples,
            } => {
                // Check if samples have Head frame sample.
                if samples.is_empty() {
                    return Err(Error::new(ErrorKind::Other, "samples must not be empty"));
                }

                let foot_relative_frame = samples.len() as u64;

                self.add_pos_to_less_than_next_head_or_ended(
                    head_absolute_frame,
                    foot_relative_frame,
                    next_head_relative_frame,
                )?;

                // Write Head
                self.write_sample_and_calc_bytes(BubbleSample::LpcmHead {
                    head_absolute_frame,
                    bub_fns,
                    foot_relative_frame,
                    next_head_relative_frame,
                    sample: samples[0],
                })?;
                // Write Body
                for sample in samples.into_iter().skip(1) {
                    self.write_sample_and_calc_bytes(BubbleSample::LpcmBody(sample))?;
                }
                // Write CRC
                self.write_crc()?;
                // Finalize
                self.metadata
                    .set_bub_state_from_connected_and_ended(self.pos);
            }
            BubFnsBlock::Expr {
                bub_fns,
                foot_relative_frame,
                next_head_relative_frame,
                expression,
            } => {
                // TODO: Is expression valid?

                self.add_pos_to_less_than_next_head_or_ended(
                    head_absolute_frame,
                    foot_relative_frame,
                    next_head_relative_frame,
                )?;
                // Write Expr
                self.write_sample_and_calc_bytes(BubbleSample::Expr {
                    head_absolute_frame,
                    bub_fns,
                    foot_relative_frame,
                    next_head_relative_frame,
                    expression,
                })?;

                // Write CRC
                self.write_crc()?;
                // Finalize
                self.metadata
                    .set_bub_state_from_connected_and_ended(self.pos);
            }
        }

        Ok(())
    }

    // pub fn write_frame(&mut self, bub_sample: BubbleSample<S>) -> Result<()> {
    //     let bub_state = self.metadata.next_pos_bub_state(self.pos);
    //     match bub_sample {
    //         BubbleSample::Head { .. } => {
    //             if bub_state != BubState::Head {
    //                 return Err(ErrorKind::InvalidData.into());
    //             }
    //         }
    //         BubbleSample::Body(_) => {
    //             if bub_state != BubState::Body {
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

    //     self.write_sample_and_calc_bytes(bub_sample)?;

    //     Ok(())
    // }
}

pub type BubFrameWriterKind<W> = FrameIOKind<BubFrameWriter<W, f32>, BubFrameWriter<W, f64>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::BubSampleKind;
    use crate::LpcmKind;

    #[test]
    fn write_lpcm_frames() -> Result<()> {
        let metadata = BubMetadata::new(
            8,
            1,
            96000.0,
            LpcmKind::F32LE,
            BubSampleKind::Lpcm,
            String::from("0.1*N"),
        );

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
        let crc = metadata.write(&mut skip).unwrap();

        let vec = Vec::new();

        let mut bub_frame_writer = BubFrameWriter::<Vec<u8>, f32>::new(vec, (metadata, crc));

        // BubFnsBlock 1
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"1 2 3 X<3 0.1*N",
            next_head_relative_frame: Some(3),
            samples: vec![1.0, 1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubFnsBlock 2
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"1 2 3 X<3 1",
            next_head_relative_frame: Some(3),
            samples: vec![0.3],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubFnsBlock 3
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 0 0==0 1",
            next_head_relative_frame: Some(2),
            samples: vec![0.4],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(2),
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0, 1.0, 1.0, 1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        // BubFnsBlock 4
        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: None,
            samples: vec![1.0],
        };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(lpcm)?;

        assert_eq!(&bub_frame_writer.inner[..], data);

        let lpcm = BubFnsBlock::Lpcm {
            bub_fns: b"0 0 n X>=3 -z",
            next_head_relative_frame: Some(5),
            samples: vec![1.0],
        };
        assert!(bub_frame_writer
            .write_head_to_less_than_next_head_or_ended(lpcm)
            .is_err());

        Ok(())
    }
}
