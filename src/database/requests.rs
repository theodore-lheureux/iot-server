use rocket::FromForm;
use serde::Deserialize;

#[derive(Clone, Deserialize, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserRequest {
    #[field(validate = len(3..))]
    pub username: String,
    #[field(validate = len(3..))]
    pub password: String,
}
