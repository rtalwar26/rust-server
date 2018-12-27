use gotham::handler::IntoResponse;
use gotham::state::State;
const HELLO_ROUTER: &'static str = "Hello Router!";

pub fn hello_handler(s: State) -> (State, impl IntoResponse) {
    (s, "hello_handler")
}
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_ROUTER)
}
