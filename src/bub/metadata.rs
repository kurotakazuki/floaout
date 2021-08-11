use crate::bub::{function::BubbleFunctions, BubbleID};
use crate::crc::CRC;
use crate::io::{ReadExt, WriteExt};
use crate::{LpcmKind, Metadata};
use mycrc::CRC;
use std::io::{ErrorKind, Read, Result, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BubbleState {
    Head,
    Normal,
    Stopped,
    Ended,
}

impl BubbleState {
    pub fn is_head(&self) -> bool {
        matches!(self, Self::Head)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub fn is_ended(&self) -> bool {
        matches!(self, Self::Ended)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BubbleSampleKind {
    LPCM,
    Expression,
}

impl BubbleSampleKind {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value: u8 = reader.read_le()?;
        Ok(match value {
            0 => Self::LPCM,
            1 => Self::Expression,
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }
    pub fn read_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
        let value: u8 = reader.read_le_and_calc_bytes(crc)?;
        Ok(match value {
            0 => Self::LPCM,
            1 => Self::Expression,
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.to_u8())
    }
    pub fn write_and_calc_bytes<W: Write>(self, writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
        writer.write_le_and_calc_bytes(self.to_u8(), crc)
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::LPCM,
            1 => Self::Expression,
            _ => unimplemented!(),
        }
    }

    pub const fn to_u8(self) -> u8 {
        match self {
            Self::LPCM => 0,
            Self::Expression => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BubbleMetadata {
    // In File Header
    /// Version of Bubble File Format Specification.
    pub spec_version: u8,
    /// Bubble ID
    pub bubble_id: BubbleID,
    /// Version of Bubble
    pub bubble_version: u16,
    /// Frames
    pub frames: u64,
    /// Samples Per Sec
    pub samples_per_sec: f64,
    /// Bits Per Sample
    pub lpcm_kind: LpcmKind,
    /// Bubble Sample Kind
    pub bubble_sample_kind: BubbleSampleKind,
    /// Name of Bubble
    pub name: String,

    /// Speakers absolute coordinates
    pub speakers_absolute_coordinates: Vec<(f64, f64, f64)>,

    /// Bubble State
    pub bubble_state: BubbleState,
    /// Head Absolute Frame
    pub head_frame: u64,

    /// Connected or Not Flag
    /// | Value | Contents |
    /// | ---------------- |
    /// | 0 | Not Connected |
    /// | 1 | Connected |
    pub connected: bool,
    /// Ended or Not Flag
    /// | Value | Contents |
    /// | ---------------- |
    /// | 0 | Not Ended |
    /// | 1 | Ended |
    pub ended: bool,
    /// Bubble Functions
    pub bubble_functions: BubbleFunctions,
    /// Tail Absolute Frame Plus One
    pub tail_absolute_frame_plus_one: u64,
    /// Next Head Absolute Frame
    /// If `self.connected` or `self.ended` is `true', this won't exist.
    pub next_head_frame: u64,
}

impl BubbleMetadata {
    pub const fn frames(&self) -> u64 {
        self.frames
    }

    pub const fn lpcm_kind(&self) -> LpcmKind {
        self.lpcm_kind
    }

    pub const fn samples_per_sec(&self) -> f64 {
        self.samples_per_sec
    }

    pub fn set_as_head(&mut self, pos: u64) {
        self.head_frame = pos;
        self.bubble_state = BubbleState::Head;
    }

    pub fn set_as_normal(&mut self) {
        self.bubble_state = BubbleState::Normal;
    }

    pub fn set_as_stopped(&mut self) {
        self.bubble_state = BubbleState::Stopped;
    }

    pub fn set_as_ended(&mut self) {
        self.bubble_state = BubbleState::Ended;
    }

    fn set_bubble_state_from_connected_and_ended(&mut self, pos: u64) {
        if self.ended {
            self.set_as_ended()
        } else if self.connected {
            self.set_as_head(pos)
        } else {
            self.set_as_stopped()
        }
    }

    pub fn init_with_pos(&mut self, pos: u64) {
        match self.bubble_state {
            BubbleState::Head => {
                if self.tail_absolute_frame_plus_one == pos {
                    self.set_bubble_state_from_connected_and_ended(pos);
                } else {
                    self.set_as_normal();
                }
            }
            BubbleState::Normal => {
                if self.tail_absolute_frame_plus_one == pos {
                    self.set_bubble_state_from_connected_and_ended(pos);
                }
            }
            BubbleState::Stopped => {
                if self.next_head_frame == pos {
                    self.set_as_head(pos);
                }
            }
            BubbleState::Ended => (),
        }
    }

    // fn bubble_state_from_connected_and_ended(&self) -> BubbleState {
    //     if self.ended {
    //         BubbleState::Ended
    //     } else if self.connected {
    //         BubbleState::Head
    //     } else {
    //         BubbleState::Stopped
    //     }
    // }

    // pub fn next_pos_bubble_state(&self, pos: u64) -> BubbleState {
    //     let next_pos = pos + 1;
    //     match self.bubble_state {
    //         BubbleState::Head => {
    //             if self.tail_absolute_frame_plus_one == next_pos {
    //                 self.bubble_state_from_connected_and_ended()
    //             } else {
    //                 BubbleState::Normal
    //             }
    //         }
    //         BubbleState::Normal => {
    //             if self.tail_absolute_frame_plus_one == next_pos {
    //                 self.bubble_state_from_connected_and_ended()
    //             } else {
    //                 BubbleState::Normal
    //             }
    //         }
    //         BubbleState::Stopped => {
    //             if self.next_head_frame == next_pos {
    //                 BubbleState::Head
    //             } else {
    //                 BubbleState::Stopped
    //             }
    //         }
    //         BubbleState::Ended => BubbleState::Ended,
    //     }
    // }
}

impl Metadata for BubbleMetadata {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Self> {
        let mut crc = CRC;

        let spec_version = reader.read_le_and_calc_bytes(&mut crc)?;
        let bubble_id = BubbleID::read_and_calc_bytes(reader, &mut crc)?;
        let bubble_version = reader.read_le_and_calc_bytes(&mut crc)?;

        let frames = reader.read_le_and_calc_bytes(&mut crc)?;
        let samples_per_sec = reader.read_le_and_calc_bytes(&mut crc)?;
        let lpcm_kind = LpcmKind::read_and_calc_bytes(reader, &mut crc)?;
        let bubble_sample_kind = BubbleSampleKind::read_and_calc_bytes(reader, &mut crc)?;

        let name_size: u8 = reader.read_le_and_calc_bytes(&mut crc)?;
        let name = reader.read_string_for_and_calc_bytes(name_size as usize, &mut crc)?;

        // CRC
        let _checksum: u32 = reader.read_le_and_calc_bytes(&mut crc)?;
        // TODO: Return Error
        assert!(crc.is_error_free());

        Ok(Self {
            spec_version,
            bubble_id,
            bubble_version,
            frames,
            samples_per_sec,
            lpcm_kind,
            bubble_sample_kind,
            name,

            speakers_absolute_coordinates: Vec::new(),

            bubble_state: BubbleState::Stopped,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,
        })
    }
    fn write<W: std::io::Write>(self, writer: &mut W) -> Result<()> {
        let mut crc = CRC;

        writer.write_le_and_calc_bytes(self.spec_version, &mut crc)?;
        self.bubble_id.write_and_calc_bytes(writer, &mut crc)?;
        writer.write_le_and_calc_bytes(self.bubble_version, &mut crc)?;

        writer.write_le_and_calc_bytes(self.frames, &mut crc)?;
        writer.write_le_and_calc_bytes(self.samples_per_sec, &mut crc)?;
        self.lpcm_kind.write_and_calc_bytes(writer, &mut crc)?;
        self.bubble_sample_kind
            .write_and_calc_bytes(writer, &mut crc)?;
        writer.write_le_and_calc_bytes(self.name.len() as u8, &mut crc)?;
        writer.write_str_and_calc_bytes(&self.name, &mut crc)?;

        // CRC
        let checksum_bytes = crc.finalize_to_endian_bytes();
        writer.write_all(&checksum_bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() -> Result<()> {
        let bubble_metadata = BubbleMetadata {
            spec_version: 0,
            bubble_id: BubbleID::new(0),
            bubble_version: 0,
            frames: 96000,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::LPCM,
            name: String::from("Vocal"),

            speakers_absolute_coordinates: Vec::new(),

            bubble_state: BubbleState::Stopped,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 1,
        };
        let expected = bubble_metadata.clone();

        let mut v: Vec<u8> = Vec::new();

        bubble_metadata.write(&mut v)?;

        let val = BubbleMetadata::read(&mut &v[..])?;

        assert_eq!(val, expected);

        Ok(())
    }
}
