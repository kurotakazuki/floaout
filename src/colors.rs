pub fn soft_light(a: f32, b: f32) -> f32 {
    if b <= 0.5 {
        a - (1.0 - 2.0 * b) * a * (1.0 - a)
    } else {
        let g_a = if a <= 0.25 {
            ((16.0 * a - 12.0) * a + 4.0) * a
        } else {
            a.sqrt()
        };
        a + (2.0 * b - 1.0) * (g_a - a)
    }
}

/// RGB
/// 0.0 ~ 1.0
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
/// 0.0 ~ 1.0
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

impl From<(Rgb, f32)> for Rgba {
    fn from(rgba: (Rgb, f32)) -> Self {
        Self {
            red: rgba.0.red,
            green: rgba.0.green,
            blue: rgba.0.blue,
            alpha: rgba.1,
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
