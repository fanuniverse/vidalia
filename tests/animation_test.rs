extern crate reqwest;
extern crate multipart;
extern crate rand;

use std::io::{Read, Write};
use std::fs::{File, remove_file};
use std::process::Command;
use rand::{Rng, thread_rng};

use multipart::server::Multipart as MultipartInbound;

#[macro_use]
#[path = "utils.rs"]
mod utils;
use utils::{setup_client, MultipartResponse};

#[test]
fn it_generates_h264() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind": "gif_to_h264"
                , "name": "h264"
                , "crf":  22
                , "preset": "fast" }
            ]
        }
        "#, "dimensions.gif");

    let mut image_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "h264" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();
    
    let tmp_file: String = thread_rng().gen_ascii_chars().take(16).collect();
    let tmp_path = format!("/tmp/test-{}.mp4", tmp_file);

    File::create(&tmp_path).unwrap().write_all(&image_buf).unwrap();

    let probe = Command::new("ffprobe")
        .args(&[&tmp_path, "-v",              "error",
                           "-select_streams", "v:0",
                           "-show_entries",   "stream=codec_name,width,height",
                           "-of",             "flat=h=0"])
        .output().unwrap()
        .stdout;
    
    remove_file(&tmp_path).unwrap();

    assert_eq!(String::from_utf8_lossy(&probe), r#"stream.0.codec_name="h264"
stream.0.width=604
stream.0.height=340
"#);
}

#[test]
fn it_generates_webm() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind":    "gif_to_webm"
                , "name":    "webm"
                , "crf":     22
                , "bitrate": 1200 }
            ]
        }
        "#, "dimensions.gif");

    let mut image_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "webm" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();

    let tmp_file: String = thread_rng().gen_ascii_chars().take(16).collect();
    let tmp_path = format!("/tmp/test-{}.mp4", tmp_file);

    File::create(&tmp_path).unwrap().write_all(&image_buf).unwrap();

    let probe = Command::new("ffprobe")
        .args(&[&tmp_path, "-v",              "error",
                           "-select_streams", "v:0",
                           "-show_entries",   "stream=codec_name,width,height",
                           "-of",             "flat=h=0"])
        .output().unwrap()
        .stdout;

    remove_file(&tmp_path).unwrap();

    assert_eq!(String::from_utf8_lossy(&probe), r#"stream.0.codec_name="vp9"
stream.0.width=604
stream.0.height=340
"#);
}

#[test]
fn it_extracts_the_first_frame_of_a_gif() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind":    "gif_first_frame_jpeg"
                , "name":    "poster"
                , "quality": 80 }
            ]
        }
        "#, "dimensions.gif");

    let mut expected_buf = Vec::new();
    File::open(fixture_path!("dimensions_first_frame.jpg"))
        .and_then(|mut f| f.read_to_end(&mut expected_buf)).unwrap();

    let mut actual_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "poster" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut actual_buf).unwrap();
            }
        }).unwrap();

    assert!(actual_buf == expected_buf);
}
