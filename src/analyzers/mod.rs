use types::{Manifest, AnalyzedImage};
use magick_rust::{MagickWand};

pub fn run(manifest: &Manifest, source: &Vec<u8>) -> Result<AnalyzedImage, &'static str> {
    let mut result = AnalyzedImage::default();

    let wand = MagickWand::new();
    wand.read_image_blob(source)?;

    for a in &manifest.analyzers { match a.as_str() {
        "width" => { result.width = Some(wand.get_image_width()) }
        &_ => { /* TODO: return an error */ }
    }}

    Ok(result)
}
