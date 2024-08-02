use clap::{Parser, Subcommand};
use core::str;
use std::process::Command;

mod config;
use config::{load_config, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Listen,
    Build,
    Run { test_number: Option<i32> },
}

fn main() {
    let args = Cli::parse();

    // Load or create config
    let user_cfg: Config = load_config();

    match &args.cmd {
        Cmd::Listen => {}
        Cmd::Build => build(&user_cfg),
        Cmd::Run { test_number } => match test_number {
            Some(number) => println!("Running test #{}", number),
            None => println!("Running all tests"),
        },
    }
}

fn build(cfg: &Config) {
    let build_cmd = Command::new(&cfg.compile_command)
        .args(&cfg.compile_args)
        .output()
        .unwrap();

    let stdout = str::from_utf8(&build_cmd.stdout).unwrap();
    println!("{}", stdout);
}
