use gotham::state::State;
use gotham::handler::IntoResponse;

pub fn hello_handler(s:State)->(State,impl IntoResponse){
    (s, "hello_handler")
}

pub fn hello_post(s:State)->(State, impl IntoResponse){
    (s,"hello_post")
}