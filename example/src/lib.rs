mod bindings;

use crate::bindings::seungjin::pouch::rekuest::get;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_foo(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let http_body = get("https://catfact.ninja/fact").unwrap();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(http_body)
        .build())
}
