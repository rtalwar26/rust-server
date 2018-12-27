use gotham::state::{FromState, State};
extern crate mime;
extern crate serde_json;

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct PathExtractor {
    name: String,
}
pub fn path_handler(state: State) -> (State, (mime::Mime, Vec<u8>)) {
    let message = {
        let product = PathExtractor::borrow_from(&state);
        format!("Product: {}", product.name);
        // serde_json::to_string(&product).expect("serialized product")
        // serde_json::to_json(&product).expect("serialized product")
        (
            mime::APPLICATION_JSON,
            serde_json::to_vec(&product).expect("serialized product"),
        )
    };
    (state, message)
}
