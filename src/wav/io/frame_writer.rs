use crate::wav::{WavFrame, WavMetadata, WavSample};
use crate::FrameWriter;
use std::io::{ErrorKind, Result, Write};

pub type WavFrameWriter<R, S> = FrameWriter<R, WavMetadata, S>;

impl<W: Write, S: WavSample> WavFrameWriter<W, S> {
    pub fn write_wav_frame(&mut self, wav_frame: WavFrame<S>) -> Result<()> {
        if wav_frame.len() != self.metadata.channels() as usize {
            return Err(ErrorKind::InvalidData.into());
        }

        if self.metadata.frames() <= self.pos {
            return Err(ErrorKind::InvalidData.into());
        } else {
            self.pos += 1;
        }

        for sample in wav_frame {
            sample.write(&mut self.inner)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wav::WavSampleKind;

    #[test]
    fn write() -> Result<()> {
        let data: Vec<u8> = Vec::new();
        let metadata = WavMetadata {
            frames: 2,
            wav_sample_kind: WavSampleKind::F32LE,
            channels: 1,
            samples_per_sec: 44100,
        };
        let mut wav_frame_writer = WavFrameWriter::<Vec<u8>, f32>::new(data, metadata);

        wav_frame_writer.write_wav_frame(vec![1.0])?;
        wav_frame_writer.write_wav_frame(vec![0.0])?;

        assert_eq!(wav_frame_writer.get_ref(), &[0, 0, 0x80, 0x3F, 0, 0, 0, 0]);
        assert!(wav_frame_writer.write_wav_frame(vec![0.0]).is_err());
        assert!(wav_frame_writer.write_wav_frame(vec![0.0]).is_err());

        Ok(())
    }
}
