use crate::models::{
    AllStaff, BasicResponse, Count, Filter, Oiv, OivRequest, StaffMember, StaffRequestParams,
};
use crate::profile::{Absence, HeadOfStructure, ProfileInfo, Statuses, Structure, Workplace};
use crate::{AppError, AppState, profile};
use rocket::{State, http::Status, serde::json::Json};

#[post(
    "/mobile/employees/v1/portals",
    format = "application/json",
    data = "<params>"
)]
pub async fn oivs(
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
            structure_type: vec![format!("staffpositions"), format!("management")],
        });
    }
    (Status::Ok, Json(BasicResponse::ok(oivs)))
}

#[post("/portals/v1/portals", format = "application/json", data = "<params>")]
pub async fn oivs_portals(
    state: &State<AppState>,
    params: Result<Json<OivRequest>, rocket::serde::json::Error<'_>>,
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

    let staff_params = StaffRequestParams {
        filters: Option::None,
        limit: Option::None,
        query: Option::None,
        after_id: Option::None,
    };
    let members = get_members(&state.staff.members, &staff_params, true);
    let mut oivs: Vec<Oiv> = vec![];
    for oiv in state.staff.oivs.clone() {
        let filtered_members: Vec<&StaffMember> = members
            .iter()
            .filter(|member| member.oiv.id == oiv.id.to_string())
            .collect();
        if params.ids.is_empty() == true || params.ids.contains(&oiv.id) {
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
                structure_type: vec![format!("staffpositions"), format!("management")],
            });
        }
    }
    (Status::Ok, Json(BasicResponse::ok(oivs)))
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
