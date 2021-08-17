use crate::bub::{BubFrameReader, BubReader};
use crate::oao::{BubInOao, OaoMetadata};
use crate::{Coord, Frame, FrameIOKind, FrameReader, Sample};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::marker::PhantomData;

pub struct OaoFrameReader<R: Read, S: Sample> {
    pub inner: R,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
    pub metadata: OaoMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coord: Vec<Coord>,

    // Buffers
    pub bubs: Vec<BubInOao>,
    /// Bubble Frame Readers
    pub bub_frame_readers: Vec<BubFrameReader<BufReader<File>, S>>,
}

impl<R: Read, S: Sample> FrameReader<R> for OaoFrameReader<R, S> {
    fn get_ref(&self) -> &R {
        &self.inner
    }
    fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
    fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read, S: Sample> OaoFrameReader<R, S> {
    pub fn new(inner: R, metadata: OaoMetadata, speakers_absolute_coord: Vec<Coord>) -> Self {
        let bubs = metadata.bubs.clone();
        Self {
            inner,
            pos: 0,
            _phantom_sample: PhantomData,
            metadata,
            speakers_absolute_coord,
            // Buffers
            bubs,
            bub_frame_readers: Vec::new(),
        }
    }

    fn set_new_bub_frame_readers(&mut self) -> Result<()> {
        for i in 0..self.bubs.len() {
            if let Some(starting_frame) = self.bubs[i].starting_frames.front() {
                if starting_frame == &self.pos {
                    self.bubs[i].starting_frames.pop_front();
                    // Push BubFrameReader
                    let bub_reader = BubReader::open(
                        format!("{}.bub", self.bubs[i].file_name),
                        self.speakers_absolute_coord.clone(),
                    )?;
                    let bub_frame_reader = bub_reader.into_bub_frame_reader::<S>();
                    self.bub_frame_readers.push(bub_frame_reader);
                }
            } else {
                // If there is no more starting frames.
                self.bubs.remove(i);
            }
        }

        Ok(())
    }

    fn read_bub_frame_readers_frame(&mut self, frame: &mut Frame<S>) -> Result<()> {
        for i in 0..self.bub_frame_readers.len() {
            match self.bub_frame_readers[i].next() {
                Some(result) => {
                    frame.add(result?)?;
                }
                None => {
                    self.bub_frame_readers.remove(i);
                }
            }
        }

        Ok(())
    }
}

impl<R: Read, S: Sample> Iterator for OaoFrameReader<R, S> {
    type Item = Result<Frame<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.metadata.frames() <= self.pos {
            return None;
        } else {
            self.pos += 1;
        }

        if let Err(e) = self.set_new_bub_frame_readers() {
            return Some(Err(e));
        }

        let channels = self.speakers_absolute_coord.len();
        let mut frame: Frame<S> = vec![S::default(); channels].into();

        if let Err(e) = self.read_bub_frame_readers_frame(&mut frame) {
            return Some(Err(e));
        }

        Some(Ok(frame))
    }
}

pub type OaoFrameReaderKind<R> = FrameIOKind<OaoFrameReader<R, f32>, OaoFrameReader<R, f64>>;

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
