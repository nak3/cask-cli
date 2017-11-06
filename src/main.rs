extern crate clap;
extern crate cask;
extern crate shrust;

use clap::{App, SubCommand, Arg};
use cask::{CaskOptions, SyncStrategy};
use cask::errors::Result;
use std::sync::Arc;

mod operation;

/// Run initiates batch or interactive operations.
fn run() -> Result<()> {
    let app = App::new("cask-cli")
        .version("1.0")
        .about(
            "cask-cli is a command line interface for cask key value store.",
        )
        .arg(
            Arg::with_name("db")
                .help("specify database directory. default \"cask.db\"")
                .long("db")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get the value from specified key.")
                .arg(Arg::with_name("key")),
        )
        .subcommand(
            SubCommand::with_name("put")
                .about("put the value with specified key.")
                .arg(Arg::with_name("key"))
                .arg(Arg::with_name("value")),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete the value with specified key.")
                .arg(Arg::with_name("key")),
        )
        .get_matches();

    let db = app.value_of("db").unwrap_or("cask.db");
    let cask = Arc::new(CaskOptions::default()
        .compaction_check_frequency(1200)
        .sync(SyncStrategy::Interval(5000))
        .max_file_size(1024 * 1024 * 1024)
        .create(true)
        .open(db)?);

    // get
    if let Some(ref matches) = app.subcommand_matches("get") {
        let key = matches.value_of("key").unwrap();
        operation::get(cask.clone(), key.to_string().clone())?;
        return Ok(());
    }

    // put
    if let Some(ref matches) = app.subcommand_matches("put") {
        let key = matches.value_of("key").unwrap();
        let value = matches.value_of("value").unwrap();
        operation::put(
            cask.clone(),
            key.to_string().clone(),
            value.to_string().clone(),
        )?;
        return Ok(());
    }

    // delete
    if let Some(ref matches) = app.subcommand_matches("delete") {
        let key = matches.value_of("key").unwrap();
        if let Err(e) = operation::delete(cask.clone(), key.to_string().clone()) {
            println!("{:?}", e);
        }
        return Ok(());
    }

    // If no arg was specified, start interactive mode
    if let Err(e) = operation::interactive(cask.clone()) {
        println!("{:?}", e);
    }
    Ok(())
}

/// Main
fn main() {
    // TODO: Error
    if let Err(e) = run() {
        println!("{:?}", e);
    }
}
