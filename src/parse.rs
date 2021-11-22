use crate::colors::HslColor;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map,
    number::complete::float,
    sequence::{delimited, tuple},
    IResult,
};

fn comma_or_spaces(input: &str) -> IResult<&str, ()> {
    map(alt((space1, delimited(space0, tag(","), space0))), |_| ())(input)
}
fn float_list_of_three(input: &str) -> IResult<&str, (f32, f32, f32)> {
    let parser = delimited(
        space0,
        tuple((float, comma_or_spaces, float, comma_or_spaces, float)),
        space0,
    );

    map(parser, |(f1, _, f2, _, f3)| (f1, f2, f3))(input)
}

fn hsl_color(input: &str) -> IResult<&str, HslColor> {
    let (input, (hue, saturation, luminosity)) =
        delimited(tag("hsl("), float_list_of_three, tag(")"))(input)?;

    Ok((input, HslColor::new(hue, saturation, luminosity)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_comma_or_spaces() {
        comma_or_spaces("    aoe").unwrap();
        comma_or_spaces(",  ").unwrap();
        comma_or_spaces(" , ").unwrap();
        comma_or_spaces("  ,").unwrap();
        comma_or_spaces(", ").unwrap();
        comma_or_spaces("   ").unwrap(); // Tab

        assert!(comma_or_spaces("l ").is_err());
    }

    #[test]
    fn parse_float_list() {
        let (rest, output) = float_list_of_three("32,11.22,04oeeooe").unwrap();
        assert_eq!(output, (32.0, 11.22, 4.0));
        assert_eq!(rest, "oeeooe")
    }

    #[test]
    fn parse_hsl() {
        let (_, color) = hsl_color("hsl(212, 12, 24.2)").unwrap();

        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsl(212 12  24.2)").unwrap();
        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));

        let (_, color) = hsl_color("hsl(  212 12  24.2)").unwrap();
        assert_eq!(color, HslColor::new(212.0, 12.0, 24.2));
    }
}
