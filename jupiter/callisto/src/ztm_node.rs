//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ztm_node")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub peer_id: String,
    pub hub: String,
    pub agent_name: String,
    pub service_name: String,
    pub r#type: String,
    pub online: bool,
    pub last_online_time: i64,
    pub service_port: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
