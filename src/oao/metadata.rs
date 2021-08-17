use crate::io::{ReadExt, WriteExt};
use crate::oao::OaoID;
use crate::utils::{read_crc, write_crc};
use crate::{LpcmKind, Metadata, CRC_32K_4_2};
use std::collections::VecDeque;
use std::io::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BubInOao {
    pub file_name: String,
    pub starting_frames: VecDeque<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OaoMetadata {
    // In File Header
    /// Version of Floaout File Format Specification.
    pub spec_version: u8,
    /// Floaout ID
    pub oao_id: OaoID,
    /// Version of Floaout
    pub oao_version: u16,
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

    // Each Bubble
    pub bubs: Vec<BubInOao>,
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
    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Self> {
        let mut crc = CRC_32K_4_2;

        let spec_version = reader.read_le_and_calc_bytes(&mut crc)?;
        let oao_id = OaoID::read_and_calc_bytes(reader, &mut crc)?;
        let oao_version = reader.read_le_and_calc_bytes(&mut crc)?;

        let num_of_bubs: u16 = reader.read_le_and_calc_bytes(&mut crc)?;
        let frames = reader.read_le_and_calc_bytes(&mut crc)?;
        let samples_per_sec = reader.read_le_and_calc_bytes(&mut crc)?;
        let lpcm_kind = LpcmKind::read_and_calc_bytes(reader, &mut crc)?;
        // Title
        let title_size: u8 = reader.read_le_and_calc_bytes(&mut crc)?;
        let title = reader.read_string_for_and_calc_bytes(title_size as usize, &mut crc)?;
        // Artist
        let artist_size: u8 = reader.read_le_and_calc_bytes(&mut crc)?;
        let artist = reader.read_string_for_and_calc_bytes(artist_size as usize, &mut crc)?;

        // CRC
        read_crc(reader, &mut crc)?;

        // Bubbles
        let mut bubs = Vec::new();
        for _ in 0..num_of_bubs {
            // File Name
            let file_name_size: u8 = reader.read_le_and_calc_bytes(&mut crc)?;
            let file_name =
                reader.read_string_for_and_calc_bytes(file_name_size as usize, &mut crc)?;
            // Starting Frames
            let mut starting_frames = VecDeque::new();
            let num_of_starting_frames: u16 = reader.read_le_and_calc_bytes(&mut crc)?;
            for _ in 0..num_of_starting_frames {
                let starting_frame: u64 = reader.read_le_and_calc_bytes(&mut crc)?;
                starting_frames.push_back(starting_frame);
            }
            bubs.push(BubInOao {
                file_name,
                starting_frames,
            });
            // CRC
            read_crc(reader, &mut crc)?;
        }

        Ok(Self {
            spec_version,
            oao_id,
            oao_version,
            frames,
            samples_per_sec,
            lpcm_kind,
            title,
            artist,
            bubs,
        })
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        let mut crc = CRC_32K_4_2;

        writer.write_le_and_calc_bytes(self.spec_version, &mut crc)?;
        self.oao_id.write_and_calc_bytes(writer, &mut crc)?;
        writer.write_le_and_calc_bytes(self.oao_version, &mut crc)?;

        writer.write_le_and_calc_bytes(self.bubs.len() as u16, &mut crc)?;
        writer.write_le_and_calc_bytes(self.frames, &mut crc)?;
        writer.write_le_and_calc_bytes(self.samples_per_sec, &mut crc)?;
        self.lpcm_kind.write_and_calc_bytes(writer, &mut crc)?;
        // Title
        writer.write_le_and_calc_bytes(self.title.len() as u8, &mut crc)?;
        writer.write_str_and_calc_bytes(&self.title, &mut crc)?;
        // Artist
        writer.write_le_and_calc_bytes(self.artist.len() as u8, &mut crc)?;
        writer.write_str_and_calc_bytes(&self.artist, &mut crc)?;

        // CRC
        write_crc(writer, &mut crc)?;

        // Bubbles
        for bub in self.bubs.iter() {
            // Name
            writer.write_le_and_calc_bytes(bub.file_name.len() as u8, &mut crc)?;
            writer.write_str_and_calc_bytes(&bub.file_name, &mut crc)?;
            // Starting Frames
            writer.write_le_and_calc_bytes(bub.starting_frames.len() as u16, &mut crc)?;
            for starting_frame in bub.starting_frames.iter() {
                writer.write_le_and_calc_bytes(*starting_frame, &mut crc)?;
            }
            // CRC
            write_crc(writer, &mut crc)?;
        }

        Ok(())
    }
}

impl Metadata for OaoMetadata {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() -> Result<()> {
        let metadata_0_bubs = OaoMetadata {
            spec_version: 0,
            oao_id: OaoID::new(0),
            oao_version: 0,
            frames: 96000,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            title: String::from("untitled"),
            artist: String::from("undefined"),

            bubs: Vec::new(),
        };
        let bub0 = BubInOao {
            file_name: "".into(),
            starting_frames: vec![].into(),
        };
        let bub1 = BubInOao {
            file_name: "a".into(),
            starting_frames: vec![1].into(),
        };
        let bub2 = BubInOao {
            file_name: "abc".into(),
            starting_frames: vec![1, 2, 3].into(),
        };
        let metadata_3_bubs = OaoMetadata {
            spec_version: 0,
            oao_id: OaoID::new(0),
            oao_version: 0,
            frames: 96000,
            samples_per_sec: 96000.0,
            lpcm_kind: LpcmKind::F32LE,
            title: String::from("untitled"),
            artist: String::from("undefined"),

            bubs: vec![bub0, bub1, bub2],
        };
        let metadatas = [metadata_0_bubs, metadata_3_bubs];

        for metadata in metadatas {
            let expected = metadata.clone();
            let mut v: Vec<u8> = Vec::new();
            metadata.write(&mut v)?;
            let val = OaoMetadata::read(&mut &v[..])?;

            assert_eq!(val, expected);
        }

        Ok(())
    }
}
