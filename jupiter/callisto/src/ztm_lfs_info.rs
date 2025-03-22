//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ztm_lfs_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub file_hash: String,
    pub hash_type: String,
    pub file_size: i64,
    pub creation_time: i64,
    pub peer_id: String,
    pub origin: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
