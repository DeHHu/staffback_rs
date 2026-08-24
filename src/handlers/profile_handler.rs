use crate::models::{BasicResponse, StaffMember};
use crate::profile::{Absence, HeadOfStructure, ProfileInfo, Statuses, Structure, Workplace};
use crate::{AppError, AppState, profile};
use rocket::{State, http::Status, serde::json::Json};

#[get("/mobile/employees/v1/employees/<id>")]
pub async fn get_profile(
    id: &str,
    state: &State<AppState>,
) -> (Status, Json<BasicResponse<profile::Profile>>) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let res = get_profile_by_id(id, &state.staff.members).await;
    match res {
        Ok(s) => {
            let profile = get_profile_from_colleague(s, &state.base_url);
            return (Status::Ok, Json(BasicResponse::ok(profile)));
        }
        Err(e) => {
            return (
                Status::NotFound,
                Json(BasicResponse::error("can not find profile")),
            );
        }
    }
}

async fn get_profile_by_id(id: &str, members: &Vec<StaffMember>) -> Result<StaffMember, AppError> {
    if let Some(member) = members.into_iter().find(|p| p.id == id) {
        return Ok(member.clone());
    } else {
        return Err(AppError::Internal);
    }
}

fn get_profile_from_colleague(member: StaffMember, base_url: &str) -> profile::Profile {
    profile::Profile {
        full_name: member.full_name,
        ext_id: member.id,
        gender: member.gender.unwrap_or(String::from("male")),
        photo_url: member.photo_url,
        birthday: String::from("03-29"),
        email: String::from("yakov@apostol.ru"),
        max_messenger: String::from(""),
        vk_profile: String::from(""),
        date_of_employment: String::from("39900-01-20"),
        position: String::from(
            "Наладчик лазера и специалист в области особых материй. Укротитель машинного духа.",
        ),
        type_of_employment: String::from("Основное место работы"),
        statuses: Statuses {
            absence: Absence {
                from: String::from(""),
                to: String::from(""),
                type_field: String::from(""),
            },
        },
        oiv: ProfileInfo {
            name: String::from(""),
            icon: String::from(""),
        },
        product: ProfileInfo {
            name: String::from(""),
            icon: String::from(""),
        },
        legal_entity: ProfileInfo {
            name: String::from(""),
            icon: String::from(""),
        },
        structure: Structure {
            position: String::from(
                "Наладчик лазера и специалист в области особых материй. Укротитель машинного духа.",
            ),
            sub_unit: String::from("Разработка и поддержка передовых вооружений"),
            legal_entity: String::from("Императорское подразделение Инфоргород"),
            oiv: String::from("Ультрамарины"),
            product: String::from(""),
        },
        work_phone: String::from("+7 (495) 800-20-20"),
        add_phone: String::from("333"),
        mobile_phone: String::from("+7 (900) 800-20-22"),
        workplace: Workplace {
            address: String::from("Яковоапостольский перулок границы миров"),
            floor: String::from("33"),
            cabinet: String::from("314"),
        },
        head_of_org_structure: HeadOfStructure {
            id: String::from("12312312"),
            photo: format!("{0}/public/w40k/{1}.jpg", base_url, "rg"),
            firts_name: String::from("Робаут"),
            middle_name: String::from("Олегович"),
            last_name: String::from("Жиллиман"),
            gender: String::from("male"),
        },
        head_of_manage_structure: HeadOfStructure {
            id: String::from("42342423"),
            photo: format!("{0}/public/w40k/{1}.jpg", base_url, "lr"),
            firts_name: String::from("Леман"),
            middle_name: String::from("Кириллович"),
            last_name: String::from("Русс"),
            gender: String::from("male"),
        },
    }
}
