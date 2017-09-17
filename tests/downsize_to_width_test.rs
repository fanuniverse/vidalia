extern crate reqwest;
extern crate multipart;
extern crate magick_rust;

use std::io::Read;

use multipart::server::Multipart as MultipartInbound;

use magick_rust::MagickWand;

#[macro_use]
#[path = "utils.rs"]
mod utils;
use utils::{setup_client, MultipartResponse};

#[test]
fn it_downsizes_images_larger_than_target() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind": "downsize_to_width"
                , "name": "thumbnail"
                , "width": 50 }
            ]
        }
        "#, "small.jpg");

    let mut image_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "thumbnail" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();

    let wand = MagickWand::new();
    wand.read_image_blob(&image_buf).unwrap();

    assert_eq!(wand.get_image_width(), 50);
    assert_eq!(wand.get_image_format().unwrap(), "JPEG");
}

#[test]
fn it_does_not_resize_images_smaller_than_target() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind": "downsize_to_width"
                , "name": "thumbnail"
                , "width": 200 }
            ]
        }
        "#, "small.jpg");

    let mut image_buf = vec![0]; /* Distinguish uninitialized vector from the empty response we expect */

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "thumbnail" = field.name.as_str() {
                image_buf = Vec::new();
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();

    assert_eq!(image_buf, vec![] as Vec<u8>);
}

#[test]
fn it_downsizes_portrait_images() {
    let response = server_response!(r#"
        {
            "transforms": [
                { "kind": "downsize_to_width"
                , "name": "thumbnail"
                , "width": 300 }
            ]
        }
        "#, "tall.png");

    let mut image_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|mut field| {
            if let "thumbnail" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();

    let wand = MagickWand::new();
    wand.read_image_blob(&image_buf).unwrap();

    assert_eq!(wand.get_image_width(), 300);
    assert_eq!(wand.get_image_height(), 420);
    assert_eq!(wand.get_image_format().unwrap(), "PNG");
}
