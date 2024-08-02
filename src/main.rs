use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Listen,
    Build,
    Run { test_number: Option<i32> },
}

fn main() {
    let args = Cli::parse();

    match &args.cmd {
        Command::Listen => {}
        Command::Build => {}
        Command::Run { test_number } => match test_number {
            Some(number) => println!("Running test #{}", number),
            None => println!("Running all tests"),
        },
    }
}
