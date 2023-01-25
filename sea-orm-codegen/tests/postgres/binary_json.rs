//! SeaORM Entity. Generated by sea-orm-codegen 0.1.0
//! 
//! This file tests that the JsonBinary column type is annotated correctly is
//! compact entity form. More information can be found in this issue:
//! 
//! https://github.com/SeaQL/sea-orm/issues/1344

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub payload: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub payload_binary: Json,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}