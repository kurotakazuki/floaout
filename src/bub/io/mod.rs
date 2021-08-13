use crate::Sample;

pub use self::frame_reader::{BubbleFrameReader, BubbleFrameReaderKind};
pub use self::frame_writer::{BubbleFrameWriter, BubbleFrameWriterKind};
pub use self::reader::BubbleReader;
pub use self::writer::BubbleWriter;

mod frame_reader;
mod frame_writer;
mod reader;
mod writer;

pub enum BubbleSample<'a, S: Sample> {
    // Lpcm
    LpcmHead {
        head_absolute_frame: u64,
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
        sample: S,
    },
    LpcmNormal(S),
    // Expression
    Expression {
        head_absolute_frame: u64,
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
        expression: &'a [u8],
    },
}

pub enum BubbleFunctionsBlock<'a, S: Sample> {
    // Lpcm
    Lpcm {
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        next_head_relative_frame: u64,
        samples: Vec<S>,
    },
    // Expression
    Expression {
        connected: bool,
        ended: bool,
        bubble_functions: &'a [u8],
        tail_relative_frame: u64,
        next_head_relative_frame: u64,
        expression: &'a [u8],
    },
}
