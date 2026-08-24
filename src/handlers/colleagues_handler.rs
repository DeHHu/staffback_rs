use crate::models::{AllStaff, BasicResponse, Filter, StaffMember, StaffRequestParams};
use crate::profile::{Absence, HeadOfStructure, ProfileInfo, Statuses, Structure, Workplace};
use crate::{AppError, AppState, profile};
use rocket::{State, http::Status, serde::json::Json};

#[post(
    "/mobile/employees/v1/search",
    format = "application/json",
    data = "<params>"
)]
pub async fn colleagues(
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
