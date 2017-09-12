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
    let client = setup_client();

    let response = client.post(vidalia_url!()).unwrap()
        .multipart(reqwest::multipart::Form::new()
            .text("manifest", r#"
            {
                "transforms": [
                    { "kind": "gif_to_h264"
                    , "name": "h264"
                    , "crf":  22
                    , "preset": "fast" }
                ]
            }
            "#)
            .file("image", fixture_path!("dimensions.gif"))
            .unwrap()
        )
        .send().unwrap();

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
