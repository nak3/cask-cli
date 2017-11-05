use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use cask::errors::Result;
use std::str;
use cask::Cask;
use std::sync::Arc;

/// get returns ...
pub fn get(cask: Arc<Cask>, key: String) -> Result<()> {
    let v = cask.get(key)?;
    println!("value:{}", str::from_utf8(&v.unwrap()).unwrap());
    Ok(())
}

/// put returns...
pub fn put(cask: Arc<Cask>, key: String, value: String) -> Result<()> {
    cask.put(key, value)?;
    Ok(())
}

/// delete returns...
pub fn delete(cask: Arc<Cask>, key: String) -> Result<()> {
    cask.delete(key)?;
    Ok(())
}

/// interactive starts interactive shell for cask.
///
// TODO: argment
pub fn interactive(cask: Arc<Cask>) -> Result<()> {
    let mut shell = Shell::new(());

    //  create
    shell.new_command_noargs(
        "delete",
        "delete the value with specified key",
        |io, _, key| {
            //        delete(cask, key
            try!(writeln!(io, "Hello World !!!"));
            Ok(())
        },
    );

    // get
    shell.new_command("get", "List strings", 1, move |io, _, key| {
        let v = cask.get(key[0].to_string())?;
        try!(writeln!(io, "{:?}", v));
        Ok(())
    });
    shell.run_loop(&mut ShellIO::default());
    Ok(())
}
