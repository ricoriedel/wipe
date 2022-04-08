use clap::Parser;

mod color;
mod char;
mod fill;
mod vec;
mod array;
mod surface;
mod animation;
mod sampler;

#[derive(Parser)]
#[clap(author = "Rico Riedel", version = "0.1.0", about = "Wipe your terminal with a random animation.")]
struct Args {
    #[clap(long, default_value = ".-+%#", help = "Set chars")]
    chars: String
}

fn main() {
    Args::parse();
}
