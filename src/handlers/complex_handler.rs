use crate::data_gen::{self};
use crate::models::{BasicResponse, Complex};

use crate::AppState;
use rocket::{State, http::Status, serde::json::Json};

#[get("/portals/v1/complexes")]
pub async fn get_complexes(state: &State<AppState>) -> (Status, Json<BasicResponse<Vec<Complex>>>) {
    return (
        Status::Ok,
        Json(BasicResponse::ok(complexes(&state.base_url))),
    );
}

fn complexes(base_url: &str) -> Vec<Complex> {
    let heads = data_gen::get_heads(base_url);
    vec![
        Complex {
            id: 1,
            sort: 1,
            head: heads[0].clone(),
            oivs: vec![1],
        },
        Complex {
            id: 2,
            sort: 2,
            head: heads[1].clone(),
            oivs: vec![2],
        },
        Complex {
            id: 3,
            sort: 3,
            head: heads[2].clone(),
            oivs: vec![1, 3],
        },
    ]
}
