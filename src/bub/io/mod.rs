use crate::Sample;

pub use self::frame_reader::{BubFrameReader, BubFrameReaderKind};
pub use self::frame_writer::{BubFrameWriter, BubFrameWriterKind};
pub use self::reader::BubReader;
pub use self::writer::BubWriter;

mod frame_reader;
mod frame_writer;
mod reader;
mod writer;

pub enum BubbleSample<'a, S: Sample> {
    // Lpcm
    LpcmHead {
        head_absolute_frame: u64,

        bub_fns: &'a [u8],
        foot_relative_frame: u64,
        next_head_relative_frame: Option<u64>,
        sample: S,
    },
    LpcmBody(S),
    // Expression
    Expr {
        head_absolute_frame: u64,

        bub_fns: &'a [u8],
        foot_relative_frame: u64,
        next_head_relative_frame: Option<u64>,
        expression: &'a [u8],
    },
}

pub enum BubFnsBlock<'a, S: Sample> {
    // Lpcm
    Lpcm {
        bub_fns: &'a [u8],
        next_head_relative_frame: Option<u64>,
        samples: Vec<S>,
    },
    // Expression
    Expr {
        bub_fns: &'a [u8],
        foot_relative_frame: u64,
        next_head_relative_frame: Option<u64>,
        expression: &'a [u8],
    },
}
