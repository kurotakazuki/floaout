use crate::bub::{
    functions::{parse, BubFns, BubFnsAST, BubFnsVariable},
    BubID,
};
use crate::io::{ReadExt, WriteExt};
use crate::utils::{read_crc, write_crc};
use crate::{LpcmKind, Metadata, CRC_32K_4_2};
use mycrc::CRC;
use std::io::{ErrorKind, Read, Result, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BubState {
    Head,
    Body,
    Stopped,
    Ended,
}

impl BubState {
    pub fn is_head(&self) -> bool {
        matches!(self, Self::Head)
    }

    pub fn is_body(&self) -> bool {
        matches!(self, Self::Body)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub fn is_ended(&self) -> bool {
        matches!(self, Self::Ended)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BubSampleKind {
    Lpcm,
    Expr(BubFnsAST),
}

impl From<BubFnsAST> for BubSampleKind {
    fn from(ast: BubFnsAST) -> Self {
        Self::Expr(ast)
    }
}

impl BubSampleKind {
    pub fn default_expr() -> Self {
        parse("0".as_bytes(), &BubFnsVariable::Sum).unwrap().into()
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let value: u8 = reader.read_le()?;
        Ok(match value {
            0 => Self::Lpcm,
            1 => Self::default_expr(),
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }
    pub fn read_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
        let value: u8 = reader.read_le_and_calc_bytes(crc)?;
        Ok(match value {
            0 => Self::Lpcm,
            1 => Self::default_expr(),
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_le(self.to_u8())
    }
    pub fn write_and_calc_bytes<W: Write>(&self, writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
        writer.write_le_and_calc_bytes(self.to_u8(), crc)
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Lpcm,
            1 => Self::default_expr(),
            _ => unimplemented!(),
        }
    }

    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::Lpcm => 0,
            Self::Expr(_) => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BubMetadata {
    // In File Header
    /// Version of Bubble File Format Specification.
    pub spec_version: u8,
    /// Bubble ID
    pub bub_id: BubID,
    /// Version of Bubble
    pub bub_version: u16,
    /// Frames
    pub frames: u64,
    /// Samples Per Sec
    pub samples_per_sec: f64,
    /// Bits Per Sample
    pub lpcm_kind: LpcmKind,
    /// Bubble Sample Kind
    pub bub_sample_kind: BubSampleKind,
    /// Name of Bubble
    pub name: String,

    /// Bubble State
    pub bub_state: BubState,
    /// Head Absolute Frame
    pub head_absolute_frame: u64,

    /// Bubble Functions
    pub bub_functions: BubFns,
    /// Foot Absolute Frame Plus One
    pub foot_absolute_frame_plus_one: u64,
    /// Next Head Absolute Frame
    /// `None` if 0.
    pub next_head_absolute_frame: Option<u64>,
}

impl BubMetadata {
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
        self.head_absolute_frame = pos;
        self.bub_state = BubState::Head;
    }

    pub fn set_as_body(&mut self) {
        self.bub_state = BubState::Body;
    }

    pub fn set_as_stopped(&mut self) {
        self.bub_state = BubState::Stopped;
    }

    pub fn set_as_ended(&mut self) {
        self.bub_state = BubState::Ended;
    }

    pub(crate) fn set_bub_state_from_connected_and_ended(&mut self, pos: u64) {
        // TODO: Create closures method
        match self.next_head_absolute_frame {
            Some(next_head_absolute_frame) => {
                if next_head_absolute_frame == pos {
                    self.set_as_head(pos)
                } else {
                    self.set_as_stopped()
                }
            }
            None => self.set_as_ended(),
        }
    }

    pub fn init_with_pos(&mut self, pos: u64) {
        match self.bub_state {
            BubState::Head => {
                if self.foot_absolute_frame_plus_one == pos {
                    self.set_bub_state_from_connected_and_ended(pos);
                } else {
                    self.set_as_body();
                }
            }
            BubState::Body => {
                if self.foot_absolute_frame_plus_one == pos {
                    self.set_bub_state_from_connected_and_ended(pos);
                }
            }
            BubState::Stopped => {
                if self.next_head_absolute_frame.expect("Some") == pos {
                    self.set_as_head(pos);
                }
            }
            BubState::Ended => (),
        }
    }

    pub(crate) fn next_head_absolute_frame_from_relative(
        next_head_relative_frame: u64,
        pos: u64,
    ) -> Option<u64> {
        if next_head_relative_frame != 0 {
            Some(next_head_relative_frame + pos - 1)
        } else {
            None
        }
    }

    pub(crate) fn set_next_head_absolute_frame_from_relative(
        &mut self,
        next_head_relative_frame: u64,
        pos: u64,
    ) {
        self.next_head_absolute_frame =
            Self::next_head_absolute_frame_from_relative(next_head_relative_frame, pos);
    }

    // IO
    /// TODO: Refactoring
    pub(crate) fn read_next_head_absolute_frame_from_relative<R: std::io::Read>(
        &mut self,
        reader: &mut R,
        pos: u64,
        crc: &mut CRC<u32>,
    ) -> Result<()> {
        let next_head_relative_frame: u64 = reader.read_le_and_calc_bytes(crc)?;
        self.set_next_head_absolute_frame_from_relative(next_head_relative_frame, pos);

        Ok(())
    }

    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<(Self, CRC<u32>)> {
        let mut crc = CRC_32K_4_2;

        let spec_version = reader.read_le_and_calc_bytes(&mut crc)?;
        let bub_id = BubID::read_and_calc_bytes(reader, &mut crc)?;
        let bub_version = reader.read_le_and_calc_bytes(&mut crc)?;

        let frames = reader.read_le_and_calc_bytes(&mut crc)?;

        let next_head_relative_frame: u64 = reader.read_le_and_calc_bytes(&mut crc)?;
        let next_head_absolute_frame =
            Self::next_head_absolute_frame_from_relative(next_head_relative_frame, 1);
        let samples_per_sec = reader.read_le_and_calc_bytes(&mut crc)?;
        let lpcm_kind = LpcmKind::read_and_calc_bytes(reader, &mut crc)?;
        let bub_sample_kind = BubSampleKind::read_and_calc_bytes(reader, &mut crc)?;

        let name_size: u8 = reader.read_le_and_calc_bytes(&mut crc)?;
        let name = reader.read_string_for_and_calc_bytes(name_size as usize, &mut crc)?;

        // CRC
        read_crc(reader, &mut crc)?;

        Ok((
            Self {
                spec_version,
                bub_id,
                bub_version,
                frames,
                samples_per_sec,
                lpcm_kind,
                bub_sample_kind,
                name,

                bub_state: BubState::Stopped,
                head_absolute_frame: 0,

                bub_functions: BubFns::new(),
                foot_absolute_frame_plus_one: 0,
                next_head_absolute_frame,
            },
            crc,
        ))
    }

    fn next_head_absolute_frame_into_relative(&self, pos: u64) -> u64 {
        match self.next_head_absolute_frame {
            Some(n) => 1 + n - pos,
            None => 0,
        }
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<CRC<u32>> {
        let mut crc = CRC_32K_4_2;

        writer.write_le_and_calc_bytes(self.spec_version, &mut crc)?;
        self.bub_id.write_and_calc_bytes(writer, &mut crc)?;
        writer.write_le_and_calc_bytes(self.bub_version, &mut crc)?;

        writer.write_le_and_calc_bytes(self.frames, &mut crc)?;
        writer.write_le_and_calc_bytes(self.next_head_absolute_frame_into_relative(1), &mut crc)?;
        writer.write_le_and_calc_bytes(self.samples_per_sec, &mut crc)?;
        self.lpcm_kind.write_and_calc_bytes(writer, &mut crc)?;
        self.bub_sample_kind
            .write_and_calc_bytes(writer, &mut crc)?;
        writer.write_le_and_calc_bytes(self.name.len() as u8, &mut crc)?;
        writer.write_str_and_calc_bytes(&self.name, &mut crc)?;

        // CRC
        write_crc(writer, &mut crc)?;

        Ok(crc)
    }
}

impl Metadata for BubMetadata {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() -> Result<()> {
        let bub_metadata = BubMetadata {
            spec_version: 0,
            bub_id: BubID::new(0),
            bub_version: 0,
            frames: 96000,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            bub_sample_kind: BubSampleKind::Lpcm,
            name: String::from("Vocal"),

            bub_state: BubState::Stopped,
            head_absolute_frame: 0,

            bub_functions: BubFns::new(),
            foot_absolute_frame_plus_one: 0,
            next_head_absolute_frame: Some(1),
        };
        let expected = bub_metadata.clone();

        let mut v: Vec<u8> = Vec::new();

        bub_metadata.write(&mut v)?;

        let val = BubMetadata::read(&mut &v[..])?.0;

        assert_eq!(val, expected);

        Ok(())
    }
}
