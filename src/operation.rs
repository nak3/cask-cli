use shrust::{Shell, ShellIO};
use cask::errors::Result;
use std::str;
use cask::Cask;
use std::sync::Arc;

/// get gets the value from specified key.
pub fn get(cask: Arc<Cask>, key: String) -> Result<()> {
    let v = cask.get(key)?;
    println!("{}", str::from_utf8(&v.unwrap()).unwrap());
    Ok(())
}

/// put puts the value with specified key.
pub fn put(cask: Arc<Cask>, key: String, value: String) -> Result<()> {
    cask.put(key, value)?;
    Ok(())
}

/// delete deletes the value with specified key.
pub fn delete(cask: Arc<Cask>, key: String) -> Result<()> {
    cask.delete(key)?;
    Ok(())
}

/// interactive starts interactive shell for cask.
pub fn interactive(cask: Arc<Cask>) -> Result<()> {
    let mut shell = Shell::new((cask.clone()));

    //  delete
    shell.new_command("delete", "delete the value with specified key", 1, |_,
     cask,
     key| {
        delete(cask.clone(), key[0].to_string())?;
        Ok(())
    });

    // get
    shell.new_command("get", "List strings", 1, move |_, cask, key| {
        get(cask.clone(), key[0].to_string())?;
        Ok(())
    });

    // put
    shell.new_command("put", "List strings", 2, move |_, cask, key| {
        put(cask.clone(), key[0].to_string(), key[1].to_string())?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
    Ok(())
}
