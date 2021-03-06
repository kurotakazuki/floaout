pub use self::io::{
    WavFrameReader, WavFrameReaderKind, WavFrameWriter, WavFrameWriterKind, WavReader, WavWriter,
};
pub use self::metadata::WavMetadata;
pub use self::sample::{WavFrame, WavSample};

mod io;
mod metadata;
mod sample;
