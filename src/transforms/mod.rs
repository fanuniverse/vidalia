mod downsize;

use types::{Manifest, Transform, TransformedImage};
use magick_rust;

pub fn init() {
    magick_rust::magick_wand_genesis();
}

pub fn run(manifest: &Manifest, source: &Vec<u8>) -> Result<Vec<TransformedImage>, &'static str> {
    manifest.transforms.iter().map(|t| {
        match t {
            &Transform::Downsize { ref name, ref width } => {
                downsize::transform(source, *width)
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            },
            _ => {
                Ok(TransformedImage { name: "unknown".to_string(), blob: vec![] })
            }
        }
    }).collect()
}
