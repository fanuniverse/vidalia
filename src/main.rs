extern crate iron;
extern crate multipart;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate magick_rust;

mod server;

use magick_rust::{magick_wand_genesis, magick_wand_terminus};

mod types;
use types::{Manifest, ProcessingResult};

fn main() {
    server::run(do_stuff);

    magick_wand_genesis();

    magick_wand_terminus();
}

fn do_stuff(_manifest: Manifest, blob: Vec<u8>) -> ProcessingResult {
    ProcessingResult { image: blob }
}
