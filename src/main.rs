//! A Hello World example application for working with Gotham.
extern crate gotham;
extern crate hyper;
extern crate mime;

use hyper::{Response, StatusCode};
use gotham::http::response::create_response;
use gotham::state::State;
use mime::TEXT_PLAIN;

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
fn say_hello(state: State) -> (State, Response) {
    let hello = String::from("Hello World!").into_bytes();
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((hello, TEXT_PLAIN)));
    (state, response)
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening to http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
