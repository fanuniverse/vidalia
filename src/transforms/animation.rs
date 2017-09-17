use std::process::Command;
use std::io::{Read, Write};
use std::fs::{File, remove_file};
use rand::{Rng, thread_rng};
use magick_rust::MagickWand;

pub fn gif_to_h264(source: &Vec<u8>, crf: u8, preset: &str) -> Result<Vec<u8>, &'static str> {
    run_ffmpeg(source, "gif", "mp4",
               &["-crf",      &crf.to_string(),
                 "-pix_fmt",  "yuv420p",
                 "-c:v",      "libx264",
                 "-preset",   preset,
                 "-movflags", "+faststart",
                 "-vf",       "scale=trunc(iw/2)*2:trunc(ih/2)*2"])
}

/* Performs constrained quality conversion, https://trac.ffmpeg.org/wiki/Encode/VP9#constrainedq */
pub fn gif_to_webm(source: &Vec<u8>, crf: u8, bitrate: u16) -> Result<Vec<u8>, &'static str> {
    run_ffmpeg(source, "gif", "webm",
               &["-crf",           &crf.to_string(),
                "-pix_fmt",        "yuv420p",
                "-c:v",            "libvpx-vp9",
                "-b:v",            &format!("{}K", bitrate),
                "-tile-columns",   "6",
                "-frame-parallel", "1"])
}

pub fn gif_first_frame_jpeg(source: &Vec<u8>, quality: u8) -> Result<Vec<u8>, &'static str> {
    let mut wand = MagickWand::new();
    wand.read_image_blob(source)?;

    wand.set_iterator_index(0)?;
    wand.set_image_compression_quality(quality as usize)?;

    wand.write_image_blob("JPEG")
}

fn run_ffmpeg(source: &Vec<u8>, in_ext: &str, out_ext: &str, args: &[&str]) -> Result<Vec<u8>, &'static str> {
    let in_path = format!("/tmp/in-{}.{}", random_file_name(), in_ext);
    let out_path = format!("/tmp/out-{}.{}", random_file_name(), out_ext);

    File::create(&in_path).or(Err("Unable to create input file"))?
        .write_all(source).or(Err("Unable to write input data"))?;

    let full_args =
        [&["-f", &in_ext,
           "-i", &in_path], args, &[&out_path]].concat();
    let result =
        Command::new("ffmpeg").args(full_args)
            .output().or(Err("Unable to spawn ffmpeg process"))
            .and_then(|cmd| {
                if cmd.status.success() {
                    let mut result = vec![];
                    match File::open(&out_path).and_then(|mut f| f.read_to_end(&mut result)) {
                        Ok(_) => {
                            Ok(result)
                        }
                        _ => {
                            Err("Unable to read ffmpeg output")
                        }
                    }
                }
                else {
                    Err("ffmpeg has exited with a non-zero status code")
                }
            });

    let _ = remove_file(&in_path);
    let _ = remove_file(&out_path);

    result
}

fn random_file_name() -> String {
    thread_rng().gen_ascii_chars().take(16).collect()
}
