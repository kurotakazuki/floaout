pub struct WavMetadata {
    format_tag: FormatTag,
    /// Number of sample frames
    frames: u32,
    /// Channels
    channels: u16,
    /// Samples per sec
    samples_per_sec: u32,
    /// Bits Per Sample
    bits_per_sample: u16,
}

impl WavMetadata {
    pub const fn frames(&self) -> u32 {
        self.frames
    }

    pub const fn channels(&self) -> u16 {
        self.channels
    }

    pub const fn samples_per_sec(&self) -> u32 {
        self.samples_per_sec
    }

    pub const fn bits_per_sample(&self) -> u16 {
        self.bits_per_sample
    }

    pub const fn bytes_per_sample(&self) -> u16 {
        self.bits_per_sample / 8
    }

    pub const fn block_align(&self) -> u16 {
        self.bytes_per_sample() * self.channels()
    }

    pub const fn avg_bytes_per_sec(&self) -> u32 {
        self.samples_per_sec() * self.block_align() as u32
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FormatTag {
    UncompressedPCM,
    IEEEFloatingPoint,
    // WaveFormatExtensible,
    Other(u16),
}

impl From<FormatTag> for u16 {
    fn from(format_tag: FormatTag) -> Self {
        match format_tag {
            FormatTag::UncompressedPCM => 1,
            FormatTag::IEEEFloatingPoint => 3,
            // FormatTag::WaveFormatExtensible => 65534,
            FormatTag::Other(n) => n,
        }
    }
}

impl From<u16> for FormatTag {
    fn from(n: u16) -> Self {
        match n {
            1 => FormatTag::UncompressedPCM,
            3 => FormatTag::IEEEFloatingPoint,
            // 65534 => FormatTag::WaveFormatExtensible,
            _ => FormatTag::Other(n),
        }
    }
}
