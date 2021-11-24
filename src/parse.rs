use crate::colors::HslColor;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0, space1},
    combinator::{map, opt},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn comma_or_spaces(input: &str) -> IResult<&str, ()> {
    map(alt((delimited(space0, tag(","), space0), space1)), |_| ())(input)
}

#[derive(Debug, PartialEq)]
enum Angle {
    Degrees(f32),
    Radians(f32),
    Gradians(f32),
    Turns(f32),
}

impl Angle {
    fn to_degrees(self) -> f32 {
        match self {
            Self::Degrees(deg) => deg,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_comma_or_spaces() {
        assert_eq!(comma_or_spaces("    aoe").unwrap().0, "aoe");
        assert_eq!(comma_or_spaces(",  ").unwrap().0, "");
        assert_eq!(comma_or_spaces(" , ").unwrap().0, "");
        assert_eq!(comma_or_spaces("  ,").unwrap().0, "");
        assert_eq!(comma_or_spaces(", ").unwrap().0, "");
        assert_eq!(comma_or_spaces("   ").unwrap().0, "");
        assert_eq!(comma_or_spaces("    ").unwrap().0, "");

        assert!(comma_or_spaces("l ").is_err());
    }

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
}
