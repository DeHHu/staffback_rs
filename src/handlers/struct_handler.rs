use crate::models::{
    AllStaff, BasicResponse, Count, Desc, Filter, Oiv, OivRequest, OrgResponse, Organization,
    StaffMember, StaffPosition, StaffRequestParams, StructRequest,
};
use crate::profile::{Absence, HeadOfStructure, ProfileInfo, Statuses, Structure, Workplace};
use crate::{AppError, AppState, data_gen, profile};
use rocket::{State, http::Status, serde::json::Json};

#[post(
    "/employee/v1/staffpositions",
    format = "application/json",
    data = "<params>"
)]
pub async fn staff_positions(
    state: &State<AppState>,
    params: Result<Json<StructRequest>, rocket::serde::json::Error<'_>>,
) -> (Status, Json<BasicResponse<Vec<StaffPosition>>>) {
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

    let members = state.staff.members.clone();
    let orgs = data_gen::get_org(&state.base_url);
    let org = orgs
        .into_iter()
        .find(|o| o.id.to_string() == params.id)
        .unwrap();
    println!("--- org {:?}", org);
    let org_members: Vec<StaffMember> = members
        .into_iter()
        .filter(|m| m.organisation.id == org.id.to_string())
        .collect();

    let positions: Vec<StaffPosition> = org_members
        .iter()
        .map(|m| StaffPosition {
            id: m.id.clone(),
            staff_type: format!("employee"),
            name: m.full_name.clone(),
            image_url: m.photo_url.clone(),
            description: Desc {
                name: m.organisation.name.clone(),
                image_url: format!(""),
            },
            children: None,
        })
        .collect();
    println!("--- positions {:?}", positions.len());
    println!("--- org_members {:?}", org_members.len());
    // let strs = StaffPosition {
    //     id: params.id,
    //     staff_type: format!("organization"),
    //     name: format!("Организация"),
    //     image_url: format!(""),
    //     description: Desc {
    //         name: format!("Организация"),
    //         image_url: format!(""),
    //     },
    //     children: Some(vec![StaffPosition {
    //         id: 1.to_string(),
    //         staff_type: format!("subdivision"),
    //         name: format!("подразделение1"),
    //         image_url: format!(""),
    //         description: Desc {
    //             name: format!("Тата"),
    //             image_url: format!(""),
    //         },
    //         children: Some(positions),
    //     }]),
    // };

    //(Status::Ok, Json(BasicResponse::ok(vec![strs])))
    (Status::Ok, Json(BasicResponse::ok(positions)))
}
