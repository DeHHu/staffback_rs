#[macro_use]
extern crate rocket;
use rocket::{State, fs::FileServer, http::Status, serde::json::Json};
use rocket_cors::Guard;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::{
    data_gen::DataSet,
    models::{
        AllStaff, BasicResponse, Filter, OivList, StaffInfo, StaffList, StaffListFilter,
        StaffMember, StaffRequestParams,
    },
};
mod data_gen;
mod models;

struct AppState {
    staff: DataSet,
}
// type ApiResult<T> = Result<Json<BasicResponse<T>>, (Status, Json<BasicResponse<T>>)>;

fn base_url() -> String {
    env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
}

#[rocket::get("/image-link")]
fn image_link() -> String {
    format!("{}/images/logo.png", base_url())
}

#[post("/v1/colleagues", format = "application/json", data = "<params>")]
fn colleagues(
    state: &State<AppState>,
    params: Result<Json<StaffRequestParams>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<AllStaff>>) {
    let params = match params {
        Ok(params) => params.into_inner(),
        Err(e) => {
            println!("{:?}", e);
            return (
                Status::BadRequest,
                Json(BasicResponse::error("invalid request json")),
            );
        }
    };
    let filtered_members = get_members(&state.staff.members, &params);
    (
        Status::Ok,
        Json(BasicResponse::ok(AllStaff {
            last_i_d: Option::None,
            list: filtered_members,
        })),
    )
}

#[post("/v1/colleagues/oivs", format = "application/json", data = "<params>")]
fn oivs(
    state: &State<AppState>,
    params: Result<Json<StaffRequestParams>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<OivList>>) {
    let params = match params {
        Ok(params) => params.into_inner(),
        Err(e) => {
            return (
                Status::BadRequest,
                Json(BasicResponse::error("invalid request json")),
            );
        }
    };
    (
        Status::Ok,
        Json(BasicResponse::ok(OivList {
            oivs: state.staff.oivs.clone(),
        })),
    )
}

#[launch]
fn rocket() -> _ {
    let loaded = dotenvy::dotenv();
    println!("dotenv loaded: {:?}", loaded);
    println!("BASE_URL = {:?}", std::env::var("BASE_URL"));
    let state = AppState {
        staff: data_gen::get_dataset(&base_url()),
    };
    let public_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");
    println!("public_dir = {:?}", public_dir);
    println!("exists = {:?}", std::path::Path::new(&public_dir).exists());
    println!("is_dir = {:?}", std::path::Path::new(&public_dir).is_dir());
    rocket::build()
        .manage(state)
        .mount("/", routes![colleagues, oivs])
        .mount("/public", FileServer::from(public_dir))
}

fn get_members(members: &Vec<StaffMember>, params: &StaffRequestParams) -> Vec<StaffMember> {
    let mut members = members.clone();
    let params = params.clone();

    if let Some(f) = params.filters {
        if let Some(oivs) = f.oiv {
            if !oivs.is_empty() {
                members = members
                    .into_iter()
                    .filter(|f| {
                        if let Some(oiv) = f.oiv.as_ref() {
                            let oiv_id_i32 = oiv.id.parse::<i32>().unwrap_or(-1);
                            if oiv_id_i32 < 0 {
                                return false;
                            }
                            let oiv_id: u32 = oiv_id_i32 as u32;
                            return oivs.contains(&oiv_id);
                        }
                        false
                    })
                    .collect();
            }
        }

        if let Some(gender) = f.gender {
            members = members
                .into_iter()
                .filter(|s| s.gender.as_ref().is_some_and(|o| *o == gender))
                .collect();
        }
    }

    if let Some(query) = params.query {
        if !query.is_empty() {
            members = members
                .into_iter()
                .filter(|f| f.full_name.to_lowercase().contains(&query.to_lowercase()))
                .collect();
        }
    }

    if let Some(last_id) = params.after_id {
        if !last_id.is_empty() {
            if let Some(index) = members.iter().position(|x| x.id == last_id) {
                if index + 1 < members.len() {
                    let mut last = index + 20;
                    if index + 20 >= members.len() {
                        last = members.len() - 1
                    }
                    return members[index + 1..last].to_vec();
                }
            }
            return Vec::new();
        }
    }
    members.iter().take(20).cloned().collect()
}
