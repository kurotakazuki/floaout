use crate::wav::WavMetadata;
use crate::{Frame, FrameIOKind, FrameWriter, Sample};
use std::io::{ErrorKind, Result, Write};
use std::marker::PhantomData;

pub struct WavFrameWriter<W: Write, S: Sample> {
    pub inner: W,
    pub metadata: WavMetadata,
    pub pos: u64,
    _phantom_sample: PhantomData<S>,
}

impl<W: Write, S: Sample> FrameWriter<W> for WavFrameWriter<W, S> {
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
    fn get_ref(&self) -> &W {
        &self.inner
    }
    fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
    fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write, S: Sample> WavFrameWriter<W, S> {
    pub fn new(inner: W, metadata: WavMetadata) -> Self {
        let pos = 0;

        Self {
            inner,
            metadata,
            pos,
            _phantom_sample: PhantomData,
        }
    }

    pub fn write_frame(&mut self, wav_frame: Frame<S>) -> Result<()> {
        if wav_frame.0.len() != self.metadata.channels() as usize {
            return Err(ErrorKind::InvalidData.into());
        }

        if self.metadata.frames() <= self.pos {
            return Err(ErrorKind::InvalidData.into());
        } else {
            self.pos += 1;
        }

        for sample in wav_frame.0 {
            sample.write(&mut self.inner)?;
        }

        Ok(())
    }
}

pub type WavFrameWriterKind<W> = FrameIOKind<WavFrameWriter<W, f32>, WavFrameWriter<W, f64>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LpcmKind;

    #[test]
    fn write() -> Result<()> {
        let data: Vec<u8> = Vec::new();
        let metadata = WavMetadata {
            frames: 2,
            lpcm_kind: LpcmKind::F32LE,
            channels: 1,
            samples_per_sec: 44100.0,
            list: vec![],
        };
        let mut wav_frame_writer = WavFrameWriter::<Vec<u8>, f32>::new(data, metadata);

        wav_frame_writer.write_frame(vec![1.0].into())?;
        wav_frame_writer.write_frame(vec![0.0].into())?;

        assert_eq!(wav_frame_writer.get_ref(), &[0, 0, 0x80, 0x3F, 0, 0, 0, 0]);
        assert!(wav_frame_writer.write_frame(vec![0.0].into()).is_err());
        assert!(wav_frame_writer.write_frame(vec![0.0].into()).is_err());

        Ok(())
    }
}
