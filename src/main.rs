extern crate iron;
extern crate multipart;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate magick_rust;

pub mod types;
mod server;
mod transforms;

use types::{Manifest, ProcessingResult};

fn main() {
    transforms::init();

    server::run(do_stuff);
}

fn do_stuff(manifest: Manifest, blob: Vec<u8>) -> Result<ProcessingResult, &'static str> {
    let images = transforms::run(&manifest, &blob)?;

    Ok(ProcessingResult { images: images })
}
