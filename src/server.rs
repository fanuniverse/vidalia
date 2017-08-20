use iron::prelude::*;
use iron::status;

use iron::request::Body;

use multipart::server::{Multipart};
use std::io::Read;

pub fn run() {
    Iron::new(serve)
        .http("0.0.0.0:3030")
        .unwrap();
}

fn serve(req: &mut Request) -> IronResult<Response> {
    match Multipart::from_request(req) {
        Ok(data) => {
            match parse_multipart(data) {
                Ok(res) => {
                    Ok(Response::with((status::Ok, "ok")))
                }
                Err(err) => {
                    Ok(Response::with((status::BadRequest, err)))
                }
            }
        }
        Err(_) => {
            Ok(Response::with((status::BadRequest, "the request is not multipart")))
        }
    }
}

fn parse_multipart(mut data: Multipart<&mut Body>) -> Result<(String, Vec<u8>), &'static str> {
    let mut data_manifest = None;
    let mut data_image_blob = None;

    data.foreach_entry(|mut field| {
        if let "manifest" = field.name.as_str() {
            data_manifest = field.data.as_text().map(|s| s.to_owned());
        }
        if let "image" = field.name.as_str() {
            let mut buf = Vec::new();

            let fully_read = field.data.as_file()
                .map(|f| f.read_to_end(&mut buf).is_ok())
                .unwrap_or(false);

            if fully_read { data_image_blob = Some(buf); }
        }
    }).or_else(|_| Err("Malformed multipart request"))?;

    data_manifest
        .ok_or("Request parameter (\"manifest\") is missing or malformed")
        .and_then(|manifest| {
            data_image_blob
                .ok_or("Request parameter (\"image\") is missing or malformed")
                .map(|blob| (manifest, blob))
        })
}
