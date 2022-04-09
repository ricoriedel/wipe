use anyhow::Error;
use clap::Parser;
use clap::ArgEnum;
use crossterm::style::Color::*;
use crate::animation::Animation;
use crate::animation::circle::CircleAnimation;
use crate::color::{ColorSampler, SimpleColorSampler};
use crate::fill::circle::CircleFillMode;
use crate::fill::FillMode;
use crate::fill::level::LevelFillMode;
use crate::vec::Vector;

mod color;
mod char;
mod fill;
mod vec;
mod array;
mod surface;
mod animation;
mod sampler;

#[derive(Copy, Clone, ArgEnum)]
enum AnimationType {
    Circle
}

#[derive(Copy, Clone, ArgEnum)]
enum ColorType {
    Red,
    Green,
    Blue,
    LightRed,
    LightGreen,
    LightBlue,
    Grey,
    Rainbow,
}

#[derive(Copy, Clone, ArgEnum)]
enum FillModeType {
    Circle,
    Level
}

#[derive(Parser)]
#[clap(author = "Rico Riedel", version = "0.1.0", about = "Wipe your terminal with a random animation.")]
struct Args {
    #[clap(short, long, help = "Add animation", arg_enum)]
    animation: Vec<AnimationType>,
    #[clap(short, long, help = "Add fill mode", arg_enum)]
    fill: Vec<FillModeType>,
    #[clap(short, long, help = "Add color pallet", arg_enum)]
    color: Vec<ColorType>,
    #[clap(long, default_value = ".-+%#", help = "Set chars")]
    chars: String,
    #[clap(long, help = "Set width [default: terminal width]")]
    width: Option<usize>,
    #[clap(long, help = "Set height [default: terminal height]")]
    height: Option<usize>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let terminal = crossterm::terminal::size()?;
    let width = args.width.unwrap_or(terminal.0 as usize);
    let height = args.width.unwrap_or(terminal.1 as usize);
    let size = Vector::from_terminal(width, height);

    let animation = create_animation(args.animation[0], size);
    let fill = create_fill(args.fill[0], size);
    let color = create_color(args.color[0]);

    Ok(())
}

fn create_animation(animation: AnimationType, size: Vector) -> Box<dyn Animation> {
    match animation {
        AnimationType::Circle => Box::new(CircleAnimation::new(size))
    }
}

fn create_fill(fill: FillModeType, size: Vector) -> Box<dyn FillMode> {
    match fill {
        FillModeType::Circle => Box::new(CircleFillMode::new(size)),
        FillModeType::Level => Box::new(LevelFillMode::new()),
    }
}

fn create_color(color: ColorType) -> Box<dyn ColorSampler> {
    match color {
        ColorType::Red => Box::new(SimpleColorSampler::new(vec![Yellow, DarkYellow, Red])),
        ColorType::Green => Box::new(SimpleColorSampler::new(vec![Cyan, DarkGreen, Green])),
        ColorType::Blue => Box::new(SimpleColorSampler::new(vec![Magenta, DarkBlue, Blue])),
        ColorType::LightRed => Box::new(SimpleColorSampler::new(vec![White, Yellow, Red])),
        ColorType::LightGreen => Box::new(SimpleColorSampler::new(vec![White, Cyan, Green])),
        ColorType::LightBlue => Box::new(SimpleColorSampler::new(vec![White, Blue, Magenta])),
        ColorType::Grey => Box::new(SimpleColorSampler::new(vec![Black, Grey, White])),
        ColorType::Rainbow => Box::new(SimpleColorSampler::new(vec![Magenta, Blue, Green, Yellow, Red]))
    }
}