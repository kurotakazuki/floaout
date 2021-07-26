use crate::bub::{
    function::{parse, FunctionVariable},
    BubbleFrameReader, BubbleSampleKind, BubbleState, FunctionInterpreter,
};
use crate::io::{ReadExt, WriteExt};
use crate::wav::WavSample;
use crate::Sample;
use std::io::{Read, Result, Write};
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub struct BubbleSample<S: WavSample> {
    _phantom_sample: PhantomData<S>,
}

impl<S: WavSample> Sample for BubbleSample<S> {}

impl<S: WavSample> BubbleSample<S> {
    // pub fn with_starting_sample(next_head_frame: u64) -> Self {
    //     Self {
    //         next_head_frame,
    //         ..Default::default()
    //     }
    // }

    // pub fn increment_time(&mut self) {
    //     match self.bubble_state {
    //         BubbleState::Head | BubbleState::Normal => self.time += 1,
    //         _ => (),
    //     }
    // }

    // pub fn set_as_starting(&mut self) {
    //     // self = &mut (Self::default());
    //     self.bubble_state = BubbleState::Head;

    //     self.time = 0;
    //     self.tail_frame_is_0 = false;
    //     self.connected = false;
    //     self.ended = false;
    //     self.function_size = 0;
    //     self.function_string.clear();
    //     // self.cbub_functions.clear();
    //     self.tail_frame = 0;
    //     self.next_head_frame = 0;
    //     self.waveform_sample = 0.0.into();
    // }

    // pub fn set_as_stopped(&mut self) {
    //     // self = &mut (Self::default());
    //     self.bubble_state = BubbleState::Stopped;

    //     self.time = 0;
    //     self.tail_frame_is_0 = false;
    //     self.connected = false;
    //     self.ended = false;
    //     self.function_size = 0;
    //     self.function_string.clear();
    //     // self.cbub_functions.clear();
    //     self.tail_frame = 0;
    //     self.next_head_frame = 0;
    //     self.waveform_sample = 0.0.into();
    // }

    // pub fn set_as_ended(&mut self) {
    //     // self = &mut (Self::default());
    //     self.bubble_state = BubbleState::Ended;

    //     self.time = 0;
    //     self.tail_frame_is_0 = false;
    //     self.connected = false;
    //     self.ended = false;
    //     self.function_size = 0;
    //     self.function_string.clear();
    //     // self.cbub_functions.clear();
    //     self.tail_frame = 0;
    //     self.next_head_frame = 0;
    //     self.waveform_sample = 0.0.into();
    // }

    // // TODO Error Handling
    // pub fn init_with_pos(&mut self, pos: u64) -> BubbleState {
    //     match self.bubble_state {
    //         BubbleState::Head => {
    //             if self.time == self.tail_frame {
    //                 if self.connected {
    //                     self.set_as_starting();
    //                 } else if self.ended {
    //                     self.set_as_ended();
    //                 } else {
    //                     self.set_as_stopped();
    //                 }
    //             } else {
    //                 self.bubble_state = BubbleState::Normal;
    //             }
    //         }
    //         BubbleState::Normal => {
    //             if self.time == self.tail_frame {
    //                 if self.connected {
    //                     self.set_as_starting();
    //                 } else if self.ended {
    //                     self.set_as_ended();
    //                 } else {
    //                     self.set_as_stopped();
    //                 }
    //             }
    //         }
    //         BubbleState::Stopped => {
    //             if self.next_head_frame == pos - 1 {
    //                 self.set_as_starting();
    //             }
    //         }
    //         BubbleState::Ended => (),
    //     };

    //     self.bubble_state
    // }

    pub fn read_flags_and_function_size<R: Read>(
        &mut self,
        reader: &mut BubbleFrameReader<R, S>,
    ) -> Result<u16> {
        let mut read_flags_and_function_size: u16 = reader.inner.read_le()?;

        // connected
        reader.metadata.connected = if read_flags_and_function_size & (1 << 15) != 0 {
            read_flags_and_function_size &= 0x7FFF;
            true
        } else {
            false
        };

        // ended
        reader.metadata.connected = if read_flags_and_function_size & (1 << 14) != 0 {
            read_flags_and_function_size &= 0xBFFF;
            true
        } else {
            false
        };

        Ok(read_flags_and_function_size)
    }

    pub fn read<R: Read>(
        &mut self,
        reader: &mut BubbleFrameReader<R, S>,
        speaker_absolute_coordinates: (f64, f64, f64),
    ) -> Result<S> {
        match reader.metadata.bubble_state {
            BubbleState::Head => {
                let function_size = self.read_flags_and_function_size(reader)?;

                let bubble_functions_vec = reader.inner.read_vec_for(function_size as usize)?;
                reader.metadata.bubble_functions =
                    parse(&bubble_functions_vec, &FunctionVariable::BubbleFunctions)
                        .unwrap()
                        .into_original()
                        .unwrap()
                        .into_bubble_functions()
                        .unwrap();

                reader.metadata.tail_frame = reader.inner.read_le()?;

                if !(reader.metadata.connected || reader.metadata.ended) {
                    reader.metadata.next_head_frame = reader.inner.read_le()?;
                }

                // Read Sample
                match reader.metadata.bubble_sample_kind {
                    BubbleSampleKind::LPCM => S::read(&mut reader.inner),
                    BubbleSampleKind::Expression => todo!(),
                }
            }
            BubbleState::Normal => {
                if reader.metadata.tail_frame == reader.pos {}

                // Read Sample
                match reader.metadata.bubble_sample_kind {
                    BubbleSampleKind::LPCM => S::read(&mut reader.inner),
                    BubbleSampleKind::Expression => todo!(),
                }
            }
            BubbleState::Stopped => Ok(S::default()),
            BubbleState::Ended => Ok(S::default()),
        }
    }

    // pub fn write_flags_and_function_size<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
    //     let mut read_connected_and_ended_and_function_size = self.function_size;

    //     // tail_frame_is_0
    //     if self.tail_frame_is_0 {
    //         read_connected_and_ended_and_function_size |= 1 << 15;
    //     }

    //     // connected
    //     if self.connected {
    //         read_connected_and_ended_and_function_size |= 1 << 14;
    //     }

    //     // ended
    //     if self.ended {
    //         read_connected_and_ended_and_function_size |= 1 << 13;
    //     }

    //     writer.write_le(read_connected_and_ended_and_function_size)?;

    //     Ok(())
    // }

    // pub fn write_cbub_sample<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
    //     match self.bubble_state {
    //         BubbleState::Head => {
    //             self.write_flags_and_function_size(writer)?;

    //             writer.write_str(&self.function_string)?;

    //             // TODO ast

    //             if !self.tail_frame_is_0 {
    //                 writer.write_le(self.tail_frame)?;
    //             }

    //             if !(self.connected || self.ended) {
    //                 writer.write_le(self.next_head_frame)?;
    //             }

    //             self.waveform_sample.write_waveform_sample(writer)?;
    //         }
    //         BubbleState::Normal => {
    //             self.waveform_sample.write_waveform_sample(writer)?;
    //         }
    //         BubbleState::Stopped => (),
    //         BubbleState::Ended => (),
    //     }

    //     Ok(())
    // }
}
