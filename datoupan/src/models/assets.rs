use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub ip_address: String,
    pub asset_type: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

