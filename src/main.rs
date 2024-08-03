use clap::{Parser, Subcommand};
use core::str;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

mod config;
mod server;
use config::{load_config, Config};
use server::listen;

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

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // Load or create config
    let user_cfg: Config = load_config();

    match &args.cmd {
        Cmd::Listen => listen(&user_cfg),
        Cmd::Build => build(&user_cfg),
        Cmd::Run { test_number } => {
            match test_number {
                Some(number) => run_test(number, &user_cfg)?,
                None => {
                    let mut current_test = 1;
                    loop {
                        let file_name = format!("input{}", current_test);
                        let file_path = Path::new(&file_name);

                        if file_path.exists() {
                            run_test(&current_test, &user_cfg)?;
                            current_test += 1;
                        } else {
                            break;
                        }
                    }
                }
            };
            Ok(())
        }
    }
}

fn run_test(num: &i32, cfg: &Config) -> std::io::Result<()> {
    // TODO: check tle, mle + compare against output
    println!("Running test {}", num);
    let test_file = File::open(format!("input{}", num)).expect("Test case doesn't exist");

    let run_cmd = Command::new(&cfg.run_command)
        .stdin(Stdio::from(test_file))
        .output()
        .unwrap();

    let stdout = str::from_utf8(&run_cmd.stdout).unwrap();
    let stderr = str::from_utf8(&run_cmd.stderr).unwrap();

    println!("{}\n{}", stdout, stderr);

    Ok(())
}

fn build(cfg: &Config) -> std::io::Result<()> {
    let build_cmd = Command::new(&cfg.compile_command)
        .args(&cfg.compile_args)
        .output()
        .unwrap();

    let stdout = str::from_utf8(&build_cmd.stdout).unwrap();
    let stderr = str::from_utf8(&build_cmd.stderr).unwrap();

    println!("{}\n{}", stdout, stderr);

    Ok(())
}
