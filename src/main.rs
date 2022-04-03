use clap::Parser;
use crate::color::factory::ColorEnum;
use crate::fill::factory::FillModeEnum;

mod color;
mod char;
mod fill;
mod vec;
mod array;

#[derive(Parser)]
#[clap(author = "Rico Riedel", version = "0.1.0", about = "Wipe your terminal with a random animation.")]
struct Args {
    #[clap(short, long, help = "Add fill mode", arg_enum)]
    fill: Vec<FillModeEnum>,
    #[clap(short, long, help = "Add color color", arg_enum)]
    pallet: Vec<ColorEnum>,
    #[clap(long, default_value = ".-+%#", help = "Set chars")]
    chars: String
}

fn main() {
    Args::parse();
}
