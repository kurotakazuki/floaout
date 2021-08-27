use crate::bub::BubFrameReader;
use crate::oao::{BubInOao, OaoMetadata};
use crate::{Coord, Frame, FrameIOKind, FrameReader, Sample};
use std::io::{Read, Result};
use std::marker::PhantomData;

pub struct OaoFrameReader<R: Read, B: Read + Clone, S: Sample> {
    pub inner: R,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
    pub metadata: OaoMetadata,
    /// Speakers absolute coordinates
    pub speakers_absolute_coord: Vec<Coord>,

    // Buffers
    pub bubs: Vec<(BubInOao, BubFrameReader<B, S>)>,
    /// Bubble Frame Readers
    pub bub_frame_readers: Vec<BubFrameReader<B, S>>,
}

impl<R: Read, B: Read + Clone, S: Sample> FrameReader<R, S> for OaoFrameReader<R, B, S> {
    fn get_ref(&self) -> &R {
        &self.inner
    }
    fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
    fn into_inner(self) -> R {
        self.inner
    }

    fn frames(&self) -> u64 {
        self.metadata.frames()
    }

    fn samples_per_sec(&self) -> f64 {
        self.metadata.samples_per_sec()
    }

    fn number_of_channels(&self) -> u32 {
        self.speakers_absolute_coord.len() as u32
    }
}

impl<R: Read, B: Read + Clone, S: Sample> OaoFrameReader<R, B, S> {
    pub fn new(
        inner: R,
        metadata: OaoMetadata,
        speakers_absolute_coord: Vec<Coord>,
        bub_frame_readers: Vec<BubFrameReader<B, S>>,
    ) -> Self {
        // TODO: Is same bubs length?
        let mut bubs = Vec::with_capacity(metadata.bubs.len());
        for (i, bub_frame_reader) in bub_frame_readers.into_iter().enumerate() {
            bubs.push((metadata.bubs[i].clone(), bub_frame_reader));
        }
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
        let mut i = 0;
        while i < self.bubs.len() {
            if let Some(starting_frame) = self.bubs[i].0.starting_frames.front() {
                if starting_frame == &self.pos {
                    self.bubs[i].0.starting_frames.pop_front();
                    // Push BubFrameReader
                    self.bub_frame_readers.push(self.bubs[i].1.clone());
                }
                i += 1;
            } else {
                // If there is no more starting frames.
                self.bubs.remove(i);
            }
        }

        Ok(())
    }

    fn read_bub_frame_readers_frame(&mut self, frame: &mut Frame<S>) -> Result<()> {
        let mut i = 0;
        while i < self.bub_frame_readers.len() {
            match self.bub_frame_readers[i].next() {
                Some(result) => {
                    frame.add(result?)?;
                    i += 1;
                }
                None => {
                    self.bub_frame_readers.remove(i);
                }
            }
        }

        Ok(())
    }
}

impl<R: Read, B: Read + Clone, S: Sample> Iterator for OaoFrameReader<R, B, S> {
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

pub type OaoFrameReaderKind<R, B> =
    FrameIOKind<OaoFrameReader<R, B, f32>, OaoFrameReader<R, B, f64>>;

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
