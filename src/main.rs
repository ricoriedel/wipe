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
use clap::{Parser, ValueEnum};
use crossterm::style::Color;
use crossterm::style::Color::*;
use rand::prelude::*;
use std::io::stdout;
use std::time::Duration;

#[derive(Parser)]
#[clap(
    author  = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about   = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Args {
    /// Set the duration of the animation [milliseconds]
    #[clap(long, default_value_t = 2000)]
    duration: u64,
    /// Set the frames per second
    #[clap(long, default_value_t = 60)]
    fps: u64,
    /// Set the chars used to model the pattern
    #[clap(long, default_value = ".:+#")]
    chars: String,
    /// Set the pattern
    #[clap(long, value_enum)]
    char_pattern: Option<PatternEnum>,
    /// Revert the pattern [possible values: true, false]
    #[clap(long)]
    char_invert: Option<bool>,
    /// Swap the x-axis and y-axis of the pattern
    #[clap(long)]
    char_swap: Option<bool>,
    /// Set the count of segments of the pattern [default: 1-4]
    #[clap(long)]
    char_segments: Option<u8>,
    /// Set the count of slices of the pattern [default: 1-4]
    #[clap(long)]
    char_slices: Option<u8>,
    /// Set the colors used to fill the pattern
    #[clap(long, value_enum)]
    colors: Option<PalletEnum>,
    /// Set the fill pattern
    #[clap(long, value_enum)]
    color_pattern: Option<PatternEnum>,
    /// Choose if the fill pattern should move [possible values: true, false]
    #[clap(long)]
    color_shift: Option<bool>,
    /// Revert the fill pattern
    #[clap(long)]
    color_invert: Option<bool>,
    /// Swap the x-axis and y-axis of the fill pattern
    #[clap(long)]
    color_swap: Option<bool>,
    /// Set the count of slices of the fill pattern [default: 1-4]
    #[clap(long)]
    color_slices: Option<u8>,
}

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

#[derive(ValueEnum, Copy, Clone)]
enum PatternEnum {
    Circle,
    Line,
    Rhombus,
    Wheel,
}

#[derive(derive_more::Constructor)]
struct PatternConfig {
    patterns: Option<PatternEnum>,
    shift: Option<bool>,
    invert: Option<bool>,
    swap: Option<bool>,
    segments: Option<u8>,
    slices: Option<u8>,
}

impl Args {
    fn char_config(&self) -> PatternConfig {
        PatternConfig::new(
            self.char_pattern,
            Some(true),
            self.char_invert,
            self.char_swap,
            self.char_segments,
            self.char_slices,
        )
    }

    fn color_config(&self) -> PatternConfig {
        PatternConfig::new(
            self.color_pattern,
            self.color_shift,
            self.color_invert,
            self.color_swap,
            Some(1),
            self.color_slices,
        )
    }

    fn pallet(&self, rand: &mut impl Rng) -> Vec<Color> {
        match choose(self.colors, rand) {
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
}

impl PatternConfig {
    fn create_base(&self, rand: &mut impl Rng) -> Box<dyn PatternFactory> {
        match choose(self.patterns, rand) {
            PatternEnum::Circle => Box::new(CircleFactory::new()),
            PatternEnum::Line => Box::new(LineFactory::new()),
            PatternEnum::Rhombus => Box::new(RhombusFactory::new()),
            PatternEnum::Wheel => Box::new(WheelFactory::new()),
        }
    }

    fn create(&self, rand: &mut impl Rng) -> Box<dyn PatternFactory> {
        let mut pattern = self.create_base(rand);
        let segments = self.segments.unwrap_or(rand.gen_range(1..=4));
        let slices = self.slices.unwrap_or(rand.gen_range(1..=4));

        if self.shift.unwrap_or(rand.gen()) {
            pattern = Box::new(ShiftFactory::new(pattern))
        }
        if self.invert.unwrap_or(rand.gen()) {
            pattern = Box::new(InvertFactory::new(pattern))
        }
        if self.swap.unwrap_or(rand.gen()) {
            pattern = Box::new(SwapFactory::new(pattern))
        }
        if segments != 1 {
            pattern = Box::new(SegmentsFactory::new(pattern, segments));
        }
        if slices != 1 {
            pattern = Box::new(SliceFactory::new(pattern, slices));
        }
        pattern
    }
}

fn choose<TValue: ValueEnum, TRand: Rng>(opt: Option<TValue>, rand: &mut TRand) -> TValue {
    match opt {
        Some(value) => value.clone(),
        None => TValue::value_variants()
            .iter()
            .choose(rand)
            .unwrap()
            .clone(),
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let rand = &mut thread_rng();

    let char = args.char_config().create(rand);
    let color = args.color_config().create(rand);
    let pallet = args.pallet(rand);

    let sampler = SamplerFactoryImpl::new(char, color);
    let char_converter = CharConverterImpl::new(args.chars);
    let color_converter = ColorConverterImpl::new(pallet);
    let converter = ConverterImpl::new(char_converter, color_converter);
    let term = TerminalImpl::new(stdout());
    let printer = PrinterImpl::new(term)?;
    let renderer = RendererImpl::new(sampler, converter, printer);

    let clock = ClockImpl::new();
    let duration = Duration::from_millis(args.duration);
    let delay = Duration::from_nanos(1_000_000_000 / args.fps);
    let timer = Timer::new(clock, duration, delay);

    timer.run(renderer)
}
