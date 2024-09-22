use dotenv::dotenv;
use std::env::{self};
use std::io;
use mks::mks;

fn main() -> io::Result<()> {

    dotenv().ok();

    let sentry_dns = env::var("SENTRY_DSN").unwrap();

    let _guard = sentry::init((sentry_dns, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    mks()
    
}


