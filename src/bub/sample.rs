// use crate::bub::{
//     function::{parse, FunctionVariable},
//     BubbleFrameReader, BubbleSampleKind, BubbleState,
// };
// use crate::io::{ReadExt, WriteExt};
// use crate::wav::{WavSample, WavFrame};
// use crate::Sample;
// use std::io::{Read, Result, Write};

// #[derive(Clone, Debug, PartialEq)]
// pub struct BubbleSample;

// impl Sample for BubbleSample {}

// impl BubbleSample {
//     fn read_flags_and_function_size<R: Read, S: WavSample>(
//         reader: &mut BubbleFrameReader<R, S>,
//     ) -> Result<u16> {
//         let mut read_flags_and_function_size: u16 = reader.inner.read_le()?;

//         // connected
//         reader.metadata.connected = if read_flags_and_function_size & (1 << 15) != 0 {
//             read_flags_and_function_size &= 0x7FFF;
//             true
//         } else {
//             false
//         };

//         // ended
//         reader.metadata.connected = if read_flags_and_function_size & (1 << 14) != 0 {
//             read_flags_and_function_size &= 0xBFFF;
//             true
//         } else {
//             false
//         };

//         Ok(read_flags_and_function_size)
//     }

//     pub fn read<R: Read, S: WavSample>(
//         reader: &mut BubbleFrameReader<R, S>,
//     ) -> Result<WavFrame<S>> {
//         reader.metadata.init_with_pos(reader.pos);

//         let channels = reader.metadata.speakers_absolute_coordinates.len();

//         let mut buf: WavFrame<S> = Vec::with_capacity(channels);

//         match reader.metadata.bubble_state {
//             BubbleState::Head => {
//                 let function_size = Self::read_flags_and_function_size(reader)?;

//                 let bubble_functions_vec = reader.inner.read_vec_for(function_size as usize)?;
//                 reader.metadata.bubble_functions =
//                     parse(&bubble_functions_vec, &FunctionVariable::BubbleFunctions)
//                         .unwrap()
//                         .into_original()
//                         .unwrap()
//                         .into_bubble_functions()
//                         .unwrap();

//                 reader.metadata.tail_absolute_frame_plus_one = reader.pos + reader.inner.read_le::<u64>()? - 1;

//                 if !(reader.metadata.connected || reader.metadata.ended) {
//                     reader.metadata.next_head_frame =
//                         reader.pos + reader.inner.read_le::<u64>()? - 1;
//                 }

//                 // Read Sample
//                 if let Some(volume) = reader.metadata.bubble_functions.to_volume(
//                     speaker_absolute_coordinates,
//                     reader.pos as f64,
//                     (reader.pos - reader.metadata.head_frame) as f64,
//                     reader.metadata.samples_per_sec,
//                 ) {
//                     match reader.metadata.bubble_sample_kind {
//                         BubbleSampleKind::LPCM => {
//                             return Ok(S::from_f64(volume) * S::read(&mut reader.inner)?)
//                         }
//                         BubbleSampleKind::Expression => todo!(),
//                     }
//                 }
//             }
//             BubbleState::Normal => {
//                 // Read Sample
//                 if let Some(volume) = reader.metadata.bubble_functions.to_volume(
//                     speaker_absolute_coordinates,
//                     reader.pos as f64,
//                     (reader.pos - reader.metadata.head_frame + 1) as f64,
//                     reader.metadata.samples_per_sec,
//                 ) {
//                     match reader.metadata.bubble_sample_kind {
//                         BubbleSampleKind::LPCM => {
//                             return Ok(S::from_f64(volume) * S::read(&mut reader.inner)?)
//                         }
//                         BubbleSampleKind::Expression => todo!(),
//                     }
//                 }
//             }
//             BubbleState::Stopped => (),
//             BubbleState::Ended => (),
//         }

//         Ok(S::default())
//     }

//     // pub fn write_flags_and_function_size<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
//     //     let mut read_connected_and_ended_and_function_size = self.function_size;

//     //     // tail_absolute_frame_plus_one_is_0
//     //     if self.tail_absolute_frame_plus_one_is_0 {
//     //         read_connected_and_ended_and_function_size |= 1 << 15;
//     //     }

//     //     // connected
//     //     if self.connected {
//     //         read_connected_and_ended_and_function_size |= 1 << 14;
//     //     }

//     //     // ended
//     //     if self.ended {
//     //         read_connected_and_ended_and_function_size |= 1 << 13;
//     //     }

//     //     writer.write_le(read_connected_and_ended_and_function_size)?;

//     //     Ok(())
//     // }

//     // pub fn write_cbub_sample<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
//     //     match self.bubble_state {
//     //         BubbleState::Head => {
//     //             self.write_flags_and_function_size(writer)?;

//     //             writer.write_str(&self.function_string)?;

//     //             // TODO ast

//     //             if !self.tail_absolute_frame_plus_one_is_0 {
//     //                 writer.write_le(self.tail_absolute_frame_plus_one)?;
//     //             }

//     //             if !(self.connected || self.ended) {
//     //                 writer.write_le(self.next_head_frame)?;
//     //             }

//     //             self.waveform_sample.write_waveform_sample(writer)?;
//     //         }
//     //         BubbleState::Normal => {
//     //             self.waveform_sample.write_waveform_sample(writer)?;
//     //         }
//     //         BubbleState::Stopped => (),
//     //         BubbleState::Ended => (),
//     //     }

//     //     Ok(())
//     // }
// }
