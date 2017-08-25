mod hash;

use types::{AnalyzedImage};
use magick_rust::{MagickWand};

pub fn run(source: &Vec<u8>) -> Result<AnalyzedImage, &'static str> {
    let wand = MagickWand::new();
    wand.read_image_blob(source)?;

    let format = wand.get_image_format()?;
    let (width, height) = get_dimensions(wand, &format);

    let hash_image = hash::MagickHashImage::new(source, &format, width, height)?;
    let _ = hash_image.dct_hash();

    Ok(AnalyzedImage {
        width: width,
        height: height
    })
}

fn get_dimensions(wand: MagickWand, format: &str) -> (usize, usize) {
    match format {
        "GIF" => {
            let (width, height, _, _) = wand.get_image_page();
            (width, height)
        }
        _ => {
            (wand.get_image_width(), wand.get_image_height())
        }
    }
}
