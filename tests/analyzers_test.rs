extern crate reqwest;
extern crate multipart;
extern crate magick_rust;

use multipart::server::Multipart as MultipartInbound;

#[macro_use]
#[path = "utils.rs"]
mod utils;
use utils::{setup_client, MultipartResponse};

#[test]
fn it_analyzes_specified_properties() {
    let client = setup_client();

    let response = client.post(vidalia_url!()).unwrap()
        .multipart(reqwest::multipart::Form::new()
            .text("manifest", r#"
            {
                "analyzers": [
                    "width"
                ]
            }
            "#)
            .file("image", fixture_path!("small.jpg"))
            .unwrap()
        )
        .send().unwrap();

    let mut analyzed = "".to_string();

    MultipartInbound::from_request(&mut MultipartResponse(response))
        .unwrap_or_else(|_| panic!("expected multipart response"))
        .foreach_entry(|field| {
            if let "analyzed" = field.name.as_str() {
                analyzed = field.data.as_text().unwrap().to_owned();
            }
        }).unwrap();

    assert_eq!(analyzed, r#"
    {
        "width": 200
    }
    "#.replace("\n", "").replace(" ", ""))
}
