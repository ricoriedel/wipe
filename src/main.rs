use std::io::stdout;
use std::time::Duration;
use anyhow::{anyhow, Error};
use clap::Parser;
use clap::ArgEnum;
use rand::rngs::OsRng;
use crate::animation::Animation;
use crate::animation::circle::CircleAnimation;
use crate::animation::rhombus::RhombusAnimation;
use crate::animation::sonar::SonarAnimation;
use crate::char::SimpleCharSampler;
use crate::choose::{Chooser, Collection};
use crate::color::{ColorSampler, SimpleColorSampler};
use crate::fill::circle::CircleFillMode;
use crate::fill::FillMode;
use crate::fill::level::LevelFillMode;
use crate::fill::stripes::StripesFillMode;
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

/// Defines an enum and implements the [Collection] trait.
macro_rules! options {
    ($name:ident { $($opt:ident,)* }) => {
        #[derive(Copy, Clone, ArgEnum)]
        enum $name {
            $($opt,)*
        }
        impl Collection for $name {
            fn all() -> Vec<Self> {
                vec![$($name::$opt,)*]
            }
        }
    }
}

options!(AnimationType {
    Circle,
    Rhombus,
    Sonar,
});

options!(ColorType {
    Red,
    Green,
    Blue,
    LightRed,
    LightGreen,
    LightBlue,
    Grey,
    Rainbow,
});

options!(FillModeType {
    Circle,
    Level,
    Stripes,
});

const MAX_FPS: u64 = 480;

/// The program arguments.
#[derive(Parser)]
#[clap(author = env ! ("CARGO_PKG_AUTHORS"), version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[clap(short, long, help = "Add animation", arg_enum)]
    animation: Vec<AnimationType>,
    #[clap(short, long, help = "Add fill mode", arg_enum)]
    fill: Vec<FillModeType>,
    #[clap(short, long, help = "Add color pallet", arg_enum)]
    color: Vec<ColorType>,
    #[clap(long, default_value = ".-+%#", parse(try_from_str = validate_chars), help = "Set chars")]
    chars: String,
    #[clap(long, default_value = "30", parse(try_from_str = validate_fps), help = "Set frames per second [max: 480]")]
    fps: u64,
    #[clap(long, default_value = "1000", parse(try_from_str = validate_duration), help = "Set duration [milliseconds]")]
    duration: u64,
    #[clap(long, help = "Set width [default: terminal width]")]
    width: Option<usize>,
    #[clap(long, help = "Set height [default: terminal height]")]
    height: Option<usize>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut chooser = Chooser::new(OsRng::default());

    let (width, height) = size(crossterm::terminal::size()?, args.width, args.height);
    let size = Vector::from_terminal(width, height);
    let delay = delay_of_fps(args.fps);
    let duration = Duration::from_millis(args.duration);

    let animation = create_animation(chooser.choose(args.animation), size);
    let fill = create_fill(chooser.choose(args.fill), size);
    let color = create_color(chooser.choose(args.color));
    let char = Box::new(SimpleCharSampler::new(args.chars));

    let sampler = ComposedSampler::new(animation, fill, color, char);
    let surface = WriteSurface::new(stdout(), width, height)?;

    let renderer = SamplerRenderer::new(surface, sampler);
    let timer = SimpleTimer::new(delay);
    let runner = Runner::new(duration, timer, renderer);

    runner.run()
}

/// Validates the chars argument.
fn validate_chars(text: &str) -> Result<String, Error> {
    if text.is_empty() {
        Err(anyhow!("can't be empty."))
    } else {
        Ok(text.to_string())
    }
}

/// Validates the fps argument.
fn validate_fps(text: &str) -> Result<u64, Error> {
    let value = text.parse()?;

    if value > MAX_FPS {
        Err(anyhow!("value is above limit of {}.", MAX_FPS))
    } else if value == 0 {
        Err(anyhow!("value is zero."))
    } else {
        Ok(value)
    }
}

/// Validates the duration argument.
fn validate_duration(text: &str) -> Result<u64, Error> {
    let value = text.parse()?;

    if value == 0 {
        Err(anyhow!("value is zero."))
    } else {
        Ok(value)
    }
}

/// Returns the size to use based on the terminal size and width and height arguments.
fn size(terminal: (u16, u16), width: Option<usize>, height: Option<usize>) -> (usize, usize) {
    let width = width.unwrap_or(terminal.0 as usize);
    let height = height.unwrap_or(terminal.1 as usize);
    (width, height)
}

/// Calculates the delay between frames based on the fps.
fn delay_of_fps(fps: u64) -> Duration {
    Duration::from_nanos(1_000_000_000 / fps)
}

fn create_animation(animation: AnimationType, size: Vector) -> Box<dyn Animation> {
    match animation {
        AnimationType::Circle => Box::new(CircleAnimation::new(size)),
        AnimationType::Rhombus => Box::new(RhombusAnimation::new(size)),
        AnimationType::Sonar => Box::new(SonarAnimation::new(size)),
    }
}

fn create_fill(fill: FillModeType, size: Vector) -> Box<dyn FillMode> {
    match fill {
        FillModeType::Circle => Box::new(CircleFillMode::new(size)),
        FillModeType::Level => Box::new(LevelFillMode::new()),
        FillModeType::Stripes => Box::new(StripesFillMode::new(size))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_chars_some_string() {
        assert_eq!("abc", &validate_chars("abc").unwrap());
    }

    #[test]
    fn validate_chars_empty() {
        assert!(validate_chars("").is_err());
    }

    #[test]
    fn validate_fps_some_string() {
        assert_eq!(35, validate_fps("35").unwrap());
    }

    #[test]
    fn validate_fps_zero() {
        assert!(validate_fps("0").is_err());
    }

    #[test]
    fn validate_fps_above_max() {
        assert!(validate_fps("500").is_err());
    }

    #[test]
    fn validate_duration_some_string() {
        assert_eq!(500, validate_duration("500").unwrap());
    }

    #[test]
    fn validate_duration_zero() {
        assert!(validate_duration("0").is_err());
    }

    #[test]
    fn size_not_set() {
        assert_eq!((4, 6), size((4, 6), None, None));
    }

    #[test]
    fn size_width_set() {
        assert_eq!((8, 3), size((2, 3), Some(8), None));
    }

    #[test]
    fn size_height_set() {
        assert_eq!((1, 6), size((1, 7), None, Some(6)));
    }

    #[test]
    fn delay_of_fps_some_number() {
        assert_eq!(Duration::from_nanos(10_526_315), delay_of_fps(95));
    }

    #[test]
    fn create_animation_all_implemented() {
        let size = Vector::new(0.0, 0.0);

        create_animation(AnimationType::Circle, size);
        create_animation(AnimationType::Rhombus, size);
        create_animation(AnimationType::Sonar, size);
    }

    #[test]
    fn create_fill_all_implemented() {
        let size = Vector::new(0.0, 0.0);

        create_fill(FillModeType::Circle, size);
        create_fill(FillModeType::Level, size);
        create_fill(FillModeType::Stripes, size);
    }

    #[test]
    fn create_color_all_implemented() {
        create_color(ColorType::Red);
        create_color(ColorType::Green);
        create_color(ColorType::Blue);
        create_color(ColorType::LightRed);
        create_color(ColorType::LightGreen);
        create_color(ColorType::LightBlue);
        create_color(ColorType::Grey);
        create_color(ColorType::Rainbow);
    }
}