use crate::bub::BubbleID;
use crate::io::{ReadExt, WriteExt};
use crate::utils::return_invalid_data_if_not_equal;
use crate::{Metadata, SampleKind};
use std::io::Result;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BubbleMetadata {
    /// Starting Sample
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
    /// Name of Bubble
    pub name: String,
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
            name,
        })
    }
    fn write<W: std::io::Write>(self, writer: &mut W) -> Result<()> {
        writer.write_str("bub")?;
        writer.write_le(self.version)?;
        self.bubble_id.write(writer)?;
        writer.write_le(self.frames)?;
        writer.write_le(self.samples_per_sec)?;
        self.sample_kind.write(writer)?;
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
            name: String::from("Vocal"),
        };
        let expected = bubble_metadata.clone();

        let mut v: Vec<u8> = Vec::new();

        bubble_metadata.write(&mut v)?;

        let val = BubbleMetadata::read(&mut &v[..])?;

        assert_eq!(val, expected);

        Ok(())
    }
}
