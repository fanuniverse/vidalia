extern crate iron;
extern crate multipart;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate magick_rust;
extern crate rustdct;
extern crate rand;

pub mod types;
mod server;
mod analyzers;
mod transforms;

use types::{Manifest, ProcessingResult};

pub fn run() {
    transforms::init();

    server::run(process);
}

fn process(manifest: Manifest, blob: Vec<u8>) -> Result<ProcessingResult, &'static str> {
    Ok(ProcessingResult {
        analyzed: try!(analyzers::run(&blob)),
        transformed: try!(transforms::run(&manifest, &blob))
    })
}
