use iron::prelude::*;
use iron::status;

use iron::request::Body;

use multipart::server::{Multipart};

pub fn run() {
    Iron::new(serve)
        .http("0.0.0.0:3030")
        .unwrap();
}

fn serve(req: &mut Request) -> IronResult<Response> {
    match Multipart::from_request(req) {
        Ok(data) => {
            match parse_multipart(data) {
                Ok(manifest) => {
                    println!("{:?}", manifest);
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

fn parse_multipart(mut data: Multipart<&mut Body>) -> Result<String, &'static str> {
    let mut data_manifest = None;

    data.foreach_entry(|field| {
        if let "manifest" = field.name.as_str() {
            data_manifest = field.data.as_text().map(|s| s.to_owned());
        }
    }).or_else(|_| Err("Malformed multipart request"))?;

    data_manifest.ok_or("No manifest is provided in the request")
}
