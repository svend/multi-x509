#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use failure::Error;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use structopt::clap::AppSettings;
use structopt::StructOpt;

const BEGIN: &str = "-----BEGIN";
const END: &str = "-----END";

/// Read certificates from stdin and run command on each one.
#[derive(StructOpt, Debug)]
#[structopt(raw(global_settings = "&[AppSettings::TrailingVarArg]"))]
struct Opt {
    /// Text to print after each certificate
    #[structopt(long = "after")]
    after: Option<String>,

    /// Command to run on each certificate
    #[structopt(name = "COMMAND", default_value = "cat")]
    command: Vec<String>,
}

fn run_command(cmd_and_args: &[String], stdin: &str) -> Result<(), Error> {
    let cmd = cmd_and_args
        .get(0)
        .ok_or_else(|| format_err!("command is required"))?;
    let args: Vec<_> = cmd_and_args.iter().skip(1).collect();

    let mut child = Command::new(cmd).args(&args).stdin(Stdio::piped()).spawn()?;

    child.stdin.as_mut().unwrap().write_all(stdin.as_bytes())?;
    child.wait()?;
    Ok(())
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let mut cert = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.starts_with(BEGIN) || !cert.is_empty() {
            cert.push(line.clone());
            if line.starts_with(END) {
                run_command(&opt.command, &format!("{}\n", cert.join("\n")))?;
                if let Some(after) = opt.after.as_ref() {
                    println!("{}", after);
                }
                cert.clear();
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        for cause in err.causes() {
            eprintln!("error: {}", cause);
        }
        std::process::exit(1);
    }
}
