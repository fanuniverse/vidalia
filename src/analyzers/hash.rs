use magick_rust::{MagickWand, FilterType, ColorspaceType};
use stream_dct::dct_2d;
use std::cmp::Ordering;

/* Algorithm outline:
 * http://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
 * Reference implementations:
 * https://github.com/JohannesBuchner/imagehash
 * https://github.com/abonander/img_hash */
pub fn perceptual_hash(blob: &Vec<u8>) -> Result<u64, &'static str> {
    let mut wand = MagickWand::new();
    wand.read_image_blob(blob)?;

    /* Hash the first frame of an animated image */
    if wand.get_image_format()? == "GIF" {
        wand.set_iterator_index(0)?;
    }

    wand.transform_image_colorspace(ColorspaceType::GRAYColorspace);
    wand.resize_image(32, 32, FilterType::LanczosFilter);

    let pixels: Vec<f64> = wand.export_image_pixels(0, 0, 32, 32, "I")
        .ok_or("Unable to export image pixels")?
        .iter().map(|p| p.clone() as f64).collect();

    let dct = dct_2d(&pixels[..], 32);
    let dct_low_freq: Vec<f64> = dct.chunks(32).take(8) /* first 8 rows */
        .flat_map(|row| row.iter().take(8)) /* first 8 columns */
        .cloned().collect();

    let median = {
        let mut low_freq_sorted = dct_low_freq.clone();
        /* https://news.ycombinator.com/item?id=9089112 */
        low_freq_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));

        (low_freq_sorted[(8 * 8 / 2) - 1] + low_freq_sorted[8 * 8 / 2]) / 2.0
    };

    let mut hash = 0u64;
    let mut increment = 1u64;

    for freq in dct_low_freq {
        if freq > median { hash = hash | increment; }
        increment = increment << 1;
    }

    Ok(hash)
}
