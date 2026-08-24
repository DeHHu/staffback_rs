use crate::data_gen::DataSet;
use crate::models::{
    AllStaff, BasicResponse, Count, Filter, FilterWrapper, Filters, Oiv, StaffInfo, StaffMember,
    StaffRequestParams,
};
use crate::profile::{Absence, HeadOfStructure, ProfileInfo, Statuses, Structure, Workplace};
use crate::{AppError, AppState, profile};
use rocket::{State, http::Status, serde::json::Json};

#[post(
    "/mobile/employees/v1/search/filters",
    format = "application/json",
    data = "<params>"
)]
pub async fn filters(
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

    let params_addresses: &str = params
        .filters
        .as_ref()
        .and_then(|f| f.addresses.as_deref())
        .unwrap_or("default");

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
    filters.addresses = Option::Some(data.addresses.clone());
    filters
}
