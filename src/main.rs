use anyhow::anyhow;
use color::colors::Color;
use std::{error::Error, str::FromStr};
use structopt::StructOpt;

#[derive(Debug)]
enum ColorType {
    Hsl,
    Rgb,
}

impl FromStr for ColorType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "hsl" | "hsla" => Ok(Self::Hsl),
            "rgb" | "rgba" => Ok(Self::Rgb),
            _ => Err(anyhow!("No such color")),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "color", about = "A utility for converting and picking colours")]
struct Opt {
    color: String,
    #[structopt(short = "o", long = "output", default_value = "rgb")]
    output: ColorType,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opt { color, output } = Opt::from_args();

    let color = Color::parse_from_str(&color)?;
    let out_color = match output {
        ColorType::Hsl => color.to_hsl_string(),
        ColorType::Rgb => color.to_rgb_string(),
    };

    println!("{}", out_color);

    Ok(())
}
