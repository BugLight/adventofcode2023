use anyhow::Result;
use clap::Parser;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

/// Run advent of code program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day of the challenge
    #[arg(short, long)]
    day: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        _ => Err(anyhow::Error::msg("No such day")),
    }
}
