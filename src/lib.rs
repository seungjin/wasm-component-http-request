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
        match incoming_response.status() {
            _ => {
                let a = incoming_response.consume().unwrap();
                let b = a.stream().unwrap();

                let a = incoming_response
                    .headers()
                    .get(&"content-length".to_string());

                let joined: Vec<u8> =
                    a.iter().flat_map(|v| v.iter().cloned()).collect();

                let content_length =
                    String::from_utf8(joined).unwrap().parse::<u64>().unwrap();

                let content =
                    String::from_utf8(b.read(content_length).unwrap())
                        .unwrap();

                Ok(content)
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
