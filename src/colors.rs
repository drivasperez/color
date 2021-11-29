use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Hsl,
    Rgb,
    Hex,
}

impl FromStr for ColorType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "hsl" | "hsla" => Ok(Self::Hsl),
            "rgb" | "rgba" => Ok(Self::Rgb),
            "hex" => Ok(Self::Hex),
            _ => Err(crate::Error::InvalidColorType(s.to_string())),
        }
    }
}

// TODO: Rename this to Color and delete Color, HslColor and RgbColor
#[derive(Clone, Debug)]
pub struct Color {
    parsed_as: ColorType,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl FromStr for Color {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, color) = crate::parse::parse_color(s).map_err(|_| crate::Error::InvalidColor)?;

        Ok(color)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.parsed_as {
            ColorType::Hsl => self.hsl_string(),
            ColorType::Rgb => self.rgb_string(),
            ColorType::Hex => self.hex_string(),
        };
        write!(f, "{}", s)
    }
}

impl Color {
    pub fn from_hsl(hue: f32, saturation: f32, luminosity: f32, alpha: f32) -> Self {
        let (red, green, blue, alpha) = hsl_to_rgb(hue, saturation, luminosity, alpha);

        Self {
            red: red.clamp(0.0, 255.0),
            green: green.clamp(0.0, 255.0),
            blue: blue.clamp(0.0, 255.0),
            alpha: alpha.clamp(0.0, 1.0),
            parsed_as: ColorType::Hsl,
        }
    }

    pub fn from_rgb(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: red.clamp(0.0, 255.0),
            green: green.clamp(0.0, 255.0),
            blue: blue.clamp(0.0, 255.0),
            alpha: alpha.clamp(0.0, 1.0),
            parsed_as: ColorType::Rgb,
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        todo!()
    }

    pub fn rgb(&self) -> (f32, f32, f32, f32) {
        let Self {
            red,
            green,
            blue,
            alpha,
            ..
        } = *self;

        (red, green, blue, alpha)
    }

    pub fn hsl(&self) -> (f32, f32, f32, f32) {
        let Self {
            red,
            green,
            blue,
            alpha,
            ..
        } = *self;

        rgb_to_hsl(red, green, blue, alpha)
    }

    pub fn rgb_string(&self) -> String {
        let Self {
            red,
            green,
            blue,
            alpha,
            ..
        } = *self;

        if (alpha - 1.0).abs() < f32::EPSILON {
            format!("rgb({} {} {})", red, green, blue)
        } else {
            format!("rgb({} {} {} / {})", red, green, blue, alpha)
        }
    }

    pub fn hsl_string(&self) -> String {
        let (hue, sat, lum, alpha) = self.hsl();

        if (alpha - 1.0).abs() < f32::EPSILON {
            format!("hsl({} {} {})", hue, sat, lum)
        } else {
            format!("hsl({} {} {} / {})", hue, sat, lum, alpha)
        }
    }

    pub fn hex_string(&self) -> String {
        let (red, green, blue, alpha) = self.rgb();
        rgb_to_hex(red, green, blue, alpha)
    }
}

fn rgb_to_hsl(red: f32, green: f32, blue: f32, alpha: f32) -> (f32, f32, f32, f32) {
    let red = red.clamp(0.0, 255.0) / 255.0;
    let green = green.clamp(0.0, 255.0) / 255.0;
    let blue = blue.clamp(0.0, 255.0) / 255.0;
    let alpha = alpha.clamp(0.0, 1.0);

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

    (
        round_to_one_decimal_place(hue),
        round_to_one_decimal_place(saturation),
        round_to_one_decimal_place(luminosity),
        alpha,
    )
}

fn hsl_to_rgb(hue: f32, saturation: f32, luminosity: f32, alpha: f32) -> (f32, f32, f32, f32) {
    let hue = hue.clamp(0.0, 360.0);
    let saturation = saturation.clamp(0.0, 100.0);
    let luminosity = luminosity.clamp(0.0, 100.0);
    let alpha = alpha.clamp(0.0, 1.0);

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
    } else if (300.0..=360.0).contains(&hue) {
        red = chroma;
        green = 0.0;
        blue = x;
    } else {
        unreachable!("HSL hue is clamped to 0..=360")
    }

    let red = ((red + lightness) * 255.0).round();
    let green = ((green + lightness) * 255.0).round();
    let blue = ((blue + lightness) * 255.0).round();

    (red, green, blue, alpha)
}

fn round_to_one_decimal_place(n: f32) -> f32 {
    (n * 10.0).round() / 10.0
}

fn rgb_to_hex(red: f32, green: f32, blue: f32, alpha: f32) -> String {
    if (alpha - 1.0).abs() < f32::EPSILON {
        format!(
            "#{:02X}{:02X}{:02X}",
            red.round() as i64,
            green.round() as i64,
            blue.round() as i64
        )
    } else {
        let alpha = (alpha * 255.0).round() as i64;
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            red.round() as i64,
            green.round() as i64,
            blue.round() as i64,
            alpha
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_rgb_to_hsl() {
        let color = Color::from_rgb(23.0, 11.0, 33.0, 1.0);

        assert_eq!((273.0, 50.0, 8.6, 1.0), color.hsl());
    }

    #[test]
    fn convert_hsl_to_rgb() {
        let color = Color::from_hsl(122.0, 33.0, 12.0, 0.4);

        assert_eq!((21.0, 41.0, 21.0, 0.4), color.rgb());
    }
}
