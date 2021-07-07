pub use self::io::{WavFrameReader, WavFrames, WavReader};
pub use self::metadata::WavMetadata;
pub use self::sample::{WavSample, WavSampleKind};

mod io;
mod metadata;
mod sample;
