use std::io::stdout;
use std::time::Duration;
use anyhow::Error;
use clap::Parser;
use clap::ArgEnum;
use rand::rngs::OsRng;
use crate::animation::Animation;
use crate::animation::circle::CircleAnimation;
use crate::char::SimpleCharSampler;
use crate::choose::{Chooser, Options};
use crate::color::{ColorSampler, SimpleColorSampler};
use crate::fill::circle::CircleFillMode;
use crate::fill::FillMode;
use crate::fill::level::LevelFillMode;
use crate::render::{Renderer, SamplerRenderer};
use crate::runner::Runner;
use crate::sampler::ComposedSampler;
use crate::surface::WriteSurface;
use crate::timer::SimpleTimer;
use crate::vec::Vector;

mod color;
mod char;
mod fill;
mod vec;
mod array;
mod surface;
mod animation;
mod sampler;
mod render;
mod timer;
mod runner;
mod choose;

#[derive(Copy, Clone, ArgEnum)]
enum AnimationType {
    Circle
}

impl Options for AnimationType {
    fn all() -> Vec<Self> where Self: Sized {
        use AnimationType::*;

        vec![Circle]
    }
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

impl Options for ColorType {
    fn all() -> Vec<Self> where Self: Sized {
        use ColorType::*;

        vec![Red, Green, Blue, LightRed, LightGreen, LightBlue, Grey, Rainbow]
    }
}

#[derive(Copy, Clone, ArgEnum)]
enum FillModeType {
    Circle,
    Level
}

impl Options for FillModeType {
    fn all() -> Vec<Self> where Self: Sized {
        use FillModeType::*;

        vec![Circle, Level]
    }
}

#[derive(Parser)]
#[clap(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[clap(short, long, help = "Add animation", arg_enum)]
    animation: Vec<AnimationType>,
    #[clap(short, long, help = "Add fill mode", arg_enum)]
    fill: Vec<FillModeType>,
    #[clap(short, long, help = "Add color pallet", arg_enum)]
    color: Vec<ColorType>,
    #[clap(long, default_value = ".-+%#", help = "Set chars")]
    chars: String,
    #[clap(long, default_value = "30", help = "Set frames per second")]
    fps: u64,
    #[clap(long, default_value = "1000", help = "Set duration [milliseconds]")]
    duration: u64,
    #[clap(long, help = "Set width [default: terminal width]")]
    width: Option<usize>,
    #[clap(long, help = "Set height [default: terminal height]")]
    height: Option<usize>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut chooser = Chooser::new(OsRng::default());

    let terminal = crossterm::terminal::size()?;
    let width = args.width.unwrap_or(terminal.0 as usize);
    let height = args.height.unwrap_or(terminal.1 as usize);
    let size = Vector::from_terminal(width, height);
    let delay = Duration::from_micros(1_000_000 / args.fps);
    let duration = Duration::from_millis(args.duration);

    let animation = create_animation(chooser.choose(args.animation), size);
    let fill = create_fill(chooser.choose(args.fill), size);
    let color = create_color(chooser.choose(args.color));
    let char = Box::new(SimpleCharSampler::new(args.chars));

    let sampler = ComposedSampler::new(animation, fill, color, char);
    let surface = WriteSurface::new(stdout(), width, height);

    let renderer = SamplerRenderer::new(surface, sampler);
    let timer = SimpleTimer::new(delay);
    let runner = Runner::new(duration, timer, renderer);

    runner.run()
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
    use crossterm::style::Color::*;

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