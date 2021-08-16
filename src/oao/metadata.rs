use crate::io::{ReadExt, WriteExt};
use crate::oao::OaoID;
use crate::utils;
use crate::{LpcmKind, Metadata, CRC_32K_4_2};
use mycrc::CRC;
use std::io::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct OaoMetadata {
    // In File Header
    /// Version of Floaout File Format Specification.
    pub spec_version: u8,
    /// Floaout ID
    pub oao_id: OaoID,
    /// Version of Floaout
    pub oao_version: u16,
    /// Bubbles
    pub bubs: u16,
    /// Frames
    pub frames: u64,
    /// Samples Per Sec
    pub samples_per_sec: f64,
    /// Bits Per Sample
    pub lpcm_kind: LpcmKind,
    /// Title of Floaout
    pub title: String,
    /// Artist of Floaout
    pub artist: String,

    /// CRC
    pub crc: CRC<u32>,
    // Each Bubble
}

impl OaoMetadata {
    pub const fn frames(&self) -> u64 {
        self.frames
    }

    pub const fn lpcm_kind(&self) -> LpcmKind {
        self.lpcm_kind
    }

    pub const fn samples_per_sec(&self) -> f64 {
        self.samples_per_sec
    }

    // IO
    pub(crate) fn read_crc<R: std::io::Read>(&mut self, reader: &mut R) -> Result<()> {
        utils::read_crc(reader, &mut self.crc)
    }
    pub(crate) fn write_crc<W: std::io::Write>(&mut self, writer: &mut W) -> Result<()> {
        utils::write_crc(writer, &mut self.crc)
    }

    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Self> {
        let mut metadata = Self {
            spec_version: Default::default(),
            oao_id: Default::default(),
            oao_version: Default::default(),
            bubs: Default::default(),
            frames: Default::default(),
            samples_per_sec: Default::default(),
            lpcm_kind: LpcmKind::F64LE,
            title: Default::default(),
            artist: Default::default(),

            crc: CRC_32K_4_2,
        };

        metadata.spec_version = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.oao_id = OaoID::read_and_calc_bytes(reader, &mut metadata.crc)?;
        metadata.oao_version = reader.read_le_and_calc_bytes(&mut metadata.crc)?;

        metadata.bubs = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.frames = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.samples_per_sec = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.lpcm_kind = LpcmKind::read_and_calc_bytes(reader, &mut metadata.crc)?;
        // Title
        let title_size: u8 = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.title =
            reader.read_string_for_and_calc_bytes(title_size as usize, &mut metadata.crc)?;
        // Artist
        let artist_size: u8 = reader.read_le_and_calc_bytes(&mut metadata.crc)?;
        metadata.artist =
            reader.read_string_for_and_calc_bytes(artist_size as usize, &mut metadata.crc)?;

        // CRC
        metadata.read_crc(reader)?;

        Ok(metadata)
    }

    pub fn write<W: std::io::Write>(&mut self, writer: &mut W) -> Result<()> {
        writer.write_le_and_calc_bytes(self.spec_version, &mut self.crc)?;
        self.oao_id.write_and_calc_bytes(writer, &mut self.crc)?;
        writer.write_le_and_calc_bytes(self.oao_version, &mut self.crc)?;

        writer.write_le_and_calc_bytes(self.bubs, &mut self.crc)?;
        writer.write_le_and_calc_bytes(self.frames, &mut self.crc)?;
        writer.write_le_and_calc_bytes(self.samples_per_sec, &mut self.crc)?;
        self.lpcm_kind.write_and_calc_bytes(writer, &mut self.crc)?;
        // Title
        writer.write_le_and_calc_bytes(self.title.len() as u8, &mut self.crc)?;
        writer.write_str_and_calc_bytes(&self.title, &mut self.crc)?;
        // Artist
        writer.write_le_and_calc_bytes(self.artist.len() as u8, &mut self.crc)?;
        writer.write_str_and_calc_bytes(&self.artist, &mut self.crc)?;

        // CRC
        self.write_crc(writer)
    }
}

impl Metadata for OaoMetadata {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() -> Result<()> {
        let mut oao_metadata = OaoMetadata {
            spec_version: 0,
            oao_id: OaoID::new(0),
            oao_version: 0,
            bubs: 0,
            frames: 96000,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            title: String::from("untitled"),
            artist: String::from("undefined"),

            crc: CRC_32K_4_2,
        };
        let expected = oao_metadata.clone();

        let mut v: Vec<u8> = Vec::new();

        oao_metadata.write(&mut v)?;

        let mut val = OaoMetadata::read(&mut &v[..])?;
        val.crc = CRC_32K_4_2;

        assert_eq!(val, expected);

        Ok(())
    }
}
