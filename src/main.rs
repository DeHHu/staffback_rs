#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

use crate::models::{
    AllStaff, BasicResponse, Filter, StaffInfo, StaffList, StaffListFilter, StaffMember,
    StaffRequestParams,
};
mod data_gen;
mod models;

#[get("/")]
fn index() -> Json<BasicResponse<AllStaff>> {
    let set = data_gen::get_dataset("https://ya.ru");
    let params = StaffRequestParams {
        filters: Option::Some(StaffListFilter {
            filters: Filter {
                oiv: Option::Some(vec![3]),
                organisations: Option::None,
                products: Option::None,
                subdivisions: Option::None,
                positions: Option::None,
                addresses: Option::None,
                locations: Option::None,
                gender: Option::Some(String::from("female")),
                statuses: Option::None,
                employed_date_range: Option::None,
            },
        }),
        limit: 20,
        query: Option::Some(String::from("Л")),
        after_id: Option::None,
    };

    let filtered_members = get_members(set.members, params);

    Json(BasicResponse::ok(AllStaff {
        last_i_d: Option::None,
        list: filtered_members,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
// fn main() {

// }

fn get_members(members: Vec<StaffMember>, params: StaffRequestParams) -> Vec<StaffMember> {
    let mut members = members;

    if let Some(f) = params.filters {
        if let Some(oivs) = f.filters.oiv {
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

        if let Some(gender) = f.filters.gender {
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
