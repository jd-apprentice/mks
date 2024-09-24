use mks::{load_sentry, mks};
use std::env::args;

fn main() {
    load_sentry();

    let folder_name = args().nth(1).ok_or_else(|| {
        log::info!("Usage: mks <folder_name>");
        "Missing Folder Name"
    });

    let folder_type = args().nth(2).ok_or_else(|| {
        log::info!("Usage: mks <folder_name> --<type>");
        "Missing Folder Type"
    });

    mks(folder_name, folder_type);
}
