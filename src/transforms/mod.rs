mod downsize_to_width;
mod ffmpeg;

use types::{Manifest, Transform, TransformedImage};
use magick_rust;

pub fn init() {
    magick_rust::magick_wand_genesis();
}

pub fn run(manifest: &Manifest, source: &Vec<u8>) -> Result<Vec<TransformedImage>, &'static str> {
    manifest.transforms.iter().map(|t| {
        match t {
            &Transform::DownsizeToWidth { ref name, ref width } => {
                downsize_to_width::transform(source, *width)
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            },
            &Transform::GifToH264 { ref name, ref crf, ref preset } => {
                ffmpeg::gif_to_h264(source, *crf, preset.as_str())
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            },
            &Transform::GifToWebM { ref name, ref crf, ref bitrate } => {
                ffmpeg::gif_to_webm(source, *crf, *bitrate)
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            }
        }
    }).collect()
}
