mod day1;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Marcus Ingelborn", version, about)]
struct Args {
    #[arg(short, long)]
    day: i32,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    match day {
        1 => day1::main(),
        _ => println!("No such day: {}", day),
    }
}
