extern crate reqwest;
extern crate multipart;

use std::io::Read;
use std::path::Path;

use multipart::server::Multipart as MultipartInbound;

static URL: &'static str = "http://localhost:3030";

#[macro_use]
#[path = "utils.rs"]
mod utils;
use utils::{setup_client, MultipartResponse};

#[test]
fn it_downsizes_an_image() {
    let client = setup_client();

    let response = client.post(URL).unwrap()
        .multipart(reqwest::multipart::Form::new()
            .text("manifest", r#"
            {
                "transforms": [
                    { "kind": "downsize"
                    , "name": "thumbnail"
                    , "width": 10 }
                ]
            }
            "#)
            .file("image", fixture_path!("small.jpg"))
            .unwrap()
        )
        .send().unwrap();

    let mut image_buf = Vec::new();

    MultipartInbound::from_request(&mut MultipartResponse(response))
        .unwrap_or_else(|_| panic!("expected multipart response"))
        .foreach_entry(|mut field| {
            if let "thumbnail" = field.name.as_str() {
                field.data.as_file().unwrap().read_to_end(&mut image_buf).unwrap();
            }
        }).unwrap();

    assert_ne!(vec![] as Vec<u8>, image_buf);
}
