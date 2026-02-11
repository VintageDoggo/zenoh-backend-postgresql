use std::{collections::{HashMap, hash_map::Entry}, thread::spawn, vec};

use async_trait::async_trait;
use tokio::sync::RwLock;
use zenoh::{
    bytes::{Encoding, ZBytes},
    internal::bail,
    key_expr::OwnedKeyExpr,
    time::Timestamp,
    Result as ZResult,
};
use zenoh_backend_traits::{
    config::{StorageConfig, VolumeConfig},
    Capability, History, Persistence, Storage, StorageInsertionResult, StoredData, Volume,
    VolumeInstance,
};
use zenoh_plugin_trait::{plugin_long_version, plugin_version, Plugin};
use zenoh_util::ffi::JsonValue;
use sqlx::{Connection, PgPool};

// sqlx needs to refer to a Tokio runtime
const WORKER_THREAD_NUM: usize = 3;
const MAX_BLOCK_THREAD_NUM: usize = 50;
lazy_static::lazy_static! {
    // The global runtime is used in the dynamic plugins, which we can't get the current runtime
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
               .worker_threads(WORKER_THREAD_NUM)
               .max_blocking_threads(MAX_BLOCK_THREAD_NUM)
               .enable_all()
               .build()
               .expect("Unable to create runtime");
}


#[cfg(feature = "dynamic_plugin")]
zenoh_plugin_trait::declare_plugin!(PgBackend);

impl Plugin for PgBackend {
    type StartArgs = VolumeConfig;
    type Instance = VolumeInstance;
    fn start(_name: &str, _args: &Self::StartArgs) -> ZResult<Self::Instance> {
        // Create a connection pool for Postgres, pass it around
        // sqlx is async, so we make a blocking async call to stay cool
        let result = TOKIO_RUNTIME.block_on(async {
            sqlx::PgPool::connect("postgresql//postgres@localhost").await
        });
        let volume : PgBackend;
        match result {
            Ok(pool) => {
                volume = PgBackend { pool : pool };
            }
            Err(e) => {
                bail!("[PGSQL] Error creating connection pool. Details: {}", e);
            }
        }

        Ok(Box::new(volume))
    }

    const DEFAULT_NAME: &'static str = "postgres_backend";
    const PLUGIN_VERSION: &'static str = plugin_version!();
    const PLUGIN_LONG_VERSION: &'static str = plugin_long_version!();
}

pub struct PgBackend {
    pool: PgPool
}

pub struct PgStorage {
    pool: PgPool
}

#[async_trait]
impl Volume for PgBackend {
    //TODO: Add query to return admin status 
    fn get_admin_status(&self) -> JsonValue {
        serde_json::Value::Null.into()
    }
    fn get_capability(&self) -> Capability {
        Capability {
            persistence: Persistence::Volatile,
            history: History::All,
        }
    }
    async fn create_storage(&self, _props: StorageConfig) -> ZResult<Box<dyn Storage>> {
        // we make sure that our database and table all exist
        
        
        
        // cloning the pool object creates a reference to the original object
        Ok(Box::new(PgStorage {
            pool : self.pool.clone()
        }))
    }
}

#[async_trait]
impl Storage for PgStorage {
    fn get_admin_status(&self) -> JsonValue {
        serde_json::Value::Null.into()
    }
    async fn put(
        &mut self,
        key: Option<OwnedKeyExpr>,
        payload: ZBytes,
        encoding: Encoding,
        timestamp: Timestamp,
    ) -> ZResult<StorageInsertionResult> {
        
        return Ok(StorageInsertionResult::Inserted); // or Updated
    }

    async fn delete(
        &mut self,
        key: Option<OwnedKeyExpr>,
        _timestamp: Timestamp,
    ) -> ZResult<StorageInsertionResult> {
        
        Ok(StorageInsertionResult::Deleted)
    }

    async fn get(
        &mut self,
        key: Option<OwnedKeyExpr>,
        _parameters: &str,
    ) -> ZResult<Vec<StoredData>> {
        let data_vec : Vec<StoredData> = Vec::new();
        
        Ok(data_vec)
    }

    async fn get_all_entries(&self) -> ZResult<Vec<(Option<OwnedKeyExpr>, Timestamp)>> {
        let mut result : Vec<(Option<OwnedKeyExpr>, Timestamp)> = Vec::new();
        Ok(result)
    }
}
