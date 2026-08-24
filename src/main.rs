#[macro_use]
extern crate rocket;
use crate::data_gen::DataSet;

use rocket::fs::FileServer;
use std::env;
use std::path::PathBuf;
mod data_gen;
mod handlers;
mod models;
mod profile;

struct AppState {
    staff: DataSet,
    base_url: String,
}
// type ApiResult<T> = Result<Json<BasicResponse<T>>, (Status, Json<BasicResponse<T>>)>;

fn base_url() -> String {
    env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
}

#[launch]
fn rocket() -> _ {
    let loaded = dotenvy::dotenv();
    println!("dotenv loaded: {:?}", loaded);
    println!("BASE_URL = {:?}", std::env::var("BASE_URL"));
    let state = AppState {
        staff: data_gen::get_dataset(&base_url()),
        base_url: String::from(&base_url()),
    };
    let public_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");
    println!("public_dir = {:?}", public_dir);
    println!("exists = {:?}", std::path::Path::new(&public_dir).exists());
    println!("is_dir = {:?}", std::path::Path::new(&public_dir).is_dir());
    rocket::build()
        .manage(state)
        .mount(
            "/",
            routes![
                handlers::colleagues,
                handlers::oivs,
                handlers::oivs_portals,
                handlers::filters,
                handlers::get_profile,
                handlers::get_complexes,
                handlers::get_organizations,
                handlers::staff_positions,
                handlers::mock_handler::push_settings,
                handlers::mock_handler::buttons,
                handlers::mock_handler::holiday,
                handlers::mock_handler::profile,
                handlers::mock_handler::subscribe,
                handlers::mock_handler::unsubscribe
            ],
        )
        .mount("/public", FileServer::from(public_dir))
}

pub enum AppError {
    Internal,
}
