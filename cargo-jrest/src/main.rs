use std::env;
use std::process::Stdio;

use clap::Parser;
use serde_json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    testname: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filtered_args = if is_run_as_cargo_command() {
        args.iter().skip(1).cloned().collect::<Vec<String>>()
    } else {
        args.iter().cloned().collect::<Vec<String>>()
    };

    let cli = Cli::parse_from(filtered_args);

    let controlled_testname = cli.testname.as_deref().unwrap_or(".").to_owned();
    let statement = format!("cargo test {}", controlled_testname);

    clear_terminal();

    println!("\n[Jrest] Running `{}`...\n", statement);

    let mut tokio_command = Command::new("cargo");

    tokio_command.args([
        "test",
        "--message-format=json",
        controlled_testname.as_str(),
    ]);
    tokio_command.stdout(Stdio::piped());

    let mut child = tokio_command.spawn().expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect(format!("Failed to run `cargo test {}`.", controlled_testname).as_str());

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("{") && line.ends_with("}") {
            let _json = serde_json::from_str::<serde_json::Value>(&line).unwrap();
            // let testname = json["name"].as_str().unwrap();
            // let teststatus = json["event"]["test"]["status"].as_str().unwrap();

            continue;
        }

        println!("Line: {}", line);
    }

    Ok(())
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn is_run_as_cargo_command() -> bool {
    let args: Vec<String> = std::env::args().collect();

    args.len() >= 2 && args[1] == "jrest"
}
