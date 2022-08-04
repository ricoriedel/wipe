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
    #[clap(long, default_value_t = 1000)]
    duration: u64,
    #[clap(long, default_value_t = 60)]
    fps: u64,
    #[clap(long, default_value = ".:+#")]
    chars: String,
    #[clap(long, value_enum)]
    char_pattern: Vec<PatternEnum>,
    #[clap(long)]
    char_invert: Option<bool>,
    #[clap(long, value_enum)]
    colors: Vec<PalletEnum>,
    #[clap(long, value_enum)]
    color_pattern: Vec<PatternEnum>,
    #[clap(long)]
    color_shift: Option<bool>,
    #[clap(long)]
    color_invert: Option<bool>,
}

#[derive(ValueEnum, Clone)]
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

#[derive(ValueEnum, Clone)]
enum PatternEnum {
    Circle,
    Line,
    Rhombus,
    Wheel,
}

#[derive(derive_more::Constructor)]
struct PatternConfig<'a> {
    patterns: &'a Vec<PatternEnum>,
    shift: Option<bool>,
    invert: Option<bool>,
}

impl Args {
    fn char_config(&self) -> PatternConfig {
        PatternConfig::new(&self.char_pattern, Some(true), self.char_invert)
    }

    fn color_config(&self) -> PatternConfig {
        PatternConfig::new(&self.color_pattern, self.color_shift, self.color_invert)
    }

    fn pallet(&self, rand: &mut impl Rng) -> Vec<Color> {
        match choose(&self.colors, rand) {
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

impl<'a> PatternConfig<'a> {
    fn create_base(&self, rand: &mut impl Rng) -> Box<dyn PatternFactory> {
        match choose(self.patterns, rand) {
            PatternEnum::Circle => Box::new(CircleFactory::default()),
            PatternEnum::Line => Box::new(LineFactory::default()),
            PatternEnum::Rhombus => Box::new(RhombusFactory::default()),
            PatternEnum::Wheel => Box::new(WheelFactory::default()),
        }
    }

    fn create(&self, rand: &mut impl Rng) -> Box<dyn PatternFactory> {
        let mut pattern = self.create_base(rand);

        if self.shift.unwrap_or(rand.gen()) {
            pattern = Box::new(ShiftFactory::new(pattern))
        }
        if self.invert.unwrap_or(rand.gen()) {
            pattern = Box::new(InvertFactory::new(pattern))
        }
        pattern
    }
}

fn choose<TValue: ValueEnum + Clone, TRand: Rng>(
    options: &Vec<TValue>,
    rand: &mut TRand,
) -> TValue {
    if options.is_empty() {
        TValue::value_variants()
            .iter()
            .choose(rand)
            .unwrap()
            .clone()
    } else {
        options.iter().choose(rand).unwrap().clone()
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

    let clock = ClockImpl::default();
    let duration = Duration::from_millis(args.duration);
    let delay = Duration::from_nanos(1_000_000_000 / args.fps);
    let timer = Timer::new(clock, duration, delay);

    timer.run(renderer)
}
