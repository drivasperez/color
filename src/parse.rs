use crate::colors::{HslColor, OldColor, RgbColor};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{eof, map, opt},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Angle {
    Degrees(f32),
    Radians(f32),
    Gradians(f32),
    Turns(f32),
}

impl Angle {
    fn to_degrees(&self) -> f32 {
        match self {
            Self::Degrees(deg) => *deg,
            Self::Radians(rad) => rad.to_degrees(),
            Self::Gradians(grad) => unimplemented!(),
            Self::Turns(turns) => turns * 360.0,
        }
    }
}

fn angle(input: &str) -> IResult<&str, Angle> {
    let parser = tuple((
        float,
        opt(alt((tag("deg"), tag("rad"), tag("grad"), tag("turn")))),
    ));

    map(parser, |(val, unit)| match unit {
        None | Some("deg") => Angle::Degrees(val),
        Some("rad") => Angle::Radians(val),
        Some("grad") => Angle::Gradians(val),
        Some("turn") => Angle::Turns(val),
        _ => unreachable!(),
    })(input)
}

fn percentage(input: &str) -> IResult<&str, f32> {
    terminated(float, tag("%"))(input)
}

fn hsl_values(input: &str) -> IResult<&str, (Angle, f32, f32, Option<f32>)> {
    let parser_commas = delimited(
        space0,
        tuple((
            angle,
            delimited(space0, tag(","), space0),
            alt((percentage, float)),
            delimited(space0, tag(","), space0),
            alt((percentage, float)),
            opt(preceded(
                delimited(space0, tag(","), space0),
                alt((map(percentage, |p| p / 100.0), float)),
            )),
        )),
        space0,
    );

    let parser_spaces = delimited(
        space0,
        tuple((
            angle,
            space1,
            alt((percentage, float)),
            space1,
            alt((percentage, float)),
            opt(preceded(
                delimited(space0, tag("/"), space0),
                alt((map(percentage, |p| p / 100.0), float)),
            )),
        )),
        space0,
    );
    let parser = alt((parser_commas, parser_spaces));
    map(parser, |(angle, _, sat, _, lum, alpha)| {
        (angle, sat, lum, alpha)
    })(input)
}

fn hsl_color(input: &str) -> IResult<&str, HslColor> {
    let (input, (hue, saturation, luminosity, alpha)) = preceded(
        alt((tag("hsla"), tag("hsl"))),
        delimited(tag("("), hsl_values, tag(")")),
    )(input)?;

    Ok((
        input,
        if let Some(a) = alpha {
            HslColor::hsla(hue.to_degrees(), saturation, luminosity, a)
        } else {
            HslColor::new(hue.to_degrees(), saturation, luminosity)
        },
    ))
}

fn comma_separated_percentages(input: &str) -> IResult<&str, (f32, f32, f32, Option<f32>)> {
    map(
        tuple((
            percentage,
            delimited(space0, tag(","), space0),
            percentage,
            delimited(space0, tag(","), space0),
            percentage,
            opt(preceded(delimited(space0, tag(","), space0), percentage)),
        )),
        |(p1, _, p2, _, p3, p4)| {
            (
                percentage_to_color_255(p1),
                percentage_to_color_255(p2),
                percentage_to_color_255(p3),
                (p4.map(|x| x / 100.0)),
            )
        }, // TODO: Convert these percentages to 0-255 floats
    )(input)
}

fn percentage_to_color_255(input: f32) -> f32 {
    (input / 100.0) * 255.0
}

fn space_separated_percentages(input: &str) -> IResult<&str, (f32, f32, f32, Option<f32>)> {
    map(
        tuple((
            percentage,
            space1,
            percentage,
            space1,
            percentage,
            opt(preceded(delimited(space0, tag("/"), space0), percentage)),
        )),
        |(p1, _, p2, _, p3, p4)| {
            (
                percentage_to_color_255(p1),
                percentage_to_color_255(p2),
                percentage_to_color_255(p3),
                (p4.map(|x| x / 100.0)),
            )
        },
    )(input)
}

fn comma_separated_floats(input: &str) -> IResult<&str, (f32, f32, f32, Option<f32>)> {
    map(
        tuple((
            float,
            delimited(space0, tag(","), space0),
            float,
            delimited(space0, tag(","), space0),
            float,
            opt(preceded(delimited(space0, tag(","), space0), float)),
        )),
        |(p1, _, p2, _, p3, p4)| (p1, p2, p3, p4),
    )(input)
}

fn space_separated_floats(input: &str) -> IResult<&str, (f32, f32, f32, Option<f32>)> {
    map(
        tuple((
            float,
            space1,
            float,
            space1,
            float,
            opt(preceded(delimited(space0, tag("/"), space0), float)),
        )),
        |(p1, _, p2, _, p3, p4)| (p1, p2, p3, p4),
    )(input)
}

fn rgb_values(input: &str) -> IResult<&str, (f32, f32, f32, Option<f32>)> {
    // Combine the above with alpha values
    alt((
        comma_separated_percentages,
        comma_separated_floats,
        space_separated_percentages,
        space_separated_floats,
    ))(input)
}

