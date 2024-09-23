use std::env::args;
use mks::{mks, load_sentry};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    load_sentry();

    let argument = args().nth(1).ok_or_else(||{
        println!("Usage: mks <folder_name>");
        "Missing Folder Name"
    });

    mks(argument)?;
    
    Ok(())
}


