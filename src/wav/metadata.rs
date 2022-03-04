use crate::io::{ReadExt, WriteBytes, WriteExt};
use crate::utils::return_invalid_data_if_not_equal;
use crate::{LpcmKind, Metadata};
use std::io::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct WavMetadata {
    /// Number of sample frames
    pub frames: u64,
    // LPCM Kind
    pub lpcm_kind: LpcmKind,
    /// Channels
    pub channels: u16,
    /// Samples per sec
    pub samples_per_sec: f64,
    /// List data
    pub list: Vec<Chunk>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub fourcc: String,
    pub data: Vec<u8>,
}

impl Metadata for WavMetadata {}

impl WavMetadata {
    pub const fn new(
        frames: u64,
        lpcm_kind: LpcmKind,
        channels: u16,
        samples_per_sec: f64,
        list: Vec<Chunk>,
    ) -> Self {
        Self {
            frames,
            lpcm_kind,
            channels,
            samples_per_sec,
            list,
        }
    }

    pub const fn calculate_frames(data_size: u32, channels: u16, bits_per_sample: u16) -> u64 {
        data_size as u64 / (channels * bits_per_sample / 8) as u64
    }

    pub const fn frames(&self) -> u64 {
        self.frames
    }

    pub const fn lpcm_kind(&self) -> LpcmKind {
        self.lpcm_kind
    }

    pub const fn format_tag(&self) -> u16 {
        self.lpcm_kind.format_tag()
    }

    pub const fn channels(&self) -> u16 {
        self.channels
    }

    pub const fn samples_per_sec(&self) -> f64 {
        self.samples_per_sec
    }

    pub const fn bits_per_sample(&self) -> u16 {
        self.lpcm_kind.bits_per_sample()
    }

    pub const fn bytes_per_sample(&self) -> u16 {
        self.bits_per_sample() / 8
    }

    pub const fn block_align(&self) -> u16 {
        self.bytes_per_sample() * self.channels()
    }

    pub const fn avg_bytes_per_sec(&self) -> u32 {
        self.samples_per_sec() as u32 * self.block_align() as u32
    }

    pub const fn data_chunk_size(&self) -> u32 {
        self.frames() as u32 * self.block_align() as u32
    }

    pub const fn standard_riff_chunk_size(&self) -> u32 {
        // riff chunk + fmt chunk + data chunk
        4 + 24 + 8 + self.data_chunk_size()
    }

    pub const fn secs(&self) -> u64 {
        self.frames() / self.samples_per_sec() as u64
    }

    pub const fn millis(&self) -> u128 {
        self.frames() as u128 * 1_000 / self.samples_per_sec() as u128
    }

    pub const fn micros(&self) -> u128 {
        self.frames() as u128 * 1_000_000 / self.samples_per_sec() as u128
    }

    pub const fn nanos(&self) -> u128 {
        self.frames() as u128 * 1_000_000_000 / self.samples_per_sec() as u128
    }

