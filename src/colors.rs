#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    hue: f32,
    saturation: f32,
    luminosity: f32,
    alpha: f32,
}

impl HslColor {
    pub fn new(hue: f32, saturation: f32, luminosity: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            luminosity: luminosity.clamp(0.0, 100.0),
            saturation: saturation.clamp(0.0, 100.0),
            alpha: 1.0,
        }
    }

    pub fn hsla(hue: f32, saturation: f32, luminosity: f32, alpha: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            luminosity: luminosity.clamp(0.0, 100.0),
            saturation: saturation.clamp(0.0, 100.0),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    /// Get a reference to the hsl color's hue.
    pub fn hue(&self) -> f32 {
        self.hue
    }

    /// Get a reference to the hsl color's saturation.
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Get a reference to the hsl color's luminosity.
    pub fn luminosity(&self) -> f32 {
        self.luminosity
    }
}

impl From<RgbColor> for HslColor {
    fn from(rgb: RgbColor) -> Self {
        rgb_to_hsl(&rgb)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RgbColor {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl RgbColor {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red: red.clamp(0.0, 255.0),
            blue: blue.clamp(0.0, 255.0),
            green: green.clamp(0.0, 255.0),
            alpha: 1.0,
        }
    }
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: red.clamp(0.0, 255.0),
            blue: blue.clamp(0.0, 255.0),
            green: green.clamp(0.0, 255.0),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
}

impl From<HslColor> for RgbColor {
    fn from(hsl: HslColor) -> Self {
        hsl_to_rgb(&hsl)
    }
}

fn rgb_to_hsl(rgb: &RgbColor) -> HslColor {
    let RgbColor {
        red,
        blue,
        green,
        alpha,
    } = *rgb;

    let red = red / 255.0;
    let blue = blue / 255.0;
    let green = green / 255.0;

    let cmin = [red, green, blue].into_iter().reduce(f32::min).unwrap();
    let cmax = [red, green, blue].into_iter().reduce(f32::max).unwrap();
    let delta = cmax - cmin;

    let mut hue;
    let mut saturation;
    let mut luminosity;

    // hue
    if delta == 0.0 {
        hue = 0.0;
    } else if (cmax - red).abs() < f32::EPSILON {
        hue = ((green - blue) / delta) % 6.0;
    } else if (cmax - green).abs() < f32::EPSILON {
        hue = (blue - red) / delta + 2.0;
    } else {
        hue = (red - green) / delta + 4.0;
    }

    hue = (hue * 60.0).round();

    if hue < 0.0 {
        hue += 360.0;
    }

    // luminosity
    luminosity = (cmax + cmin) / 2.0;

    // saturation
    saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * luminosity - 1.0).abs())
    };

    saturation *= 100.0;
    luminosity *= 100.0;

    HslColor {
        hue: round_to_one_decimal_place(hue),
        saturation: round_to_one_decimal_place(saturation),
        luminosity: round_to_one_decimal_place(luminosity),
        alpha,
    }
}

fn hsl_to_rgb(hsl: &HslColor) -> RgbColor {
    let HslColor {
        hue,
        saturation,
        luminosity,
        alpha,
    } = *hsl;

    let saturation = saturation / 100.0;
    let luminosity = luminosity / 100.0;

    let chroma = (1.0 - (2.0 * luminosity - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let lightness = luminosity - chroma / 2.0;
    let red;
    let green;
    let blue;

    if (0.0..60.0).contains(&hue) {
        red = chroma;
        green = x;
        blue = 0.0;
    } else if (60.0..120.0).contains(&hue) {
        red = x;
        green = chroma;
        blue = 0.0;
    } else if (120.0..180.0).contains(&hue) {
        red = 0.0;
        green = chroma;
        blue = x;
    } else if (180.0..240.0).contains(&hue) {
        red = 0.0;
        green = x;
        blue = chroma;
    } else if (240.0..300.0).contains(&hue) {
        red = x;
        green = 0.0;
        blue = chroma;
    } else if (300.0..360.0).contains(&hue) {
        red = chroma;
        green = 0.0;
        blue = x;
    } else {
        unreachable!("HSL hue is clamped to 0..=360")
    }

    let red = ((red + lightness) * 255.0).round();
    let green = ((green + lightness) * 255.0).round();
    let blue = ((blue + lightness) * 255.0).round();

    RgbColor {
        red,
        green,
        blue,
        alpha,
    }
}

fn round_to_one_decimal_place(n: f32) -> f32 {
    (n * 10.0).round() / 10.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_rgb_to_hsl() {
        let rgb = RgbColor::new(23.0, 11.0, 33.0);

        assert_eq!(HslColor::new(273.0, 50.0, 8.6), rgb.into());
    }

    #[test]
    fn convert_hsl_to_rgb() {
        let hsl = HslColor::new(122.0, 33.0, 12.0);

        assert_eq!(RgbColor::new(21.0, 41.0, 21.0), hsl.into());
    }
}
