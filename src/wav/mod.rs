pub use self::io::{
    WavFrameReader, WavFrameReaderKind, WavFrameWriter, WavFrameWriterKind, WavReader, WavWriter,
};
pub use self::metadata::WavMetadata;

mod io;
mod metadata;
