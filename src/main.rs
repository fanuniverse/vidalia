#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate magick_rust;

use magick_rust::{MagickWand, magick_wand_genesis, magick_wand_terminus};

fn main() {
    magick_wand_genesis();

    process().expect("processing failed");

    magick_wand_terminus();
}

fn process() -> Result<(), &'static str> {
    let wand = MagickWand::new();
    wand.read_image("small.jpg")?;
    wand.fit(100, 100);
    wand.write_image("out.jpg")
}
