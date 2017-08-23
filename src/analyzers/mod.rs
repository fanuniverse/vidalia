use types::{Manifest, AnalyzedImage};
use magick_rust::{MagickWand};

pub fn run(manifest: &Manifest, source: &Vec<u8>) -> Result<AnalyzedImage, &'static str> {
    let mut result = AnalyzedImage::default();

    let wand = MagickWand::new();
    wand.read_image_blob(source)?;

    let format = wand.get_image_format()?;
    let (width, height) = get_dimensions(wand, format);

    for a in &manifest.analyzers { match a.as_str() {
        "width" => { result.width = Some(width) }
        "height" => { result.height = Some(height) }
        &_ => { /* TODO: return an error */ }
    }}

    Ok(result)
}

fn get_dimensions(wand: MagickWand, format: String) -> (usize, usize) {
    match format.as_str() {
        "GIF" => {
            let (width, height, _, _) = wand.get_image_page();
            (width, height)
        }
        _ => {
            (wand.get_image_width(), wand.get_image_height())
        }
    }
}
