use magick_rust::{MagickWand, FilterType, ColorspaceType};
use std::cmp::Ordering;
use rustdct::dct2::{DCT2, DCT2SplitRadix};

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

    let pixels: Vec<f32> = wand.export_image_pixels(0, 0, 32, 32, "I")
        .ok_or("Unable to export image pixels")?
        .iter().map(|p| p.clone() as f32).collect();

    let dct = dct_2d(pixels, 32);
    let dct_low_freq: Vec<f32> = dct.chunks(32).take(8) /* first 8 rows */
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

fn dct_2d(m: Vec<f32>, stride: usize) -> Vec<f32> {
    assert!(stride.is_power_of_two());
    let mut dct = DCT2SplitRadix::new(stride);

    let mut rows: Vec<f32> = Vec::with_capacity(stride * stride);

    for row in m.chunks(stride) {
        let mut row_in = row.to_owned();
        let mut row_out = vec![0f32; stride];
        dct.process(&mut row_in, &mut row_out);
        rows.append(&mut row_out);
    };

    transpose(rows.as_mut_slice(), stride);

    let mut columns: Vec<f32> = Vec::with_capacity(stride * stride);

    for column in rows.chunks(stride) {
        let mut column_in = column.to_owned();
        let mut column_out = vec![0f32; stride];
        dct.process(&mut column_in, &mut column_out);
        columns.append(&mut column_out);
    }

    transpose(columns.as_mut_slice(), stride);

    columns
}

fn transpose(m: &mut [f32], stride: usize) {
    for y in 0..stride {
        for x in 0..y {
            m.swap(y * stride + x, x * stride + y);
        }
    }
}
