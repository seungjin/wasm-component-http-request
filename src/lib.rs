mod bindings;

use anyhow::Result;
use bindings::exports::seungjin::pouch::rekuest;
use url::Url;

use wasi::http::outgoing_handler::handle;
use wasi::http::outgoing_handler::OutgoingRequest;
use wasi::http::types::ErrorCode;
use wasi::http::types::{Fields, Method, Scheme};
use wasi::io::poll;

struct Component;

impl rekuest::Guest for Component {
    fn get(u: String) -> Result<String, u32> {
        let url = Url::parse(u.as_str()).unwrap();

        let scheme: &Scheme = match url.scheme() {
            "http" => &Scheme::Http,
            "https" => &Scheme::Https,
            x => &Scheme::Other(x.to_string()),
        };

        let fields = Fields::new();
        let req = OutgoingRequest::new(fields);
        req.set_method(&Method::Get).unwrap();
        req.set_scheme(Some(scheme)).unwrap();
        req.set_authority(Some(url.host_str().unwrap())).unwrap();
        req.set_path_with_query(Some(url.path())).unwrap();
        let res = handle(req, None).unwrap();
        let pollable = res.subscribe();
        poll::poll(&[&pollable]);

        let incoming_response = res.get().unwrap().unwrap().unwrap();
        let incoming_body = incoming_response.consume().unwrap();
        let input_stream = incoming_body.stream().unwrap();

        let mut body: Vec<u8> = Vec::new();

        loop {
            let item = match input_stream.read(1024) {
                Ok(x) => x,
                Err(x) => break,
            };
            if item.is_empty() {
                break;
            }
            for i in item.into_iter() {
                body.push(i);
            }
        }

        let body_string = String::from_utf8(body).unwrap();

        Ok(body_string)
    }
}

bindings::export!(Component with_types_in bindings);
