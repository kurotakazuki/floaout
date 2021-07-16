// use crate::Sample;
// use crate::bub::{FunctionAST, function::parse, FunctionInterpreter, BubbleFrameReader};
// use crate::io::{ReadExt, WriteExt};
// use std::io::{Read, Write, Result};

// #[derive(Clone, Debug, PartialEq)]
// pub struct BubbleSample {
//     /// Bubble State
//     pub bubble_state: BubbleState,
//     /// Absolute Time
//     pub absolute_time: u64,
//     /// Connected or Not Flag
//     /// | Value | Contents |
//     /// | ---------------- |
//     /// | 0 | Not Connected |
//     /// | 1 | Connected |
//     pub connected: bool,
//     /// Ended or Not Flag
//     /// | Value | Contents |
//     /// | ---------------- |
//     /// | 0 | Not Ended |
//     /// | 1 | Ended |
//     pub ended: bool,
//     /// Functions
//     pub functions: Vec<(FunctionAST, FunctionAST)>,
//     // TODO
//     // /// Bubble Functions
//     // /// TODO Error handling, cbub_functions must not be empty.
//     // pub cbub_functions: Vec<CbubFunction>,
//     /// Ending Time which means "Sample length"
//     /// If `self.ending_time_is_0` is `true', this won't exist.
//     pub ending_time: u64,
//     /// Next Starting Sample
//     /// If `self.connected` is `true', this won't exist.
//     /// If `self.ended` is `true', this won't exist.
//     pub next_starting_sample: u64,
// }

// impl Sample for BubbleSample {}

// impl CbubSample {
//     pub fn with_starting_sample(next_starting_sample: u64) -> Self {
//         Self {
//             next_starting_sample,
//             ..Default::default()
//         }
//     }

//     pub fn increment_time(&mut self) {
//         match self.bubble_state {
//             BubbleState::Starting | BubbleState::Normal => self.time += 1,
//             _ => (),
//         }
//     }

//     pub fn set_as_starting(&mut self) {
//         // self = &mut (Self::default());
//         self.bubble_state = BubbleState::Starting;

//         self.time = 0;
//         self.ending_time_is_0 = false;
//         self.connected = false;
//         self.ended = false;
//         self.function_size = 0;
//         self.function_string.clear();
//         // self.cbub_functions.clear();
//         self.ending_time = 0;
//         self.next_starting_sample = 0;
//         self.waveform_sample = 0.0.into();
//     }

//     pub fn set_as_stopped(&mut self) {
//         // self = &mut (Self::default());
//         self.bubble_state = BubbleState::Stopped;

//         self.time = 0;
//         self.ending_time_is_0 = false;
//         self.connected = false;
//         self.ended = false;
//         self.function_size = 0;
//         self.function_string.clear();
//         // self.cbub_functions.clear();
//         self.ending_time = 0;
//         self.next_starting_sample = 0;
//         self.waveform_sample = 0.0.into();
//     }

//     pub fn set_as_ended(&mut self) {
//         // self = &mut (Self::default());
//         self.bubble_state = BubbleState::Ended;

//         self.time = 0;
//         self.ending_time_is_0 = false;
//         self.connected = false;
//         self.ended = false;
//         self.function_size = 0;
//         self.function_string.clear();
//         // self.cbub_functions.clear();
//         self.ending_time = 0;
//         self.next_starting_sample = 0;
//         self.waveform_sample = 0.0.into();
//     }

//     // TODO Error Handling
//     pub fn init_with_pos(&mut self, pos: u64) -> BubbleState {
//         match self.bubble_state {
//             BubbleState::Starting => {
//                 if self.time == self.ending_time {
//                     if self.connected {
//                         self.set_as_starting();
//                     } else if self.ended {
//                         self.set_as_ended();
//                     } else {
//                         self.set_as_stopped();
//                     }
//                 } else {
//                     self.bubble_state = BubbleState::Normal;
//                 }
//             }
//             BubbleState::Normal => {
//                 if self.time == self.ending_time {
//                     if self.connected {
//                         self.set_as_starting();
//                     } else if self.ended {
//                         self.set_as_ended();
//                     } else {
//                         self.set_as_stopped();
//                     }
//                 }
//             }
//             BubbleState::Stopped => {
//                 if self.next_starting_sample == pos - 1 {
//                     self.set_as_starting();
//                 }
//             }
//             BubbleState::Ended => (),
//         };

