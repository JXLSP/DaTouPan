use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name="attack")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub source_asset_id: i32,
    pub target_asset_id: i32,
    pub path_details: String,
    pub risk_score: f32,
}