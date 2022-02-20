/// Frames per sec
pub trait FramesPerSec {
    /// Returns frames per second.
    fn frames_per_sec(&self) -> f64;
}

/// Frames
pub trait Frames: FramesPerSec {
    /// Returns number of Frames.
    fn frames(&self) -> u64;

    /// Returns the total number of whole seconds.
    fn as_secs(&self) -> u64 {
        self.frames() / self.frames_per_sec() as u64
    }
    /// Returns the total number of whole milliseconds.
    fn as_millis(&self) -> u128 {
        self.frames() as u128 * 1_000 / self.frames_per_sec() as u128
    }
    /// Returns the total number of whole microseconds.
    fn as_micros(&self) -> u128 {
        self.frames() as u128 * 1_000_000 / self.frames_per_sec() as u128
    }
    /// Returns the total number of nanoseconds.
    fn as_nanos(&self) -> u128 {
        self.frames() as u128 * 1_000_000_000 / self.frames_per_sec() as u128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration() {
        struct Meta {
            frames: u64,
            frames_per_sec: f64,
        }
        impl FramesPerSec for Meta {
            fn frames_per_sec(&self) -> f64 {
                self.frames_per_sec
            }
        }
        impl Frames for Meta {
            fn frames(&self) -> u64 {
                self.frames
            }
        }

        let meta = Meta {
            frames: 48000,
            frames_per_sec: 48000.0,
        };
        assert_eq!(meta.as_secs(), 1);
        assert_eq!(meta.as_millis(), 1_000);
        assert_eq!(meta.as_micros(), 1_000_000);
        assert_eq!(meta.as_nanos(), 1_000_000_000);
        let meta = Meta {
            frames: 47999,
            frames_per_sec: 48000.0,
        };
        assert_eq!(meta.as_secs(), 0);
        assert_eq!(meta.as_millis(), 999);
        assert_eq!(meta.as_micros(), 999_979);
        assert_eq!(meta.as_nanos(), 999_979_166);
        let meta = Meta {
            frames: 95999,
            frames_per_sec: 48000.0,
        };
        assert_eq!(meta.as_secs(), 1);
        assert_eq!(meta.as_millis(), 1_999);
        assert_eq!(meta.as_micros(), 1_999_979);
        assert_eq!(meta.as_nanos(), 1_999_979_166);
    }
}
