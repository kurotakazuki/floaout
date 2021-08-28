// TODO: Add feilds like frame_span, vertex_spacing, colors, and so on.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeSpace {
    pub space: Vec<f64>,
}

impl VolumeSpace {
    pub const fn new() -> Self {
        Self { space: Vec::new() }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeSpaces {
    /// Number of frames between spaces
    pub frames_between_spaces: u64,
    pub vertex_spacing: f64,
    pub start: f64,
    pub range: usize,
    pub spaces: Vec<VolumeSpace>,
}

impl VolumeSpaces {
    pub const fn new() -> Self {
        Self {
            frames_between_spaces: 3200,
            // -1.2 ~ 1.2 (0.2 spacing)
            vertex_spacing: 0.2,
            start: -1.2,
            range: 13,
            spaces: Vec::new(),
        }
    }
}
