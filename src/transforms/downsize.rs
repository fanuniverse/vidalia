use magick_rust::{MagickWand};

pub fn transform(source: &Vec<u8>, width: usize) -> Result<Vec<u8>, &'static str> {
    let wand = MagickWand::new();

    wand.read_image_blob(source)?;
    wand.fit(width, width);
    wand.write_image_blob("JPEG") /* FIXME IMPORTANT: infer the format? */
}
