extern crate reqwest;
extern crate multipart;
extern crate magick_rust;

use multipart::server::Multipart as MultipartInbound;

#[macro_use]
#[path = "utils.rs"]
mod utils;
use utils::{setup_client, MultipartResponse};

#[test]
fn it_analyzes_raster_images() {
    let client = setup_client();

    let response = client.post(vidalia_url!()).unwrap()
        .multipart(reqwest::multipart::Form::new()
            .text("manifest", "{}")
            .file("image", fixture_path!("small.jpg"))
            .unwrap()
        )
        .send().unwrap();

    let mut analyzed = "".to_string();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|field| {
            if let "analyzed" = field.name.as_str() {
                analyzed = field.data.as_text().unwrap().to_owned();
            }
        }).unwrap();

    assert_eq!(analyzed, r#"
    {
        "width": 200,
        "height": 198,
        "hash": "0010010110101011101011011011010110010111010100010101000101010001"
    }
    "#.replace("\n", "").replace(" ", ""))
}

#[test]
fn it_correctly_identifies_gif_dimensions() {
    let client = setup_client();

    let response = client.post(vidalia_url!()).unwrap()
        .multipart(reqwest::multipart::Form::new()
            .text("manifest", "{}")
            .file("image", fixture_path!("dimensions.gif"))
            .unwrap()
        )
        .send().unwrap();

    let mut analyzed = "".to_string();

    MultipartInbound::from_request(&mut MultipartResponse(response)).expect("response is not multipart")
        .foreach_entry(|field| {
            if let "analyzed" = field.name.as_str() {
                analyzed = field.data.as_text().unwrap().to_owned();
            }
        }).unwrap();

    assert_eq!(analyzed, r#"
    {
        "width": 604,
        "height": 340,
        "hash": "1000110110010101100001011000110111101101010001010110010101100101"
    }
    "#.replace("\n", "").replace(" ", ""))
}