fn rgb_color(input: &str) -> IResult<&str, RgbColor> {
    let (input, (red, green, blue, alpha)) = preceded(
        alt((tag("rgba"), tag("rgb"))),
        delimited(tag("("), rgb_values, tag(")")),
    )(input)?;

    Ok((
        input,
        if let Some(a) = alpha {
            RgbColor::rgba(red, green, blue, a)
        } else {
            RgbColor::new(red, green, blue)
        },
    ))
}

pub fn parse_color(input: &str) -> IResult<&str, OldColor> {
    terminated(
        alt((map(hsl_color, OldColor::Hsl), map(rgb_color, OldColor::Rgb))),
        eof,
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_hsl_values() {
        let (rest, output) = hsl_values("32,11.22,04oeeooe").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = hsl_values("32deg, 11.22,04oeeooe").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = hsl_values("32deg   , 11.22,04oeeooe").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = hsl_values("360rad 12% 34").unwrap();
        assert_eq!(output, (Angle::Radians(360.0), 12.0, 34.0, None));
        assert_eq!(rest, "");

        // Can't mix and match separators
        assert!(hsl_values("354rad 12%, 34").is_err());
        assert!(hsl_values("354rad, 12% 34").is_err());
    }

    #[test]
    fn parse_hsla_values() {
        let (_, output) = hsl_values("32,11.22,4.0,0.2").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, Some(0.2)));

        let (_, output) = hsl_values("32 11.22 4.0 / 0.2").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, Some(0.2)));

        let (_, output) = hsl_values("32,11.22,4.0, 20%").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, Some(0.2)));

        let (_, output) = hsl_values("32 11.22 4.0 / 50%").unwrap();
        assert_eq!(output, (Angle::Degrees(32.0), 11.22, 4.0, Some(0.5)));
    }

    #[test]
    fn parse_hsl() {
        let (_, color) = hsl_color("hsl(212, 12, 24.2)").unwrap();

        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsla(212, 12, 24.2)").unwrap();
        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsl(212 12  24.2)").unwrap();
        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsl(  212 12  24.2)").unwrap();
        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsl(2turn 24.3 4%)").unwrap();
        // Clamped at max 360
        assert_eq!(color, HslColor::new(360.0, 24.3, 4.0));

        let (_, color) = hsl_color("hsl(2turn, -24.3, 101%)").unwrap();
        // Clamped at max 360, max 100, max 100, min 0
        assert_eq!(color, HslColor::new(360.0, 0.0, 100.0));
    }

    #[test]
    fn parse_hsl_with_transparency() {
        let (_, color) = hsl_color("hsla(212 12 24.2 / 0.3)").unwrap();
        assert_eq!(color, HslColor::hsla(212.0, 12.0, 24.2, 0.3));

        let (_, color) = hsl_color("hsl(212, 12, 24.2 , 0.3)").unwrap();
        assert_eq!(color, HslColor::hsla(212.0, 12.0, 24.2, 0.3));

        let (_, color) = hsl_color("hsla(212 12 24.2 / 30%)").unwrap();
        assert_eq!(color, HslColor::hsla(212.0, 12.0, 24.2, 0.3));

        let (_, color) = hsl_color("hsl(212, 12, 24.2 , 30%)").unwrap();
        assert_eq!(color, HslColor::hsla(212.0, 12.0, 24.2, 0.3));

        // Can't have transparency slash and commas
        assert!(hsl_color("hsl(21deg, 32.2, 32% / 32%").is_err());
        assert!(hsl_color("hsl(21deg, 32.2, 32% / 32deg").is_err());
    }

    #[test]
    fn parse_rgb_values() {
        let (rest, output) = rgb_values("32,11.22,04oeeooe").unwrap();
        assert_eq!(output, (32.0, 11.22, 4.0, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32%,11.22%,04%oeeooe").unwrap();
        assert_eq!(output, (81.6, 28.611, 10.2, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32 11.22 04oeeooe").unwrap();
        assert_eq!(output, (32.0, 11.22, 4.0, None));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32% 11.22% 04%oeeooe").unwrap();
        assert_eq!(output, (81.6, 28.611, 10.2, None));
        assert_eq!(rest, "oeeooe");

        // Cannot mix and match percentages and floats
        assert!(rgb_values("32, 2%, 225").is_err());
        assert!(rgb_values("32%, 2%, 225").is_err());
    }

    #[test]
    fn parse_rgba_values() {
        let (rest, output) = rgb_values("32,11.22,04,0.2oeeooe").unwrap();
        assert_eq!(output, (32.0, 11.22, 4.0, Some(0.2)));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32%,11.22%,04%,44%oeeooe").unwrap();
        assert_eq!(output, (81.6, 28.611, 10.2, Some(0.44)));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32 11.22 04 / 0.9oeeooe").unwrap();
        assert_eq!(output, (32.0, 11.22, 4.0, Some(0.9)));
        assert_eq!(rest, "oeeooe");

        let (rest, output) = rgb_values("32% 11.22% 04%/7.3%oeeooe").unwrap();
        assert_eq!(output, (81.6, 28.611, 10.2, Some(0.073)));
        assert_eq!(rest, "oeeooe");

        // Cannot mix and match percentages and floats
        assert!(rgb_values("32, 2%, 225 / 1").is_err());
        assert!(rgb_values("32%, 2%, 225, 44%").is_err());
    }
}
