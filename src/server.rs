use serde::{Deserialize, Serialize};
use serde_json;

use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::path::PathBuf;

use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProblemData {
    pub name: String,
    pub group: String,
    pub tests: Vec<TestCase>,
}

pub fn listen(cfg: &Config) -> std::io::Result<()> {
    let address = format!("localhost:{}", cfg.port);

    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = Vec::new();

        stream.read_to_end(&mut buffer)?;
        let request = String::from_utf8_lossy(&buffer);

        if request.starts_with("POST / HTTP/1.1") {
            if let Some(body) = request.split("\r\n\r\n").nth(1) {
                if let Ok(received_problem) = serde_json::from_str::<ProblemData>(&body) {
                    match save_problem(&received_problem, &cfg) {
                        Ok(_) => println!("Saved {}", received_problem.name),
                        Err(_) => println!("Error saving {}", received_problem.name),
                    }
                }
            }
        }
    }

    Ok(())
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '(' || *c == ')')
        .collect()
}

fn save_problem(problem: &ProblemData, cfg: &Config) -> std::io::Result<()> {
    let mut problem_path = PathBuf::from(&cfg.contest_directory);

    let group_parts = problem.group.split('-');

    for part in group_parts {
        problem_path = problem_path.join(sanitize_filename(part))
    }

    let problem_name = sanitize_filename(&problem.name.split('-').last().unwrap());
    problem_path = problem_path.join(problem_name);

    fs::create_dir_all(&problem_path)?;

    let solution_file = problem_path.join("solution.cpp");
    write_file(&solution_file, &cfg.cpp_template)?;

    for (num, case) in problem.tests.iter().enumerate() {
        let input_file = problem_path.join(format!("input{}", num + 1));
        let output_file = problem_path.join(format!("output{}", num + 1));
        write_file(&input_file, &case.input)?;
        write_file(&output_file, &case.output)?;
    }

    println!("{}", problem_path.display());
    Ok(())
}

fn write_file(path: &PathBuf, contents: &str) -> std::io::Result<()> {
    if !path.exists() {
        fs::write(path, contents)?;
    }
    Ok(())
}
