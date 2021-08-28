use crate::io::{ReadExt, WriteExt};
use crate::Rgb;
use mycrc::CRC;
use std::io::{Read, Result, Write};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BubID {
    pub id: u128,
    pub rgb: Option<Rgb>,
}

impl BubID {
    pub const fn new(id: u128, rgb: Option<Rgb>) -> Self {
        Self { id, rgb }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            id: reader.read_le()?,
            rgb: None,
        })
    }
    pub fn read_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
        Ok(Self {
            id: reader.read_le_and_calc_bytes(crc)?,
            rgb: None,
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.id)
    }
    pub fn write_and_calc_bytes<W: Write>(self, writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
        writer.write_le_and_calc_bytes(self.id, crc)
    }
}
