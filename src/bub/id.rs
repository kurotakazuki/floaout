use crate::io::{ReadExt, WriteExt};
use std::io::{Read, Result, Write};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BubbleID {
    pub id: u128,
}

impl BubbleID {
    pub fn new(id: u128) -> Self {
        Self { id }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            id: reader.read_le()?,
        })
    }

    pub fn write<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.write_le(self.id)
    }
}