//         self.bubble_state
//     }

//     // TODO Use function or macro
//     pub fn read_flags_and_function_size<R: ReadExt>(&mut self, reader: &mut R) -> Result<()> {
//         let mut read_flags_and_function_size = reader.read_le()?;

//         // ending_time_is_0
//         if read_flags_and_function_size & (1 << 15) != 0 {
//             read_flags_and_function_size &= 0x7FFF;
//             self.ending_time_is_0 = true;
//         } else {
//             self.ending_time_is_0 = false;
//         }

//         // connected
//         if read_flags_and_function_size & (1 << 14) != 0 {
//             read_flags_and_function_size &= 0xBFFF;
//             self.connected = true;
//         } else {
//             self.connected = false;
//         }

//         // ended
//         if read_flags_and_function_size & (1 << 13) != 0 {
//             read_flags_and_function_size &= 0xDFFF;
//             self.ended = true;
//         } else {
//             self.ended = false;
//         }

//         self.function_size = read_flags_and_function_size;

//         Ok(())
//     }

//     pub fn read_current_state_cbub_sample<R: ReadExt>(
//         &mut self,
//         reader: &mut R,
//         bits_per_sample: BitsPerSample,
//     ) -> Result<()> {
//         match self.bubble_state {
//             BubbleState::Starting => {
//                 self.read_flags_and_function_size(reader)?;

//                 self.function_string = reader.read_to_string_for(self.function_size as usize)?;
//                 // TODO ast
//                 // // TODO Error Handling
//                 // let ast = parse(lex(&buf).unwrap()).unwrap();
//                 // self.cbub_functions.push(CbubFunction::new(ast));

//                 if !self.ending_time_is_0 {
//                     self.ending_time = reader.read_le()?;
//                 }

//                 if !(self.connected || self.ended) {
//                     self.next_starting_sample = reader.read_le()?;
//                 }

//                 self.waveform_sample =
//                     WaveformSample::read_waveform_sample(reader, bits_per_sample)?;
//             }
//             BubbleState::Normal => {
//                 self.waveform_sample =
//                     WaveformSample::read_waveform_sample(reader, bits_per_sample)?
//             }
//             BubbleState::Stopped => (),
//             BubbleState::Ended => (),
//         }

//         Ok(())
//     }

//     pub fn write_flags_and_function_size<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
//         let mut read_flags_and_function_size = self.function_size;

//         // ending_time_is_0
//         if self.ending_time_is_0 {
//             read_flags_and_function_size |= 1 << 15;
//         }

//         // connected
//         if self.connected {
//             read_flags_and_function_size |= 1 << 14;
//         }

//         // ended
//         if self.ended {
//             read_flags_and_function_size |= 1 << 13;
//         }

//         writer.write_le(read_flags_and_function_size)?;

//         Ok(())
//     }

//     pub fn write_cbub_sample<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
//         match self.bubble_state {
//             BubbleState::Starting => {
//                 self.write_flags_and_function_size(writer)?;

//                 writer.write_str(&self.function_string)?;

//                 // TODO ast

//                 if !self.ending_time_is_0 {
//                     writer.write_le(self.ending_time)?;
//                 }

//                 if !(self.connected || self.ended) {
//                     writer.write_le(self.next_starting_sample)?;
//                 }

//                 self.waveform_sample.write_waveform_sample(writer)?;
//             }
//             BubbleState::Normal => {
//                 self.waveform_sample.write_waveform_sample(writer)?;
//             }
//             BubbleState::Stopped => (),
//             BubbleState::Ended => (),
//         }

//         Ok(())
//     }
// }
