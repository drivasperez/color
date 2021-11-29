use color::colors::{Color, ColorType};
use std::error::Error;
use structopt::StructOpt;

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
                ColorType::Hex => color.hex_string(),
            };
            println!("{}", color);
        }
    } else {
        println!("{}", color);
    }

    Ok(())
}
