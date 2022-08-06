pub mod convert;
pub mod pattern;
pub mod transform;

mod error;
mod printer;
mod renderer;
mod term;
mod timer;
mod vec;

pub use error::*;
pub use printer::*;
pub use renderer::*;
pub use term::*;
pub use timer::*;
pub use vec::*;

use crate::convert::*;
use crate::pattern::*;
use crate::transform::*;
use clap::builder::NonEmptyStringValueParser;
use clap::{value_parser, Parser, ValueEnum};
use crossterm::style::Color;
use crossterm::style::Color::*;
use rand::prelude::*;
use std::io::stdout;
use std::time::Duration;

/// The command line arguments.
#[derive(Parser, Default)]
#[clap(
    author  = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about   = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Args {
    /// Set the duration as milliseconds
    #[clap(
        long,
        default_value_t = 2000,
        value_parser = value_parser!(u64).range(0..=60_000),
        help = "Set the animation duration [milliseconds]"
    )]
    duration: u64,
    /// Set the frames per second
    #[clap(long, default_value_t = 60, value_parser = value_parser!(u64).range(1..=480))]
    fps: u64,
    /// Set the chars used to model the pattern
    #[clap(long, default_value = ".:+#", value_parser = NonEmptyStringValueParser::new())]
    chars: String,
    /// Set the pattern
    #[clap(long, value_enum)]
    char_pattern: Option<PatternEnum>,
    /// Invert the pattern
    #[clap(long)]
    char_invert: Option<bool>,
    /// Swap the x-axis and y-axis of the pattern
    #[clap(long)]
    char_swap: Option<bool>,
    /// Set the segment count of the pattern [default: 1-4]
    #[clap(long, value_parser = value_parser!(u8).range(1..255))]
    char_segments: Option<u8>,
    /// Set the slice count of the pattern  [default: 1-4]
    #[clap(long, value_parser = value_parser!(u8).range(1..255))]
    char_slices: Option<u8>,
    /// Set the colors used to fill the pattern
    #[clap(long, value_enum)]
    colors: Option<PalletEnum>,
    /// Set the fill pattern
    #[clap(long, value_enum)]
    color_pattern: Option<PatternEnum>,
    /// Choose if the fill pattern should move
    #[clap(long)]
    color_shift: Option<bool>,
    /// Invert the fill pattern
    #[clap(long)]
    color_invert: Option<bool>,
    /// Swap the x-axis and y-axis of the fill pattern
    #[clap(long)]
    color_swap: Option<bool>,
    /// Set the slice count of the fill pattern [default: 1-4]
    #[clap(long, value_parser = value_parser!(u8).range(1..255))]
    color_slices: Option<u8>,
}

/// All color pallets.
#[derive(ValueEnum, Copy, Clone)]
enum PalletEnum {
    Red,
    Yellow,
    Green,
    Blue,
    Magenta,
    Cyan,
    Rainbow,

    DarkRed,
    DarkYellow,
    DarkGreen,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    DarkRainbow,

    RedYellow,
    YellowGreen,
    GreenBlue,
    BlueCyan,
    CyanMagenta,
    MagentaRed,

    Gray,
}

/// All possible [Pattern]s.
#[derive(ValueEnum, Copy, Clone, PartialEq, Debug)]
enum PatternEnum {
    Circle,
    Line,
    Rhombus,
    Wheel,
}

/// A configuration for a composed [Pattern].
#[derive(derive_more::Constructor)]
struct PatternConfig {
    pattern: PatternEnum,
    shift: bool,
    invert: bool,
    swap: bool,
    segments: f32,
    slices: f32,
}

impl Args {
    /// Returns the configuration for the char [Pattern].
    fn char_config(&self, rng: &mut impl Rng) -> PatternConfig {
        PatternConfig::new(
            choose(self.char_pattern, rng),
            true,
            self.char_invert.unwrap_or(rng.gen()),
            self.char_swap.unwrap_or(rng.gen()),
            self.char_segments.unwrap_or(rng.gen_range(1..=4)) as f32,
            self.char_slices.unwrap_or(rng.gen_range(1..=4)) as f32,
        )
    }

