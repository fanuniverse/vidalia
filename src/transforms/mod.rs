mod downsize_to_width;
mod animation;

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
                animation::gif_to_h264(source, *crf, preset.as_str())
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            },
            &Transform::GifToWebM { ref name, ref crf, ref bitrate } => {
                animation::gif_to_webm(source, *crf, *bitrate)
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            },
            &Transform::GifFirstFrameJpeg { ref name, ref quality } => {
                 animation::gif_first_frame_jpeg(source, *quality)
                    .map(|blob| TransformedImage { name: name.to_owned(), blob: blob })
            }
        }
    }).collect()
}
