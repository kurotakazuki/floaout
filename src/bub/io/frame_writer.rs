use crate::bub::{BubbleMetadata, BubbleState};
use crate::io::WriteExt;
use crate::{Frame, FrameWriter, LPCMKind, Sample};
use std::io::{Error, ErrorKind, Result, Write};

pub type BubbleFrameWriter<W, S> = FrameWriter<W, BubbleMetadata, S>;

impl<W: Write, S: Sample> BubbleFrameWriter<W, S> {
    fn write_flags_and_function_size(
        &mut self,
        connected: bool,
        ended: bool,
        function_size: u16,
    ) -> Result<()> {
        let mut read_flags_and_function_size = function_size;

        // connected
        if connected {
            read_flags_and_function_size |= 1 << 15;
        }

        // ended
        if ended {
            read_flags_and_function_size |= 1 << 14;
        }

        self.inner.write_le(read_flags_and_function_size)?;

        Ok(())
    }

    pub fn write_sample(&mut self, bubble_sample: BubbleSample<S>) -> Result<()> {
        match bubble_sample {
            BubbleSample::Head {
                connected,
                ended,
                bubble_functions,
                tail_relative_frame,
                next_head_relative_frame,
                sample,
            } => {
                self.write_flags_and_function_size(
                    connected,
                    ended,
                    bubble_functions.len() as u16,
                )?;

                self.inner.write(bubble_functions)?;

                self.inner.write_le(tail_relative_frame)?;

                if !(connected || ended) {
                    self.inner.write_le(next_head_relative_frame)?;
                }

                sample.write(&mut self.inner)?;
            }
            BubbleSample::Normal(sample) => {
                sample.write(&mut self.inner)?;
            }
            BubbleSample::Expression { .. } => todo!(),
        }

        Ok(())
    }

    pub fn write_frame(&mut self, bubble_sample: BubbleSample<S>) -> Result<()> {
        // if bub_frame.0.len() != self.metadata.channels() as usize {
        //     return Err(ErrorKind::InvalidData.into());
        // }

        // if self.metadata.frames() <= self.pos {
        //     return Err(ErrorKind::InvalidData.into());
        // } else {
        //     self.pos += 1;
        // }

        // TODO: Check if the BubbleSample follows the metadata.

        self.write_sample(bubble_sample)?;

        Ok(())
    }
}

pub enum BubbleSample<'a, S: Sample> {
    // LPCM
    Head {
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
        sample: S,
    },
    Normal(S),
    // Expression
    // TODO
    Expression {
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
        expression: &'a [u8],
    },
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
                    LPCMKind::F32LE,
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
                    LPCMKind::F64LE,
                    w.metadata.lpcm_kind()
                ),
            )),
            Self::F64LE(w) => Ok(w),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn write() -> Result<()> {
//         let data: Vec<u8> = Vec::new();
//         let metadata = BubbleMetadata {
//             frames: 2,
//             lpcm_kind: LPCMKind::F32LE,
//             channels: 1,
//             samples_per_sec: 44100,
//         };
//         let mut bub_frame_writer = BubbleFrameWriter::<Vec<u8>, f32>::new(data, metadata);

//         bub_frame_writer.write_bub_frame(vec![1.0].into())?;
//         bub_frame_writer.write_bub_frame(vec![0.0].into())?;

//         assert_eq!(bub_frame_writer.get_ref(), &[0, 0, 0x80, 0x3F, 0, 0, 0, 0]);
//         assert!(bub_frame_writer.write_bub_frame(vec![0.0].into()).is_err());
//         assert!(bub_frame_writer.write_bub_frame(vec![0.0].into()).is_err());

//         Ok(())
//     }
// }
