//! A Hello World example application for working with Gotham.

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use gotham::router::builder::*;
use gotham::router::Router;

mod get_handlers;
mod path_handlers;
mod post_handlers;
mod prod_handlers;
use self::get_handlers::*;
use self::path_handlers::*;
use self::post_handlers::*;

fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/products/:name")
            .with_path_extractor::<PathExtractor>()
            .to(path_handler);

        route.get("/").to(say_hello);
        route.get("/hello").to(hello_handler);
        route.post("/post").to(hello_post);
        route
            .get("/products")
            // This tells the Router that for requests which match this route that query string
            // extraction should be invoked storing the result in a `QueryStringExtractor` instance.
            .with_query_string_extractor::<self::prod_handlers::QueryStringExtractor>()
            .to(self::prod_handlers::get_product_handler);
    })
}
/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello Router!");
    }
}
