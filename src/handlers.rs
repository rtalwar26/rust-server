extern crate futures;
extern crate hyper;
extern crate mime;
extern crate serde_json;
use self::futures::{future, Future, Stream};
use self::serde_json::{Error, Value};
use gotham::handler::IntoResponse;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::header::X_REQUEST_ID;
use gotham::state::request_id;
use gotham::state::{FromState, State};
use hyper::{Body, HeaderMap, Method, Response, StatusCode, Uri, Version};

fn print_request_elements(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = Version::borrow_from(state);
    let headers = HeaderMap::borrow_from(state);
    println!("Method: {:?}", method);
    println!("URI: {:?}", uri);
    println!("HTTP Version: {:?}", http_version);
    println!("Headers: {:?}", headers);
}

pub fn hello_handler(s: State) -> (State, impl IntoResponse) {
    (s, "hello_handler")
}

pub fn hello_post(mut state: State) -> Box<HandlerFuture> {
    print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                let v: Value = serde_json::from_str(&body_content).unwrap();
                println!("Body: {}", body_content);
                println!("Please call {} at the number {}", v["name"], v["phones"][0]);
                let res = create_empty_response(&state, StatusCode::OK);
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

fn create_empty_response(state: &State, status: StatusCode) -> Response<Body> {
    // new builder for the response
    let mut builder = Response::builder();

    // always add status and req-id
    builder.status(status);
    builder.header(X_REQUEST_ID, request_id(state));

    // attach an empty body by default
    let built = builder.body(Body::empty());

    // this expect should be safe due to generic bounds
    built.expect("Response built from a compatible type")
}