    /// Returns the configuration for the color [Pattern].
    fn color_config(&self, rng: &mut impl Rng) -> PatternConfig {
        PatternConfig::new(
            choose(self.color_pattern, rng),
            self.color_shift.unwrap_or(rng.gen()),
            self.color_invert.unwrap_or(rng.gen()),
            self.color_swap.unwrap_or(rng.gen()),
            1.0,
            self.color_slices.unwrap_or(rng.gen_range(1..=4)) as f32,
        )
    }

    /// Returns the colors for the [ColorConverter].
    fn pallet(&self, rng: &mut impl Rng) -> Vec<Color> {
        match choose(self.colors, rng) {
            PalletEnum::Red => vec![DarkRed, Red, White],
            PalletEnum::Yellow => vec![DarkYellow, Yellow, White],
            PalletEnum::Green => vec![DarkGreen, Green, White],
            PalletEnum::Blue => vec![DarkBlue, Blue, White],
            PalletEnum::Magenta => vec![DarkMagenta, Magenta, White],
            PalletEnum::Cyan => vec![DarkCyan, Cyan, White],
            PalletEnum::Rainbow => vec![Red, Yellow, Green, Blue, Cyan, Magenta],

            PalletEnum::DarkRed => vec![Black, DarkRed, Red],
            PalletEnum::DarkYellow => vec![Black, DarkYellow, Yellow],
            PalletEnum::DarkGreen => vec![Black, DarkGreen, Green],
            PalletEnum::DarkBlue => vec![Black, DarkBlue, Blue],
            PalletEnum::DarkMagenta => vec![Black, DarkMagenta, Magenta],
            PalletEnum::DarkCyan => vec![Black, DarkCyan, Cyan],
            PalletEnum::DarkRainbow => vec![
                DarkRed,
                DarkYellow,
                DarkGreen,
                DarkBlue,
                DarkCyan,
                DarkMagenta,
            ],

            PalletEnum::RedYellow => vec![Red, DarkRed, DarkYellow, Yellow],
            PalletEnum::YellowGreen => vec![Yellow, DarkYellow, DarkGreen, Green],
            PalletEnum::GreenBlue => vec![Green, DarkGreen, DarkBlue, Blue],
            PalletEnum::BlueCyan => vec![Blue, DarkBlue, DarkCyan, Cyan],
            PalletEnum::CyanMagenta => vec![Cyan, DarkCyan, DarkMagenta, Magenta],
            PalletEnum::MagentaRed => vec![Magenta, DarkMagenta, DarkRed, Red],

            PalletEnum::Gray => vec![Black, DarkGrey, Grey, White],
        }
    }

    /// Returns the duration for the [Timer].
    fn duration(&self) -> Duration {
        Duration::from_millis(self.duration)
    }

    /// Returns the delay for the [Timer].
    fn delay(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.fps)
    }
}

impl PatternConfig {
    /// Creates a new base [Pattern].
    fn create_base(&self) -> Box<dyn PatternFactory> {
        match self.pattern {
            PatternEnum::Circle => Box::new(CircleFactory::new()),
            PatternEnum::Line => Box::new(LineFactory::new()),
            PatternEnum::Rhombus => Box::new(RhombusFactory::new()),
            PatternEnum::Wheel => Box::new(WheelFactory::new()),
        }
    }

    /// Creates a new composed [Pattern].
    fn create(&self) -> Box<dyn PatternFactory> {
        let mut pattern = self.create_base();

        if self.shift {
            pattern = Box::new(ShiftFactory::new(pattern))
        }
        if self.invert {
            pattern = Box::new(InvertFactory::new(pattern))
        }
        if self.swap {
            pattern = Box::new(SwapFactory::new(pattern))
        }
        if self.segments != 1.0 {
            pattern = Box::new(SegmentsFactory::new(pattern, self.segments));
        }
        if self.slices != 1.0 {
            pattern = Box::new(SliceFactory::new(pattern, self.slices));
        }
        pattern
    }
}

