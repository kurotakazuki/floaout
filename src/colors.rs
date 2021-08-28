/// RGB
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl From<(f32, f32, f32)> for Rgb {
    fn from(rgb: (f32, f32, f32)) -> Self {
        Self {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
        }
    }
}

impl From<Rgb> for (f32, f32, f32) {
    fn from(rgb: Rgb) -> Self {
        (rgb.red, rgb.green, rgb.blue)
    }
}

/// RGBA
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl From<(f32, f32, f32, f32)> for Rgba {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self {
            red: rgba.0,
            green: rgba.1,
            blue: rgba.2,
            alpha: rgba.3,
        }
    }
}

impl From<[f32; 4]> for Rgba {
    fn from(rgba: [f32; 4]) -> Self {
        Self {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: rgba[3],
        }
    }
}

impl From<Rgba> for (f32, f32, f32, f32) {
    fn from(rgba: Rgba) -> Self {
        (rgba.red, rgba.green, rgba.blue, rgba.alpha)
    }
}

impl From<Rgba> for (Rgb, f32) {
    fn from(rgba: Rgba) -> Self {
        ((rgba.red, rgba.green, rgba.blue).into(), rgba.alpha)
    }
}

impl From<Rgba> for [f32; 4] {
    fn from(rgba: Rgba) -> Self {
        [rgba.red, rgba.green, rgba.blue, rgba.alpha]
    }
}
