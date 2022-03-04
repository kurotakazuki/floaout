pub use self::frame_reader::{WavFrameReader, WavFrameReaderKind};
pub use self::frame_writer::{WavFrameWriter, WavFrameWriterKind};
pub use self::reader::WavReader;
pub use self::writer::WavWriter;

mod frame_reader;
mod frame_writer;
mod reader;
mod writer;
