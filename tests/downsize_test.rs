extern crate vidalia;
extern crate reqwest;
extern crate multipart;

use std::io::Read;
use std::path::Path;

use std::thread;
use std::sync::{Once, ONCE_INIT};

use multipart::server::Multipart as MultipartInbound;

static START: Once = ONCE_INIT;
static URL: &'static str = "http://localhost:3030";

fn setup_client() -> reqwest::Client {
    START.call_once(|| {
        thread::spawn(vidalia::run);
    });

    reqwest::Client::new().unwrap()
}

macro_rules! fixture_path {
    ($fixture_name:expr) => (
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join($fixture_name)
            .to_str().unwrap()
    )
}

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

struct MultipartResponse(reqwest::Response);

impl Read for MultipartResponse {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'r> multipart::server::HttpRequest for &'r mut MultipartResponse {
    type Body = Self;

    fn multipart_boundary(&self) -> Option<&str> {
        const BOUNDARY: &'static str = "boundary=";

        let content_type = std::str::from_utf8(
            self.0.headers()
                .get_raw("Content-Type").unwrap()
                .one().unwrap())
            .unwrap();

        let boundary_start = content_type.find(BOUNDARY).unwrap() + BOUNDARY.len();
        let boundary = &content_type[boundary_start..content_type.len()];

        Some(boundary)
    }

    fn body(self) -> Self {
        self
    }
}