    // IO
    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Self> {
        let check_fourcc = |reader: &mut R, val: &str| {
            let fourcc = reader.read_string::<4>()?;
            return_invalid_data_if_not_equal(fourcc, val.to_string())
        };
        // Riff chunk
        check_fourcc(reader, "RIFF")?;
        // File size - 8
        reader.read_array::<4>()?;
        check_fourcc(reader, "WAVE")?;

        // Other chunk
        let other_chunk = |reader: &mut R| -> Result<()> {
            let chunk_size: u32 = reader.read_le()?;
            let mut buf = vec![0; chunk_size as usize];
            reader.read_exact(&mut buf)?;

            Ok(())
        };

        // Declare
        let mut format_tag: u16 = 0;
        let mut channels: u16 = 0;
        let mut samples_per_sec: u32 = 0;
        let mut avg_bytes_per_sec: u32 = 0;
        let mut block_align: u16 = 0;
        let mut bits_per_sample: u16 = 0;
        let mut list: Vec<Chunk> = vec![];

        // Read chunks
        loop {
            let fourcc = reader.read_string::<4>()?;
            match fourcc.as_ref() {
                "LIST" => {
                    let list_size: u32 = reader.read_le()?;
                    check_fourcc(reader, "INFO")?;
                    let mut curr_size: u32 = 4;
                    while list_size != curr_size {
                        let fourcc = reader.read_string::<4>()?;
                        let child_size: u32 = reader.read_le()?;
                        let mut data = vec![0; child_size as usize];
                        reader.read_exact(&mut data)?;
                        list.push(
                            Chunk {
                                fourcc,
                                data,
                            }
                        );
                        curr_size += 8 + child_size;
                    }
                }
                "fmt " => {
                    let fmt_size: u32 = reader.read_le()?;
                    return_invalid_data_if_not_equal(fmt_size, 16)?;

                    format_tag = reader.read_le()?;
                    channels = reader.read_le()?;
                    samples_per_sec = reader.read_le()?;
                    avg_bytes_per_sec = reader.read_le()?;
                    block_align = reader.read_le()?;
                    bits_per_sample = reader.read_le()?;
                }
                "data" => {
                    let data_size: u32 = reader.read_le()?;

                    let frames =
                        Self::calculate_frames(data_size, channels, bits_per_sample);

                    let wav_metadata = Self {
                        frames,
                        lpcm_kind: LpcmKind::from_format_tag_and_bits_per_sample(
                            format_tag,
                            bits_per_sample,
                        ),
                        channels,
                        samples_per_sec: samples_per_sec as f64,
                        list,
                    };

                    return_invalid_data_if_not_equal(
                        avg_bytes_per_sec,
                        wav_metadata.avg_bytes_per_sec(),
                    )?;
                    return_invalid_data_if_not_equal(
                        block_align,
                        wav_metadata.block_align(),
                    )?;

                    return Ok(wav_metadata);
                },
                _ => other_chunk(reader)?,
            }
        }
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        // Riff chunk
        writer.write_str("RIFF")?;
        self.standard_riff_chunk_size().write_le_bytes(writer)?;
        writer.write_str("WAVE")?;
        // Fmt chunk
        writer.write_str("fmt ")?;
        writer.write_le(16_u32)?;
        self.format_tag().write_le_bytes(writer)?;
        self.channels().write_le_bytes(writer)?;
        (self.samples_per_sec() as u32).write_le_bytes(writer)?;
        self.avg_bytes_per_sec().write_le_bytes(writer)?;
        self.block_align().write_le_bytes(writer)?;
        self.bits_per_sample().write_le_bytes(writer)?;
        // Data chunk
        writer.write_str("data")?;
        self.data_chunk_size().write_le_bytes(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn duration() {
        let metadata = WavMetadata {
            frames: 48000,
            lpcm_kind: LpcmKind::F32LE,
            channels: 1,
            samples_per_sec: 48000.0,
            list: vec![],
        };
        assert_eq!(metadata.secs(), 1);
        assert_eq!(metadata.millis(), 1_000);
        assert_eq!(metadata.micros(), 1_000_000);
        assert_eq!(metadata.nanos(), 1_000_000_000);
        let metadata = WavMetadata {
            frames: 48001,
            lpcm_kind: LpcmKind::F32LE,
            channels: 1,
            samples_per_sec: 48000.0,
            list: vec![],
        };
        assert_eq!(metadata.secs(), 1);
        assert_eq!(metadata.millis(), 1_000);
        assert_eq!(metadata.micros(), 1_000_020);
        assert_eq!(metadata.nanos(), 1_000_020_833);
        let metadata = WavMetadata {
            frames: 95999,
            lpcm_kind: LpcmKind::F32LE,
            channels: 1,
            samples_per_sec: 48000.0,
            list: vec![],
        };
        assert_eq!(metadata.secs(), 1);
        assert_eq!(metadata.millis(), 1_999);
        assert_eq!(metadata.micros(), 1_999_979);
        assert_eq!(metadata.nanos(), 1_999_979_166);
    }

    #[test]
    fn info() {
        let lpcm_kind = LpcmKind::F32LE;
        let frames = 0;
        let samples_per_sec = 44100.0;

        let metadata = WavMetadata {
            frames,
            lpcm_kind,
            channels: 1,
            samples_per_sec,
            list: vec![],
        };
        assert_eq!(metadata.format_tag(), 3);
        assert_eq!(metadata.channels(), 1);
        assert_eq!(metadata.samples_per_sec(), 44100.0);
        assert_eq!(metadata.avg_bytes_per_sec(), 176400);
        assert_eq!(metadata.block_align(), 4);
        assert_eq!(metadata.bits_per_sample(), 32);
        assert_eq!(metadata.bytes_per_sample(), 4);
        assert_eq!(metadata.secs(), 0);

        let metadata = WavMetadata {
            frames: 88200,
            lpcm_kind,
            channels: 1,
            samples_per_sec,
            list: vec![],
        };
        assert_eq!(metadata.format_tag(), 3);
        assert_eq!(metadata.channels(), 1);
        assert_eq!(metadata.samples_per_sec(), 44100.0);
        assert_eq!(metadata.avg_bytes_per_sec(), 176400);
        assert_eq!(metadata.block_align(), 4);
        assert_eq!(metadata.bits_per_sample(), 32);
        assert_eq!(metadata.bytes_per_sample(), 4);
        assert_eq!(metadata.data_chunk_size(), 352800);
        assert_eq!(metadata.standard_riff_chunk_size(), 352836);
        assert_eq!(metadata.secs(), 2);

        let metadata = WavMetadata {
            frames,
            lpcm_kind,
            channels: 2,
            samples_per_sec,
            list: vec![],
        };
        assert_eq!(metadata.format_tag(), 3);
        assert_eq!(metadata.channels(), 2);
        assert_eq!(metadata.samples_per_sec(), 44100.0);
        assert_eq!(metadata.avg_bytes_per_sec(), 352800);
        assert_eq!(metadata.block_align(), 8);
        assert_eq!(metadata.bits_per_sample(), 32);
        assert_eq!(metadata.bytes_per_sample(), 4);
    }

    #[test]
    fn read() -> Result<()> {
        let lpcm_kind = LpcmKind::F32LE;

        let mut data: &[u8] = &[
            0x52, 0x49, 0x46, 0x46, 0x44, 0x62, 0x05, 0x00, 0x57, 0x41, 0x56, 0x45, 0x66, 0x6D,
            0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x44, 0xAC, 0x00, 0x00,
            0x10, 0xB1, 0x02, 0x00, 0x04, 0x00, 0x20, 0x00, 0x64, 0x61, 0x74, 0x61, 0x20, 0x62,
            0x05, 0x00,
        ];
        let val = WavMetadata::read(&mut data)?;
        let expect = WavMetadata {
            frames: 88200,
            lpcm_kind,
            channels: 1,
            samples_per_sec: 44100.0,
            list: vec![],
        };
        assert_eq!(val, expect);

        const DATA: [u8; 734] = [
            0x52, 0x49, 0x46, 0x46, 0xE6, 0xA6, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45, 0x4A, 0x55,
            0x4E, 0x4B, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x6D, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x02, 0x00, 0x44, 0xAC, 0x00, 0x00, 0x20, 0x62, 0x05, 0x00, 0x08, 0x00,
            0x20, 0x00, 0x66, 0x61, 0x63, 0x74, 0x04, 0x00, 0x00, 0x00, 0x82, 0x14, 0x00, 0x00,
            0x62, 0x65, 0x78, 0x74, 0x5A, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x30, 0x32, 0x31, 0x2D, 0x30, 0x36, 0x2D,
            0x31, 0x33, 0x31, 0x33, 0x2D, 0x35, 0x38, 0x2D, 0x34, 0x30, 0x20, 0x48, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x63, 0x69, 0x64, 0x18, 0x00,
            0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFC, 0x42, 0x64, 0x61,
            0x74, 0x61, 0x10, 0xA4, 0x00, 0x00,
        ];

        let mut data: &[u8] = &DATA;

        let val = WavMetadata::read(&mut data)?;
        let expect = WavMetadata {
            frames: 5250,
            lpcm_kind,
            channels: 2,
            samples_per_sec: 44100.0,
            list: vec![],
        };
        assert_eq!(val, expect);

        for i in 0..100 {
            let mut data: &[u8] = &DATA[0..i];
            let val = WavMetadata::read(&mut data);
            assert!(val.is_err());
        }

        let mut data: &[u8] = &DATA[0..500];
        let val = WavMetadata::read(&mut data);
        assert!(val.is_err());

        Ok(())
    }

    #[test]
    fn write_and_read() -> Result<()> {
        let mut v = Vec::new();
        let metadata = WavMetadata {
            frames: 88200,
            lpcm_kind: LpcmKind::F32LE,
            channels: 1,
            samples_per_sec: 44100.0,
            list: vec![],
        };
        let mut data: &[u8] = &[
            0x52, 0x49, 0x46, 0x46, 0x44, 0x62, 0x05, 0x00, 0x57, 0x41, 0x56, 0x45, 0x66, 0x6D,
            0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x44, 0xAC, 0x00, 0x00,
            0x10, 0xB1, 0x02, 0x00, 0x04, 0x00, 0x20, 0x00, 0x64, 0x61, 0x74, 0x61, 0x20, 0x62,
            0x05, 0x00,
        ];

        metadata.write(&mut v)?;

        assert_eq!(v, data);

        let written_metadata = WavMetadata::read(&mut data)?;

        assert_eq!(written_metadata, metadata);

        Ok(())
    }
}
