use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct BasicResponse<T>
where
    T: Serialize,
{
    pub data: Option<T>,
    pub error: Option<ResponseError>,
    pub timestamp: Option<String>,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ResponseError {
    pub message: Option<String>,
}

impl<T> BasicResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            data: None,
            error: Some(ResponseError {
                message: Some(msg.to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}

// #[derive(Serialize, Debug, Clone)]
// #[serde(crate = "rocket::serde", rename_all = "camelCase")]
// pub struct StaffList {
//     pub last_id: Option<String>,
//     pub last_name: String,
//     pub age: u32,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct StaffInfo {
    pub id: String,
    pub name: String,
}

impl StaffInfo {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    pub to: Option<String>,
}
impl Status {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            to: Option::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct AllStaff {
    pub last_id: Option<String>,
    pub list: Vec<StaffMember>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct StaffMember {
    pub id: String,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Option<String>,
    pub photo_url: String,
    pub position: StaffInfo,
    pub oiv: StaffInfo,
    pub product: Option<StaffInfo>,
    pub organisation: Option<StaffInfo>,
    pub subdivision: Option<StaffInfo>,
    pub statuses: Option<Vec<Status>>,
    pub employment_type: Option<String>,
}

impl StaffMember {
    pub fn new(
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        gender: impl Into<String>,
        photo_url: impl Into<String>,
        position: StaffInfo,
        oiv: StaffInfo,
        product: StaffInfo,
        organisation: StaffInfo,
        subdivision: StaffInfo,
        statuses: Vec<Status>,
        employment_type: impl Into<String>,
    ) -> Self {
        let first_name = first_name.into();
        let last_name = last_name.into();
        let full_name = format!("{0} {1}", first_name.clone(), last_name.clone());
        Self {
            id: Uuid::new_v4().to_string(),
            full_name: full_name,
            first_name: first_name,
            last_name: last_name,
            middle_name: Option::None,
            gender: Option::Some(gender.into()),
            photo_url: photo_url.into(),
            position: position,
            oiv: oiv,
            product: Option::Some(product),
            organisation: Option::Some(organisation),
            subdivision: Option::Some(subdivision),
            statuses: Option::Some(statuses),
            employment_type: Option::Some(employment_type.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Head {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub position: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Oiv {
    pub id: u32,
    pub icon_url: Option<String>,
    pub short_name: String,
    pub name: String,
    pub count: Count,
    pub head: Option<Head>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Count {
    pub employees: Option<u32>,
    pub organizations: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct FilterWrapper {
    pub filters: Filters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Filters {
    pub oiv: Option<Vec<StaffInfo>>,
    pub organisations: Option<Vec<StaffInfo>>,
    pub products: Option<Vec<StaffInfo>>,
    pub subdivisions: Option<Vec<StaffInfo>>,
    pub positions: Option<Vec<StaffInfo>>,
    pub addresses: Option<Vec<StaffInfo>>,
    pub locations: Option<Vec<StaffInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Filter {
    pub oiv: Option<Vec<u32>>,
    pub organisations: Option<Vec<String>>,
    pub products: Option<Vec<String>>,
    pub subdivisions: Option<Vec<String>>,
    pub positions: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
    pub locations: Option<Vec<String>>,
    pub gender: Option<String>,
    pub statuses: Option<Vec<String>>,
    pub employed_date_range: Option<EmployedDateRange>,
}

impl Filter {
    pub fn empty() -> Self {
        Self {
            oiv: Option::None,
            organisations: Option::None,
            products: Option::None,
            subdivisions: Option::None,
            positions: Option::None,
            addresses: Option::None,
            locations: Option::None,
            gender: Option::None,
            statuses: Option::None,
            employed_date_range: Option::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct EmployedDateRange {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct StaffRequestParams {
    pub filters: Option<Filter>,
    pub limit: Option<u32>,
    pub query: Option<String>,
    pub after_id: Option<String>,
}
