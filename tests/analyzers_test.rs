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
    let response = server_response!("{}", "small.jpg");

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
        "hash": "0100111111010010010100111101000000001111110101000010011011010011"
    }
    "#.replace("\n", "").replace(" ", ""))
}

#[test]
fn it_correctly_identifies_gif_dimensions() {
    let response = server_response!("{}", "dimensions.gif");

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
        "hash": "0101011001101111001100011100001000101011111001001001001101001101"
    }
    "#.replace("\n", "").replace(" ", ""))
}
