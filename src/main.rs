#[macro_use]
extern crate rocket;
use rocket::{State, fs::FileServer, http::Status, serde::json::Json};
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, path::Component::ParentDir};

use crate::models::Count;
use crate::{
    data_gen::DataSet,
    models::{
        AllStaff, BasicResponse, Filter, FilterWrapper, Filters, Oiv, StaffInfo, StaffMember,
        StaffRequestParams,
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

#[post(
    "/mobile/employees/v1/search",
    format = "application/json",
    data = "<params>"
)]
async fn colleagues(
    state: &State<AppState>,
    params: Result<Json<StaffRequestParams>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<AllStaff>>) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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
    let filtered_members = get_members(&state.staff.members, &params, false);
    (
        Status::Ok,
        Json(BasicResponse::ok(AllStaff {
            last_id: Option::None,
            list: filtered_members,
        })),
    )
}

#[post(
    "/mobile/employees/v1/portals",
    format = "application/json",
    data = "<params>"
)]
async fn oivs(
    state: &State<AppState>,
    params: Result<Json<StaffRequestParams>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<Vec<Oiv>>>) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let params = match params {
        Ok(params) => params.into_inner(),
        Err(_) => {
            return (
                Status::BadRequest,
                Json(BasicResponse::error("invalid request json")),
            );
        }
    };
    let members = get_members(&state.staff.members, &params, true);
    let mut oivs: Vec<Oiv> = vec![];
    for oiv in state.staff.oivs.clone() {
        let filtered_members: Vec<&StaffMember> = members
            .iter()
            .filter(|member| member.oiv.id == oiv.id.to_string())
            .collect();
        oivs.push(Oiv {
            id: oiv.id,
            icon_url: oiv.icon_url,
            short_name: oiv.short_name,
            name: oiv.name,
            count: Count {
                employees: Some(filtered_members.len() as u32),
                organizations: oiv.count.organizations,
            },
            head: oiv.head,
        });
    }
    (Status::Ok, Json(BasicResponse::ok(oivs)))
}

#[post(
    "/mobile/employees/v1/search/filters",
    format = "application/json",
    data = "<params>"
)]
async fn filters(
    state: &State<AppState>,
    params: Result<Json<StaffRequestParams>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<FilterWrapper>>) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let params = match params {
        Ok(params) => params.into_inner(),
        Err(_) => {
            return (
                Status::BadRequest,
                Json(BasicResponse::error("invalid request json")),
            );
        }
    };
    (
        Status::Ok,
        Json(BasicResponse::ok(FilterWrapper {
            filters: get_filters(&params, &state.staff),
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
        .mount("/", routes![colleagues, oivs, filters])
        .mount("/public", FileServer::from(public_dir))
}

fn get_members(
    members: &Vec<StaffMember>,
    params: &StaffRequestParams,
    takeAll: bool,
) -> Vec<StaffMember> {
    let mut members = members.clone();
    let params = params.clone();

    let filter = match params.filters {
        Some(filter) => filter,
        None => Filter::empty(),
    };

    let params_oivs = filter.oiv.unwrap_or(Vec::new());

    let limit: usize = params.limit.unwrap_or(20) as usize;
    if !params_oivs.is_empty() {
        members = members
            .into_iter()
            .filter(|f| {
                let oiv = f.oiv.clone();

                let Some(id) = oiv.id.parse::<u32>().ok() else {
                    return false;
                };
                params_oivs.contains(&id)
            })
            .collect();
    }
    if let Some(gender) = filter.gender {
        members = members
            .into_iter()
            .filter(|s| {
                let Some(staff_gender) = s.gender.clone() else {
                    return false;
                };
                staff_gender == gender
            })
            .collect();
    }
    if let Some(query) = params.query {
        if !query.is_empty() {
            members = members
                .into_iter()
                .filter(|f| f.full_name.to_lowercase().contains(&query.to_lowercase()))
                .collect();
        }
    }
    if takeAll {
        return members;
    }
    if let Some(last_id) = params.after_id {
        if !last_id.is_empty() {
            if let Some(index) = members.iter().position(|x| x.id == last_id) {
                if index + 1 < members.len() {
                    let mut last = index + limit;
                    if index + limit >= members.len() {
                        last = members.len() - 1
                    }
                    return members[index + 1..last].to_vec();
                }
            }
            return Vec::new();
        }
    }
    members.iter().take(limit).cloned().collect()
}

fn get_filters(params: &StaffRequestParams, data: &DataSet) -> Filters {
    let mut filters = Filters {
        oiv: Option::None,
        organisations: Option::None,
        products: Option::None,
        subdivisions: Option::None,
        positions: Option::None,
        addresses: Option::None,
        locations: Option::None,
    };
    let oivs: Vec<StaffInfo> = data
        .oivs
        .iter()
        .map(|oiv| StaffInfo {
            id: oiv.id.clone().to_string(),
            name: oiv.name.clone(),
        })
        .collect();

    let params_oivs: &[u32] = params
        .filters
        .as_ref()
        .and_then(|f| f.oiv.as_deref())
        .unwrap_or(&[]);

    let params_addresses: &[String] = params
        .filters
        .as_ref()
        .and_then(|f| f.addresses.as_deref())
        .unwrap_or(&[]);

    let params_organisations: &[String] = params
        .filters
        .as_ref()
        .and_then(|f| f.organisations.as_deref())
        .unwrap_or(&[]);

    let params_subdivisions: &[String] = params
        .filters
        .as_ref()
        .and_then(|f| f.subdivisions.as_deref())
        .unwrap_or(&[]);

    filters.oiv = Option::Some(oivs);

    if !params_addresses.is_empty() || !params_oivs.is_empty() {
        filters.organisations = Option::Some(data.organisations.clone());
    }
    if !params_oivs.is_empty() {
        filters.products = Option::Some(data.products.clone());
    }
    if !params_organisations.is_empty() && !params_addresses.is_empty() {
        filters.subdivisions = Option::Some(data.divisions.clone());
    }
    if !params_organisations.is_empty()
        && !params_addresses.is_empty()
        && !params_oivs.is_empty()
        && !params_subdivisions.is_empty()
    {
        filters.locations = Option::Some(data.locations.clone());
    }
    filters
}
