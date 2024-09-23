use std::env::{self, set_current_dir};
use std::fs::{self, File};
use std::io;
use std::path::Path;

pub fn load_sentry() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let sentry_dns = env::var("SENTRY_DSN").unwrap();

    let _guard = sentry::init((
        sentry_dns,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}

pub fn make_dir(path: &str) -> io::Result<()> {
    fs::create_dir(path).inspect(|_| println!("Created folder: {path}"))
}

fn about(logo: &str) {
    println!("{logo}");
    println!("\nSkaffolding utility to create folder structures for different purposes.\n");
}

pub fn mks(folder_name: Result<String, &'static str>) {
    about(crate::LOGO);

    let folder_name = match folder_name {
        Ok(folder_name) => folder_name,
        Err(_) => {
            println!("Usage: mks <folder_name>");
            return;
        }
    };

    let _ = make_dir(&folder_name);

    let new_dir_path = Path::new(&folder_name);
    let _ = set_current_dir(new_dir_path);

    let _new_file = File::create(crate::FILE_TO_CREATE);

    let _ = crate::FOLDERS_TO_CREATE
        .into_iter()
        .try_for_each(make_dir);
}
