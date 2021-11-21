#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    hue: f32,
    saturation: f32,
    luminosity: f32,
}

impl HslColor {
    pub fn new<H, S, L>(h: H, s: S, l: L) -> Self
    where
        H: Into<f32>,
        S: Into<f32>,
        L: Into<f32>,
    {
        Self {
            hue: h.into().clamp(0.0, 360.0),
            luminosity: l.into().clamp(0.0, 100.0),
            saturation: s.into().clamp(0.0, 100.0),
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
    red: u16,
    green: u16,
    blue: u16,
}

impl RgbColor {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self {
            red: r.clamp(0, 255),
            blue: b.clamp(0, 255),
            green: g.clamp(0, 255),
        }
    }
}

impl From<HslColor> for RgbColor {
    fn from(hsl: HslColor) -> Self {
        hsl_to_rgb(&hsl)
    }
}

fn rgb_to_hsl(rgb: &RgbColor) -> HslColor {
    let RgbColor { red, blue, green } = *rgb;

    let red = f32::from(red) / 255.0;
    let blue = f32::from(blue) / 255.0;
    let green = f32::from(green) / 255.0;

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
    }
}

fn hsl_to_rgb(hsl: &HslColor) -> RgbColor {
    let HslColor {
        hue,
        saturation,
        luminosity,
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

    let red = ((red + lightness) * 255.0).round() as u16;
    let green = ((green + lightness) * 255.0).round() as u16;
    let blue = ((blue + lightness) * 255.0).round() as u16;

    RgbColor { red, green, blue }
}

fn round_to_one_decimal_place(n: f32) -> f32 {
    (n * 10.0).round() / 10.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_rgb_to_hsl() {
        let rgb = RgbColor::new(23_u16, 11_u16, 33_u16);

        assert_eq!(HslColor::new(273_i16, 50.0, 8.6), rgb.into());
    }

    #[test]
    fn convert_hsl_to_rgb() {
        let hsl = HslColor::new(122_i16, 33_i16, 12_i16);

        assert_eq!(RgbColor::new(21, 41, 21), hsl.into());
    }
}
