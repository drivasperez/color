#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    pub hue: f32,
    pub saturation: f32,
    pub luminosity: f32,
}

impl From<RgbColor> for HslColor {
    fn from(rgb: RgbColor) -> Self {
        rgb_to_hsl(&rgb)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RgbColor {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl RgbColor {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self {
            red: r,
            blue: b,
            green: g,
        }
    }
}

impl From<HslColor> for RgbColor {
    fn from(hsl: HslColor) -> Self {
        hsl_to_rgb(&hsl)
    }
}

impl HslColor {
    pub fn new<H, S, L>(h: H, s: S, l: L) -> Self
    where
        H: Into<f32>,
        S: Into<f32>,
        L: Into<f32>,
    {
        Self {
            hue: h.into(),
            luminosity: l.into(),
            saturation: s.into(),
        }
    }
}

fn rgb_to_hsl(rgb: &RgbColor) -> HslColor {
    let RgbColor { red, blue, green } = *rgb;

    let r = f32::from(red) / 255.0;
    let b = f32::from(blue) / 255.0;
    let g = f32::from(green) / 255.0;

    let cmin = [r, g, b].into_iter().reduce(f32::min).unwrap();
    let cmax = [r, g, b].into_iter().reduce(f32::max).unwrap();
    let delta = cmax - cmin;

    let mut h;
    let mut s;
    let mut l;

    // hue
    if delta == 0.0 {
        h = 0.0;
    } else if cmax == r {
        h = ((g - b) / delta) % 6.0;
    } else if cmax == g {
        h = (b - r) / delta + 2.0;
    } else {
        h = (r - g) / delta + 4.0;
    }

    h = (h * 60.0).round();

    if h < 0.0 {
        h += 360.0;
    }

    // luminosity
    l = (cmax + cmin) / 2.0;

    // saturation
    s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    s *= 100.0;
    l *= 100.0;

    HslColor {
        hue: round_to_one_decimal_place(h),
        saturation: round_to_one_decimal_place(s),
        luminosity: round_to_one_decimal_place(l),
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
    } else if 60.0 <= hue && hue < 120.0 {
        red = x;
        green = chroma;
        blue = 0.0;
    } else if 120.0 <= hue && hue < 180.0 {
        red = 0.0;
        green = chroma;
        blue = x;
    } else if 180.0 <= hue && hue < 240.0 {
        red = 0.0;
        green = x;
        blue = chroma;
    } else if 240.0 <= hue && hue < 300.0 {
        red = x;
        green = 0.0;
        blue = chroma;
    } else if 300.0 <= hue && hue < 360.0 {
        red = chroma;
        green = 0.0;
        blue = x;
    } else {
        panic!("Shouldn't be possible if initialised properly")
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
