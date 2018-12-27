extern crate crypto;
extern crate futures;
extern crate hyper;
extern crate mime;
extern crate serde_json;

use self::crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use self::crypto::{aes, blockmodes, buffer, symmetriccipher};
use self::futures::{future, Future, Stream};
use self::serde_json::Value;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::header::X_REQUEST_ID;
use gotham::state::request_id;
use gotham::state::{FromState, State};
use hyper::header::CONTENT_TYPE;
use hyper::{Body, HeaderMap, Method, Response, StatusCode, Uri, Version};

#[derive(Serialize, Deserialize)]
struct EncryptedBody {
    body: String,
}
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

pub fn hello_post(mut state: State) -> Box<HandlerFuture> {
    print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                // let v: Value = serde_json::from_str(&body_content).unwrap();
                let p: EncryptedBody = serde_json::from_str(&body_content).unwrap();

                println!("Body: {}", body_content);
                // println!("Please call {} at the number {}", v["name"], v["phones"][0]);
                let res = create_empty_response(p.body, &state, StatusCode::OK);
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}
#[derive(Serialize, Deserialize)]
struct DecryptBody {
    body: String,
}
fn create_empty_response(body: String, state: &State, status: StatusCode) -> Response<Body> {
    // new builder for the response
    let mut builder = Response::builder();

    // always add status and req-id
    builder.status(status);
    builder.header(X_REQUEST_ID, request_id(state));
    builder.header(CONTENT_TYPE, mime::APPLICATION_JSON.to_string());
    // attach an empty body by default
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let decrypted = decrypt(body.as_bytes(), &key, &iv).ok().unwrap();
    // let responseBody = str::from_utf8(decrypted).ok();
    let d_body = DecryptBody {
        body: String::from_utf8(decrypted).ok().unwrap(),
    };
    let chunks = vec![serde_json::to_string(&d_body).expect("serialized product")];

    let stream = futures::stream::iter_ok::<_, ::std::io::Error>(chunks);

    let body = Body::wrap_stream(stream);

    let built = builder.body(body);

    // this expect should be safe due to generic bounds
    built.expect("Response built from a compatible type")
}

fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}
