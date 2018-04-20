#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use failure::Error;
use std::io;
use std::io::prelude::*;
use std::process::{self, Command, Stdio};
use structopt::StructOpt;
use structopt::clap::AppSettings;

const BEGIN: &str = "-----BEGIN";
const END: &str = "-----END";

/// Read certificates from stdin and run command on each one.
#[derive(StructOpt, Debug)]
#[structopt(author = "Svend Sorensen", raw(global_settings = "&[AppSettings::TrailingVarArg]"))]
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

fn run(command: &[String], after: &Option<String>) -> Result<(), Error> {
    let stdin = io::stdin();
    let mut cert = Vec::new();

    for line in stdin.lock().lines() {
        let l = line?;
        if l.starts_with(BEGIN) || !cert.is_empty() {
            cert.push(l.clone());
            if l.starts_with(END) {
                run_command(&command, &format!("{}\n", cert.join("\n")))?;
                if let Some(after) = after.as_ref() {
                    println!("{}", after);
                }
                cert.clear();
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = run(&opt.command, &opt.after) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
