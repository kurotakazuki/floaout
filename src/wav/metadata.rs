use crate::wav::WavSample;
use crate::Metadata;
use std::any::type_name;
use std::marker::PhantomData;

pub struct WavMetadata<S: WavSample> {
    /// Number of sample frames
    frames: u32,
    /// Channels
    channels: u16,
    /// Samples per sec
    samples_per_sec: u32,
    _phantom_sample: PhantomData<S>,
}

impl<S: WavSample> Metadata for WavMetadata<S> {}

impl<S: WavSample> WavMetadata<S> {
    pub fn new(frames: u32, channels: u16, samples_per_sec: u32) -> Self {
        Self {
            frames,
            channels,
            samples_per_sec,
            _phantom_sample: PhantomData,
        }
    }

    pub fn format_tag(&self) -> u16 {
        match &type_name::<S>()[0..1] {
            // "i" | "u" => 1,
            "f" => 3,
            _ => unimplemented!(),
        }
    }

    pub fn frames(&self) -> u32 {
        self.frames
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }

    pub fn samples_per_sec(&self) -> u32 {
        self.samples_per_sec
    }

    pub fn bits_per_sample(&self) -> u16 {
        match type_name::<S>() {
            // "i16" | "u16" => 2,
            "f32" => 32,
            "f64" => 64,
            _ => unimplemented!(),
        }
    }

    pub fn bytes_per_sample(&self) -> u16 {
        self.bits_per_sample() / 8
    }

    pub fn block_align(&self) -> u16 {
        self.bytes_per_sample() * self.channels()
    }

    pub fn avg_bytes_per_sec(&self) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let metadata = WavMetadata::<f32>::new(0, 1, 44100);
        assert_eq!(metadata.format_tag(), 3);
        assert_eq!(metadata.channels(), 1);
        assert_eq!(metadata.samples_per_sec(), 44100);
        assert_eq!(metadata.avg_bytes_per_sec(), 176400);
        assert_eq!(metadata.block_align(), 4);
        assert_eq!(metadata.bits_per_sample(), 32);
        assert_eq!(metadata.bytes_per_sample(), 4);

        let metadata = WavMetadata::<f32>::new(0, 2, 44100);
        assert_eq!(metadata.format_tag(), 3);
        assert_eq!(metadata.channels(), 2);
        assert_eq!(metadata.samples_per_sec(), 44100);
        assert_eq!(metadata.avg_bytes_per_sec(), 352800);
        assert_eq!(metadata.block_align(), 8);
        assert_eq!(metadata.bits_per_sample(), 32);
        assert_eq!(metadata.bytes_per_sample(), 4);
    }
}
