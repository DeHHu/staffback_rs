use rocket::{Data, http::ContentType, serde::json::Json};

use crate::models::NextHolidayRequestBody;
use serde_json::Value;

#[get("/mobileproxy/hs/mobileproxy/v8/employees/<id>")]
pub async fn profile(id: &str) -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (ContentType::JSON, include_str!("../mocks/profile.json"))
}

#[get("/mobileproxy/hs/mobileproxy/v6/pushSettings")]
pub async fn push_settings() -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (
        ContentType::JSON,
        include_str!("../mocks/pushsettings.json"),
    )
}

#[get("/mobileproxy/hs/mobileproxy/v5/employees/<id>/buttons")]
pub async fn buttons(id: &str) -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (ContentType::JSON, include_str!("../mocks/buttons.json"))
}

#[post(
    "/mobileproxy/hs/mobileproxy/getEmployeeNextHoliday",
    format = "application/json",
    data = "<params>"
)]
pub async fn holiday(
    params: Result<Json<NextHolidayRequestBody>, rocket::serde::json::Error<'_>>,
) -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (ContentType::JSON, include_str!("../mocks/nextholiday.json"))
}

#[post("/mobileproxy/hs/mobileproxy/subscribe", data = "<payload>")]
pub async fn subscribe(payload: Data<'_>) -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (ContentType::JSON, include_str!("../mocks/subscribe.json"))
}

#[post("/mobileproxy/hs/mobileproxy/unSubscribe", data = "<payload>")]
pub async fn unsubscribe(payload: Data<'_>) -> (ContentType, &'static str) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    (ContentType::JSON, include_str!("../mocks/subscribe.json"))
}
