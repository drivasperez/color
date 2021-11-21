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

    // Achromatic case
    if saturation == 0.0 {
        return RgbColor {
            red: luminosity as u16,
            green: luminosity as u16,
            blue: luminosity as u16,
        };
    }

    let t2 = if luminosity <= 0.5 {
        luminosity * (saturation + 1.0)
    } else {
        luminosity + saturation - (luminosity * saturation)
    };

    let t1 = luminosity * 2.0 - t2;

    let red = hue_to_rgb(t1, t2, hue + 2.0).round() as u16;
    let green = hue_to_rgb(t1, t2, hue).round() as u16;
    let blue = hue_to_rgb(t1, t2, hue - 2.0).round() as u16;

    RgbColor { red, green, blue }
}

fn hue_to_rgb(t1: f32, t2: f32, hue: f32) -> f32 {
    let mut adjusted_hue = hue;
    if adjusted_hue < 0.0 {
        adjusted_hue += 6.0
    };
    if adjusted_hue > 6.0 {
        adjusted_hue -= 6.0
    };

    if hue < 1.0 {
        return (t2 - t1) * adjusted_hue + t1;
    }

    if hue < 3.0 {
        return t2;
    }

    if hue < 4.0 {
        return (t2 - t1) * (4.0 - adjusted_hue) + t1;
    }

    t1
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
