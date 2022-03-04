pub use self::io::{
    WavFrameReader, WavFrameReaderKind, WavFrameWriter, WavFrameWriterKind, WavReader, WavWriter,
};
pub use self::metadata::{WavMetadata, Chunk};

mod io;
mod metadata;
