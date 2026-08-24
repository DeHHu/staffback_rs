use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Profile {
    pub full_name: String,
    pub ext_id: String,
    pub gender: String,
    pub photo_url: String,
    pub birthday: String,
    pub email: String,
    pub max_messenger: String,
    pub vk_profile: String,
    pub date_of_employment: String,
    pub position: String,
    pub type_of_employment: String,
    pub statuses: Statuses,
    pub oiv: ProfileInfo,
    pub product: ProfileInfo,
    pub legal_entity: ProfileInfo,
    pub structure: Structure,
    pub work_phone: String,
    pub add_phone: String,
    pub mobile_phone: String,
    pub workplace: Workplace,
    pub head_of_org_structure: HeadOfStructure,
    pub head_of_manage_structure: HeadOfStructure,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Statuses {
    pub absence: Absence,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Absence {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ProfileInfo {
    pub name: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Structure {
    pub position: String,
    pub sub_unit: String,
    pub legal_entity: String,
    pub oiv: String,
    pub product: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Workplace {
    pub address: String,
    pub floor: String,
    pub cabinet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct HeadOfStructure {
    pub id: String,
    pub photo: String,
    pub firts_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub gender: String,
}
