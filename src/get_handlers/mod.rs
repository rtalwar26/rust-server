use gotham::handler::IntoResponse;
use gotham::state::State;
mod myenums;
mod mystructs;
use self::myenums::TimeUnit;
use self::mystructs::MyHttpStatus;
use self::mystructs::Queue;
const HELLO_ROUTER: &'static str = "Hello Router!";

pub fn hello_handler(s: State) -> (State, impl IntoResponse) {
    let mut o = Queue { older: Vec::new() };
    o.push('c');
    println!("{:?}", o.older);
    let mut o1 = Queue { older: Vec::new() };
    o1.push(2);
    println!("{:?}", o1.older);
    println!("{}", MyHttpStatus::http_status_from_u32(200).unwrap());
    println!("{}", TimeUnit::Seconds.plural());
    println!("{:?}", b"rajat");
    println!("{}", TimeUnit::ManHours(3, 24).plural());
    (s, HELLO_ROUTER)
}
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_ROUTER)
}
