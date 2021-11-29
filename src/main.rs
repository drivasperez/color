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
    color: Color,
    #[structopt(short = "o", long = "output")]
    output: Option<Vec<ColorType>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opt { color, output } = Opt::from_args();

    if let Some(v) = output {
        for c in v {
            let color = match c {
                ColorType::Hsl => color.hsl_string(),
                ColorType::Rgb => color.rgb_string(),
            };
            println!("{}", color);
        }
    } else {
        println!("{}", color);
    }

    Ok(())
}
