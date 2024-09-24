use crate::{FolderName, FolderType};
use log::info;
use std::env::{self, set_current_dir};
use std::fs::{create_dir_all, File};
use std::io;
use std::path::Path;

/// # Panics
/// Will panic if `SENTRY_DSN` is not set in the .env file
#[inline]
pub fn load_sentry() {
    dotenvy::dotenv().unwrap_or_else(|_| {
        panic!("Missing .env file");
    });

    let sentry_dns = env::var("SENTRY_DSN").unwrap_or_else(|_| {
        panic!("Missing SENTRY_DSN environment variable");
    });

    let _guard = sentry::init((
        sentry_dns,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}

/// # Errors
/// Will return an error if the folder cannot be created
#[inline]
pub fn make_dir(path: &str) -> io::Result<()> {
    create_dir_all(path).inspect(|()| info!("Created folder: {path}"))
}

/// # Panics
/// Will panic if `folder_name` or `folder_type` is not set
#[inline]
pub fn mks(folder_name: FolderName, folder_type: FolderType) {
    info!("{}", crate::LOGO);
    info!("\nSkaffolding utility to create folder structures for different purposes.\n");

    let Ok(f_name) = folder_name else {
        info!("{}", crate::USAGE_MESSAGE);
        panic!("Missing Folder Name");
    };

    _ = make_dir(&f_name);

    let new_dir_path = Path::new(&f_name);
    _ = set_current_dir(new_dir_path);

    let _new_file = File::create(crate::FILE_TO_CREATE);

    let Ok(f_type) = folder_type else {
        info!("{}", crate::USAGE_MESSAGE);
        panic!("Missing Folder Type");
    };

    match f_type.as_str() {
        "--hacking" => {
            _ = crate::FOLDERS.hacking.into_iter().try_for_each(make_dir);
        }
        "--study" => {
            _ = crate::FOLDERS.study.into_iter().try_for_each(make_dir);
        }
        _ => {
            info!("{}", crate::USAGE_MESSAGE);
        }
    }
}
