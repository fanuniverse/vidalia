use iron::prelude::*;
use iron::status;
use iron::request::Body;
use iron::headers::ContentType;
use iron::modifiers::Header;

use multipart::server::Multipart as MultipartInbound;
use multipart::client::lazy::Multipart as MultipartOutbound;
use std::io::{Read};

use types::{Manifest, ProcessingResult};

use serde_json::from_str as json_from_str;
use serde_json::to_string as json_to_string;

type ProcessingFn = fn(Manifest, Vec<u8>) -> Result<ProcessingResult, &'static str>;

pub fn run(processing_fn: ProcessingFn) {
    Iron::new(move |req: &mut Request| serve(req, processing_fn))
        .http("0.0.0.0:3030")
        .unwrap();
}

fn serve(req: &mut Request, processing_fn: ProcessingFn) -> IronResult<Response> {
    match MultipartInbound::from_request(req) {
        Ok(data) => {
            match parse_multipart(data) {
                Ok((manifest, blob)) => {
                    Ok(processing_fn(manifest, blob)
                        .and_then(|processed| prepare_multipart(processed))
                        .map(|(header, data)| Response::with((status::Ok, Header(header), data)))
                        .unwrap_or_else(|err| Response::with((status::InternalServerError, err))))
                }
                Err(err) => {
                    Ok(Response::with((status::BadRequest, err)))
                }
            }
        }
        _ => {
            Ok(Response::with((status::BadRequest, "the request is not multipart")))
        }
    }
}

fn parse_multipart(mut data: MultipartInbound<&mut Body>) -> Result<(Manifest, Vec<u8>), &'static str> {
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

fn prepare_multipart(result: ProcessingResult) -> Result<(ContentType, Vec<u8>), &'static str> {
    let mut multipart = MultipartOutbound::new();

    let analyzed = json_to_string(&result.analyzed)
        .or(Err("Unable to serialize image analysis result"))?;
    multipart.add_text("analyzed", analyzed);

    for transformed_image in &result.transformed {
        multipart.add_stream(
            transformed_image.name.to_owned(),
            &*transformed_image.blob,
            None as Option<&str>, /* filename */
            None); /* content type (None = application/octet-stream) */
    }

    match multipart.prepare() {
        Ok(mut prepared) => {
            let content_type = format!("multipart/form-data; boundary={}", prepared.boundary());
            let header = ContentType(content_type.parse().unwrap());
            let mut body = Vec::new();

            if prepared.read_to_end(&mut body).is_ok() { Ok((header, body)) }
            else { Err("Unable to export processing result as multipart data") }
        },
        _ => { Err("Unable to export processing result as multipart data") }
    }
}
