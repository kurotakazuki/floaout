pub use self::io::WavFrameReader;
pub use self::metadata::{FormatTag, WavMetadata};
pub use self::sample::WavSample;

mod io;
mod metadata;
mod sample;
