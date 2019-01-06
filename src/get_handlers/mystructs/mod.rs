extern crate hyper;

pub struct Queue<T> {
    pub older: Vec<T>,
}

impl<T> Queue<T> {
    pub fn push(&mut self, c: T) {
        self.older.push(c);
    }
}
pub struct MyHttpStatus {}
impl MyHttpStatus {
    pub fn http_status_from_u32(n: u32) -> Option<String> {
        match n {
            200 => Some("OK".to_string()),
            304 => Some("NotModified".to_string()),
            _ => None,
        }
    }
}
