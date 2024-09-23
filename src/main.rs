use mks::{load_sentry, mks};
use std::env::args;

fn main() {
    load_sentry();

    let argument = args().nth(1).ok_or_else(|| {
        println!("Usage: mks <folder_name>");
        "Missing Folder Name"
    });

    mks(argument);
}
