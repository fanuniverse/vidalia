extern crate iron;
extern crate multipart;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate magick_rust;

pub mod types;
mod server;
mod transforms;
mod analyzers;

use types::{Manifest, ProcessingResult};

pub fn run() {
    transforms::init();

    server::run(process);
}

fn process(manifest: Manifest, blob: Vec<u8>) -> Result<ProcessingResult, &'static str> {
    Ok(ProcessingResult {
        transformed: try!(transforms::run(&manifest, &blob)),
        analyzed: try!(analyzers::run(&manifest, &blob))
    })
}
