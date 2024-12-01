use actix_web::HttpResponse;
use validator::Validate;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 20, message = "username is too short"))]
    pub username: String,
    #[validate(length(min = 1, max = 20, message = "password is too short"))]
    pub password: String,
}

pub async fn login_(request: LoginRequest) -> HttpResponse {}

pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 20, message = "username is too short"))]
    pub username: String,
    #[validate(length(min = 3, max = 20, message = "password is too short"))]
    pub password: String,
    #[validate(length(min = 6, max = 20, message = "expires day is too short"))]
    pub expires_days: u32,
    #[validate(length(min = 6, max = 20))]
    pub email: String,
}
pub async fn create_account() -> HttpResponse {}