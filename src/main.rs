use clap::Parser;
use rand::rngs::OsRng;
use crate::char::CharSampler;
use crate::pallet::{choose_pallet, create_pallet, PalletEnum};

mod pallet;
mod char;

#[derive(Parser)]
#[clap(author = "Rico Riedel", version = "0.1.0", about = "Wipe your terminal with a random animation.")]
struct Args {
    #[clap(short, long, help = "Add color pallet", arg_enum)]
    pallet: Vec<PalletEnum>,
    #[clap(long, default_value = ".-+%#", help = "Set chars")]
    chars: String
}

fn main() {
    let args = Args::parse();
    let rng = &mut OsRng::default();

    let chars = CharSampler::new(args.chars);

    let pallet_key = choose_pallet(args.pallet, rng);
    let pallet = create_pallet(pallet_key);
}
