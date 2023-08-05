use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub id: i32,
    #[serde(skip_serializing)]
    pub password: String,
    pub username: String,
}
