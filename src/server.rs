use serde::{Deserialize, Serialize};
use serde_json;

use std::io::Read;
use std::net::TcpListener;

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

pub fn start_listening(cfg: &Config) -> std::io::Result<()> {
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
                    println!("Saved {} ", received_problem.name);
                }
            }
        }
    }

    Ok(())
}
