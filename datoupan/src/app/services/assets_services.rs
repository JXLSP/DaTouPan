use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Serialize)]
pub struct CreateAssets {
    #[validate(length(min = 1, max = 100))]
    pub asset_name: String,
    #[validate(length(min = 1, max = 100))]
    pub ip_address: String,
    #[validate(length(min = 1, max = 100))]
    pub asset_type: String,
    #[validate(length(min = 1, max = 100))]
    pub asset_instance: String,
    #[validate(length(min = 1, max = 100))]
    pub asset_location: String,
}

pub async fn created_asset_services(request: CreateAssets) -> HttpResponse {}