use std::env::args;
use std::fs::{self, File};
use std::io;
use std::env::set_current_dir;
use std::path::Path;
use crate::{FOLDERS_TO_CREATE, FILE_TO_CREATE, LOGO};

fn make_dir(path: &str) -> io::Result<()> {
    fs::create_dir(path).inspect(|_| println!("Created folder: {path}"))
}

fn get_folder_name() -> Result<String, &'static str> {
    args().skip(1).next().ok_or_else(|| {
        println!("Usage: mks <folder_name>");
        "Missing folder name"
    })
}

fn about(logo: &str) {
    println!("{logo}");
    println!("\nSkaffolding utility to create a simple structure for htb machines.\n");
}
 
pub fn mks() -> io::Result<()> {
    about(LOGO);

    let Ok(folder_name) = get_folder_name() else {
        return Ok(());
    };

    make_dir(&folder_name)?;

    let new_dir_path = Path::new(&folder_name);
    set_current_dir(&new_dir_path)?;

    let _new_file = File::create(FILE_TO_CREATE)?;

    FOLDERS_TO_CREATE
        .into_iter()
        .map(make_dir)
        .collect::<io::Result<_>>()
}