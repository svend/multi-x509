mod certs;

use failure::{format_err, Error};
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use structopt::clap::AppSettings;
use structopt::StructOpt;

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

    let mut child = Command::new(cmd)
        .args(&args)
        .stdin(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(stdin.as_bytes())?;
    child.wait()?;
    Ok(())
}

fn run<R: BufRead>(r: R, command: &[String], after: &Option<String>) -> Result<(), Error> {
    let certs = certs::Certs::new(r);
    for cert in certs {
        run_command(&command, &format!("{}\n", &cert))?;
        if let Some(after) = after {
            println!("{}", after);
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    if let Err(err) = run(stdin, &opt.command, &opt.after) {
        for cause in err.iter_chain() {
            eprintln!("error: {}", cause);
        }
        std::process::exit(1);
    }
}
