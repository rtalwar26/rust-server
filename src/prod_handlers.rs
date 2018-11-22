use gotham::state::{FromState, State};
extern crate mime;
extern crate serde_json;
// use std::{thread, time};

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct QueryStringExtractor {
    pub name: String,
    pub _type: Option<String>,
}

/// A Product
#[derive(Serialize)]
pub struct Product {
    name: String,
    _type: Option<String>,
}
pub fn get_product_handler(mut state: State) -> (State, (mime::Mime, Vec<u8>)) {
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
            _type: query_param._type,
        };


        // thread::sleep(time::Duration::from_millis(10000));

        (
            mime::APPLICATION_JSON,
            serde_json::to_vec(&product).expect("serialized product"),
        )
    };
    (state, res)
}
