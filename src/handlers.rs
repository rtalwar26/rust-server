use gotham::handler::IntoResponse;
use gotham::state::{State};
extern crate mime;
extern crate serde_json;

pub fn hello_handler(s:State)->(State,impl IntoResponse){
    (s, "hello_handler")
}

pub fn hello_post(s:State)->(State, impl IntoResponse){
    (s,"hello_post")
}

