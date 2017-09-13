use magick_rust::{MagickWand, FilterType};

pub fn transform(source: &Vec<u8>, target_width: usize) -> Result<Vec<u8>, &'static str> {
    let wand = MagickWand::new();
    wand.read_image_blob(source)?;

    let aspect_ratio = wand.get_image_width() as f32 / wand.get_image_height() as f32;
    let target_height = (target_width as f32 / aspect_ratio).floor() as usize;

    wand.resize_image(target_width, target_height, FilterType::LanczosFilter);

    let format = wand.get_image_format()?;

    wand.write_image_blob(format.as_str())
}
