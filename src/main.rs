//! A Hello World example application for working with Gotham.

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use gotham::router::builder::*;
use gotham::router::Router;

use gotham::state::{FromState, State};
mod handlers;
use self::handlers::*;

const HELLO_ROUTER: &'static str = "Hello Router!";

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_ROUTER)
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    name: String,
    _type: Option<String>
}

/// A Product
#[derive(Serialize)]
struct Product {
    name: String,
    _type: Option<String>
}

/// Handler function for `GET` requests directed to `/products`
///
/// This handler uses the Serde project when generating responses. You don't need to
/// know about Serde in order to understand the response that is being created here but if you're
/// interested you can learn more at `http://serde.rs`.
fn get_product_handler(mut state: State) -> (State, (mime::Mime, Vec<u8>)) {
    let res = {
        // Access the `QueryStringExtractor` instance from `state` which was put there for us by the
        // `Router` during request evaluation.
        //
        // As well as permitting storage in `State` by deriving `StateData` our query string
        // extractor struct automatically gains the `take_from` method and a number of other
        // methods via the `gotham::state::FromState` trait.
        //
        // n.b. Once taken out of `state` values can no longer be accessed by other application
        // code or middlewares.
        let query_param = QueryStringExtractor::take_from(&mut state);

        let product = Product {
            name: query_param.name,
            _type: query_param._type
        };
        

        (
            mime::APPLICATION_JSON,
            serde_json::to_vec(&product).expect("serialized product"),
        )
    };
    (state, res)
}



fn router()->Router{
    build_simple_router(|route|{
        route.get("/").to(say_hello);
        route.get("/hello").to(hello_handler);
        route.post("/post").to(hello_post);
        route
            .get("/products")
            // This tells the Router that for requests which match this route that query string
            // extraction should be invoked storing the result in a `QueryStringExtractor` instance.
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(get_product_handler);
    })
}
/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr,router())
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
