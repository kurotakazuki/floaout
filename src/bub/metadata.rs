use crate::bub::{function::BubbleFunctions, BubbleID};
use crate::io::{ReadExt, WriteExt};
use crate::utils::return_invalid_data_if_not_equal;
use crate::{Metadata, SampleKind};
use std::io::{ErrorKind, Read, Result, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BubbleState {
    Head,
    Normal,
    Stopped,
    Ended,
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

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.to_u8())
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
    /// Head Sample
    pub starting_sample: u64,
    /// This is the number of `Bubble` version.
    pub version: u8,
    /// Bubble ID
    pub bubble_id: BubbleID,
    /// Frames
    pub frames: u64,
    /// Samples Per Sec
    pub samples_per_sec: f64,
    /// Bits Per Sample
    pub sample_kind: SampleKind,
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

    pub const fn sample_kind(&self) -> SampleKind {
        self.sample_kind
    }

    pub const fn samples_per_sec(&self) -> f64 {
        self.samples_per_sec
    }

    fn set_as_head(&mut self, pos: u64) {
        self.head_frame = pos;
        self.bubble_state = BubbleState::Head;
    }

    fn set_as_normal(&mut self) {
        self.bubble_state = BubbleState::Normal;
    }

    fn set_as_stopped(&mut self) {
        self.bubble_state = BubbleState::Stopped;
    }

    fn set_as_ended(&mut self) {
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
}

impl Metadata for BubbleMetadata {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Self> {
        let bub = reader.read_array::<3>()?;

        return_invalid_data_if_not_equal(&bub[..], "bub".as_bytes())?;

        let version = reader.read_le()?;
        let bubble_id = BubbleID::read(reader)?;

        let frames = reader.read_le()?;
        let samples_per_sec = reader.read_le()?;
        let sample_kind = SampleKind::read(reader)?;
        let bubble_sample_kind = BubbleSampleKind::read(reader)?;

        let name_size: u8 = reader.read_le()?;
        let name = reader.read_string_for(name_size as usize)?;

        // TODO CRC-32C

        Ok(Self {
            starting_sample: 0,
            version,
            bubble_id,
            frames,
            samples_per_sec,
            sample_kind,
            bubble_sample_kind,
            name,

            speakers_absolute_coordinates: Vec::new(),

            bubble_state: BubbleState::Head,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 0,
        })
    }
    fn write<W: std::io::Write>(self, writer: &mut W) -> Result<()> {
        writer.write_str("bub")?;
        writer.write_le(self.version)?;
        self.bubble_id.write(writer)?;
        writer.write_le(self.frames)?;
        writer.write_le(self.samples_per_sec)?;
        self.sample_kind.write(writer)?;
        self.bubble_sample_kind.write(writer)?;
        writer.write_le(self.name.len() as u8)?;
        writer.write_str(&self.name)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() -> Result<()> {
        let bubble_metadata = BubbleMetadata {
            starting_sample: 0,
            version: 0,
            bubble_id: BubbleID::new(0),
            frames: 96000,
            samples_per_sec: 96000.0,
            sample_kind: SampleKind::F32LE,
            bubble_sample_kind: BubbleSampleKind::LPCM,
            name: String::from("Vocal"),

            speakers_absolute_coordinates: Vec::new(),

            bubble_state: BubbleState::Head,
            head_frame: 0,

            bubble_functions: BubbleFunctions::new(),
            connected: false,
            ended: false,
            tail_absolute_frame_plus_one: 0,
            next_head_frame: 0,
        };
        let expected = bubble_metadata.clone();

        let mut v: Vec<u8> = Vec::new();

        bubble_metadata.write(&mut v)?;

        let val = BubbleMetadata::read(&mut &v[..])?;

        assert_eq!(val, expected);

        Ok(())
    }
}
