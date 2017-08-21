use iron::prelude::*;
use iron::status;

use iron::request::Body;

use multipart::server::{Multipart};
use multipart::client::lazy::Multipart as MultipartOutbound;
use std::io::{Read};

use types::{Manifest, ProcessingResult};

use serde_json::from_str as json_from_str;

type ProcessingFn = fn(Manifest, Vec<u8>) -> ProcessingResult;

pub fn run(processing_fn: ProcessingFn) {
    Iron::new(move |req: &mut Request| serve(req, processing_fn))
        .http("0.0.0.0:3030")
        .unwrap();
}

fn serve(req: &mut Request, processing_fn: ProcessingFn) -> IronResult<Response> {
    match Multipart::from_request(req) {
        Ok(data) => {
            match parse_multipart(data) {
                Ok((manifest, blob)) => {
                    let resp_buf = prepare_multipart(processing_fn(manifest, blob));

                    match resp_buf {
                        Ok(buf) => { Ok(Response::with((status::Ok, buf))) },
                        Err(err) => { Ok(Response::with((status::InternalServerError, err))) }
                    }
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

fn parse_multipart(mut data: Multipart<&mut Body>) -> Result<(Manifest, Vec<u8>), &'static str> {
    let mut data_manifest = None;
    let mut data_image_blob = None;

    data.foreach_entry(|mut field| {
        if let "manifest" = field.name.as_str() {
            data_manifest = field.data.as_text()
                .and_then(|enc| json_from_str(&enc).unwrap_or(None));
        }
        if let "image" = field.name.as_str() {
            let mut buf = Vec::new();

            let fully_read = field.data.as_file()
                .map(|f| f.read_to_end(&mut buf).is_ok())
                .unwrap_or(false);

            if fully_read { data_image_blob = Some(buf); }
        }
    }).or(Err("Malformed multipart request"))?;

    data_manifest
        .ok_or("Request parameter (\"manifest\") is missing or malformed")
        .and_then(|manifest| {
            data_image_blob
                .ok_or("Request parameter (\"image\") is missing or malformed")
                .map(|blob| (manifest, blob))
        })
}

fn prepare_multipart(result: ProcessingResult) -> Result<Vec<u8>, &'static str> {
    let ProcessingResult { image } = result;

    let mut resp_multipart = MultipartOutbound::new();
    resp_multipart.add_stream("image", &*image, None as Option<&str>, None);

    let res = resp_multipart.prepare();

    match res {
        Ok(mut fields) => {
            let mut out_buf = Vec::new();

            if fields.read_to_end(&mut out_buf).is_ok() { Ok(out_buf) }
            else { Err("Unable to export processing result as multipart data") }
        },
        _ => { Err("Unable to export processing result as multipart data") }
    }
}
