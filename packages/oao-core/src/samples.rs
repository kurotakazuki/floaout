use core::mem::size_of;

/// SampleFormat
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// Unsigned 8-bit integer.
    U8,
    /// Unsigned 16-bit integer.
    U16,
    /// Unsigned 24-bit integer.
    U24,
    /// Unsigned 32-bit integer.
    U32,
    /// Unsigned 64-bit integer.
    U64,
    /// Signed 8-bit integer.
    I8,
    /// Signed 16-bit integer.
    I16,
    /// Signed 24-bit integer.
    I24,
    /// Signed 32-bit integer.
    I32,
    /// Signed 64-bit integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
}

impl SampleFormat {
    /// Returns the size of this SampleFormat in bytes.
    pub const fn sample_size(&self) -> usize {
        match *self {
            SampleFormat::U8 => size_of::<u8>(),
            SampleFormat::U16 => size_of::<u16>(),
            SampleFormat::U24 => 3,
            SampleFormat::U32 => size_of::<u32>(),
            SampleFormat::U64 => size_of::<u64>(),
            SampleFormat::I8 => size_of::<u8>(),
            SampleFormat::I16 => size_of::<i16>(),
            SampleFormat::I24 => 3,
            SampleFormat::I32 => size_of::<i32>(),
            SampleFormat::I64 => size_of::<i64>(),
            SampleFormat::F32 => size_of::<f32>(),
            SampleFormat::F64 => size_of::<f64>(),
        }
    }
}

/// Sample
pub trait Sample:
    Clone
    + Copy
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + Default
    + PartialOrd
    + PartialEq
    + Sized
{
    /// The `SampleFormat` corresponding to this data type.
    const FORMAT: SampleFormat;
}

impl Sample for i16 {
    const FORMAT: SampleFormat = SampleFormat::I16;
}

impl Sample for f32 {
    const FORMAT: SampleFormat = SampleFormat::F32;
}

impl Sample for f64 {
    const FORMAT: SampleFormat = SampleFormat::F64;
}

/// Frame
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame<S: Sample, const CH: usize>(pub [S; CH]);

impl<S: Sample, const CH: usize> From<Frame<S, CH>> for [S; CH] {
    fn from(value: Frame<S, CH>) -> Self {
        value.0
    }
}

impl<S: Sample, const CH: usize> From<[S; CH]> for Frame<S, CH> {
    fn from(value: [S; CH]) -> Self {
        Frame(value)
    }
}
