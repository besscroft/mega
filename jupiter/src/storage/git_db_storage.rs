use std::{env, sync::Arc};

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use callisto::refs;
use common::errors::MegaError;
use venus::internal::pack::reference::RefCommand;
use venus::internal::pack::reference::Refs;
use venus::repo::Repo;

use crate::{
    raw_storage::{self, RawStorage},
    storage::GitStorageProvider,
};

#[derive(Clone)]
pub struct GitDbStorage {
    pub raw_storage: Arc<dyn RawStorage>,
    pub connection: Arc<DatabaseConnection>,
    pub raw_obj_threshold: usize,
}

#[async_trait]
impl GitStorageProvider for GitDbStorage {
    async fn save_ref(&self, repo: &Repo, refs: &RefCommand) -> Result<(), MegaError> {
        let mut model: refs::Model = refs.clone().into();
        model.repo_id = repo.repo_id;
        let a_model = model.into_active_model();
        refs::Entity::insert(a_model)
            .exec(self.get_connection())
            .await
            .unwrap();
        Ok(())
    }

    async fn remove_ref(&self, repo: &Repo, refs: &RefCommand) -> Result<(), MegaError> {
        refs::Entity::delete_many()
            .filter(refs::Column::RepoId.eq(repo.repo_id))
            .filter(refs::Column::RefName.eq(refs.ref_name.clone()))
            .exec(self.get_connection())
            .await?;
        Ok(())
    }

    async fn get_ref(&self, repo: &Repo) -> Result<Vec<Refs>, MegaError> {
        let result = refs::Entity::find()
            .filter(refs::Column::RepoId.eq(repo.repo_id))
            // .filter(refs::Column::RefName.eq(ref_name))
            .all(self.get_connection())
            .await?;
        // if let Some(model) = result {
        //     return Ok(model.ref_git_id);
        // }
        // Ok(String::new())
        let res: Vec<Refs> = result.into_iter().map(|x| x.into()).collect();
        Ok(res)
    }

    async fn update_ref(&self, repo: &Repo, ref_name: &str, new_id: &str) -> Result<(), MegaError> {
        let ref_data: refs::Model = refs::Entity::find()
            .filter(refs::Column::RepoId.eq(repo.repo_id))
            .filter(refs::Column::RefName.eq(ref_name))
            .one(self.get_connection())
            .await
            .unwrap()
            .unwrap();
        let mut ref_data: refs::ActiveModel = ref_data.into();
        ref_data.ref_git_id = Set(new_id.to_string());
        ref_data.updated_at = Set(chrono::Utc::now().naive_utc());
        ref_data.update(self.get_connection()).await.unwrap();
        Ok(())
    }
}

impl GitDbStorage {
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn new(connection: Arc<DatabaseConnection>) -> Self {
        let raw_obj_threshold = env::var("MEGA_BIG_OBJ_THRESHOLD_SIZE")
            .expect("MEGA_BIG_OBJ_THRESHOLD_SIZE not configured")
            .parse::<usize>()
            .unwrap();
        let storage_type = env::var("MEGA_RAW_STORAGE").unwrap();
        let path = env::var("MEGA_OBJ_LOCAL_PATH").unwrap();
        GitDbStorage {
            connection,
            raw_storage: raw_storage::init(storage_type, path).await,
            raw_obj_threshold,
        }
    }

    pub fn mock() -> Self {
        GitDbStorage {
            connection: Arc::new(DatabaseConnection::default()),
            raw_storage: raw_storage::mock(),
            raw_obj_threshold: 1024,
        }
    }
}
