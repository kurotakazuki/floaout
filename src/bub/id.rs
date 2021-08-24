use crate::io::{ReadExt, WriteExt};
use mycrc::CRC;
use std::io::{Read, Result, Write};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BubID {
    pub id: u128,
}

impl BubID {
    pub const fn new(id: u128) -> Self {
        Self { id }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            id: reader.read_le()?,
        })
    }
    pub fn read_and_calc_bytes<R: Read>(reader: &mut R, crc: &mut CRC<u32>) -> Result<Self> {
        Ok(Self {
            id: reader.read_le_and_calc_bytes(crc)?,
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.id)
    }
    pub fn write_and_calc_bytes<W: Write>(self, writer: &mut W, crc: &mut CRC<u32>) -> Result<()> {
        writer.write_le_and_calc_bytes(self.id, crc)
    }
}