/// Returns the value of the [Option] or a random enum variant.
fn choose<TValue: ValueEnum, TRand: Rng>(opt: Option<TValue>, rng: &mut TRand) -> TValue {
    match opt {
        Some(value) => value.clone(),
        None => TValue::value_variants().iter().choose(rng).unwrap().clone(),
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let rand = &mut thread_rng();

    let char = args.char_config(rand).create();
    let color = args.color_config(rand).create();
    let pallet = args.pallet(rand);
    let duration = args.duration();
    let delay = args.delay();

    let sampler = SamplerFactoryImpl::new(char, color);
    let char_converter = CharConverterImpl::new(args.chars);
    let color_converter = ColorConverterImpl::new(pallet);
    let converter = ConverterImpl::new(char_converter, color_converter);
    let term = TerminalImpl::new(stdout());
    let printer = PrinterImpl::new(term)?;
    let renderer = RendererImpl::new(sampler, converter, printer);

    let clock = ClockImpl::new();
    let timer = Timer::new(clock, duration, delay);

    timer.run(renderer)
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn args_pallet_all_defined() {
        let rand = &mut StepRng::new(1, 1);

        for value in PalletEnum::value_variants() {
            let args = Args {
                colors: Some(*value),
                ..Args::default()
            };
            assert!(args.pallet(rand).len() > 0);
        }
    }

    #[test]
    fn duration() {
        let args = Args {
            duration: 3500,
            ..Args::default()
        };
        assert_eq!(Duration::from_secs_f32(3.5), args.duration());
    }

    #[test]
    fn delay() {
        let args = Args {
            fps: 20,
            ..Args::default()
        };
        assert_eq!(Duration::from_secs_f32(0.05), args.delay());
    }

    #[test]
    fn char_config_pattern() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            char_pattern: Some(PatternEnum::Line),
            ..Args::default()
        };
        assert_eq!(PatternEnum::Line, args.char_config(rng).pattern);
    }

    #[test]
    fn char_config_invert() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            char_invert: Some(false),
            ..Args::default()
        };
        assert_eq!(false, args.char_config(rng).invert);
    }

    #[test]
    fn char_config_shift() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args::default();

        assert_eq!(true, args.char_config(rng).shift);
    }

    #[test]
    fn char_config_swap() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            char_swap: Some(true),
            ..Args::default()
        };
        assert_eq!(true, args.char_config(rng).swap);
    }

    #[test]
    fn char_config_segments() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            char_segments: Some(12),
            ..Args::default()
        };
        assert_abs_diff_eq!(12.0, args.char_config(rng).segments);
    }

    #[test]
    fn char_config_slices() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            char_slices: Some(42),
            ..Args::default()
        };
        assert_abs_diff_eq!(42.0, args.char_config(rng).slices);
    }

    #[test]
    fn color_config_pattern() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            color_pattern: Some(PatternEnum::Circle),
            ..Args::default()
        };
        assert_eq!(PatternEnum::Circle, args.color_config(rng).pattern);
    }

    #[test]
    fn color_config_invert() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            color_invert: Some(true),
            ..Args::default()
        };
        assert_eq!(true, args.color_config(rng).invert);
    }

    #[test]
    fn color_config_shift() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            color_shift: Some(false),
            ..Args::default()
        };
        assert_eq!(false, args.color_config(rng).shift);
    }

    #[test]
    fn color_config_swap() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            color_swap: Some(true),
            ..Args::default()
        };
        assert_eq!(true, args.color_config(rng).swap);
    }

    #[test]
    fn color_config_segments() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args::default();

        assert_abs_diff_eq!(1.0, args.color_config(rng).segments);
    }

    #[test]
    fn color_config_slices() {
        let rng = &mut StepRng::new(1, 1);
        let args = Args {
            color_slices: Some(23),
            ..Args::default()
        };
        assert_abs_diff_eq!(23.0, args.color_config(rng).slices);
    }

    #[test]
    fn pattern_config_all_defined() {
        for value in PatternEnum::value_variants() {
            let config = PatternConfig {
                pattern: *value,
                shift: true,
                invert: true,
                swap: true,
                segments: 3.0,
                slices: 2.0,
            };
            config
                .create()
                .create(&Config::default())
                .sample(Vector::default());
        }
    }
}
