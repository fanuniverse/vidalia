extern crate vidalia;
extern crate reqwest;
extern crate multipart;

use std::str;

use std::io;
use std::io::Read;

use std::thread;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

pub fn setup_client() -> reqwest::Client {
    START.call_once(|| {
        thread::spawn(vidalia::run);
    });

    reqwest::Client::new()
}

#[macro_export]
macro_rules! vidalia_url { () => { "http://localhost:3030" } }

#[macro_export]
macro_rules! fixture_path {
    ($fixture_name:expr) => (
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join($fixture_name)
            .to_str().unwrap()
    )
}

#[macro_export]
macro_rules! server_response {
    ($manifest:expr, $fixture_name:expr) => {
        setup_client()
            .post(vidalia_url!())
            .multipart(reqwest::multipart::Form::new()
                .text("manifest", $manifest)
                .file("image", fixture_path!($fixture_name)).unwrap()
            )
            .send().unwrap();
    }
}

#[derive(Debug)]
pub struct MultipartResponse(pub reqwest::Response);

impl Read for MultipartResponse {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'r> multipart::server::HttpRequest for &'r mut MultipartResponse {
    type Body = Self;

    fn multipart_boundary(&self) -> Option<&str> {
        const BOUNDARY: &'static str = "boundary=";

        let content_type = str::from_utf8(
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
