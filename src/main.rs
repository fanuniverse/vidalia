extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate magick_rust;

use magick_rust::{MagickWand, magick_wand_genesis, magick_wand_terminus};

mod types;
use types::{Manifest, Transform};

fn main() {
    magick_wand_genesis();

    process(get_manifest().unwrap()).expect("oops");

    magick_wand_terminus();
}

fn get_manifest() -> Result<Manifest, serde_json::Error> {
    let data = r#"{
                    "file": "small.jpg",
                    "transforms": [
                      { "kind": "downsize", "width": 100 }
                    ]
                  }"#;

    serde_json::from_str(data)
}

fn process(manifest: Manifest) -> Result<(), &'static str> {
    for t in manifest.transforms {
        match t {
            Transform::Downsize { width } => {
                let wand = MagickWand::new();
                wand.read_image(&manifest.file)?;
                wand.fit(width, width);
                wand.write_image("out.jpg")?;
            }
            _ => {}
        }
    }

    Ok(())
}
