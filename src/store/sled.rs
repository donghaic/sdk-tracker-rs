use std::path::PathBuf;
use std::sync::Arc;

use sled;

use crate::store::error::Result as StorageResult;
use crate::store::error::StorageError;

// sled storage stuff starts here
/// thin wrapper around the sled db
#[derive(Clone, Debug)]
pub struct SledStore {
    db: sled::Db,
}


impl SledStore {
    /// create the sled db at some specified path
    pub fn open(path: impl Into<PathBuf>) -> sled::Result<SledStore> {
        Ok(Self {
            db: sled::open(path.into())?,
        })
    }
}

impl SledStore {
    pub(crate) async fn set(&self, key: Arc<[u8]>, value: Arc<[u8]>) -> StorageResult<()> {
        match self.db.insert(key, value.as_ref()) {
            Ok(_) => Ok(()),
            Err(err) => Err(StorageError::custom(err)),
        }
    }

    pub(crate) async fn get(&self, key: Arc<[u8]>) -> StorageResult<Option<Arc<[u8]>>> {
        Ok(self.db.get(key)
            .map_err(StorageError::custom)?
            .map(|val| val.as_ref().into()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_store() {
        let mut store = SledStore::open("./ads_db").unwrap();
        store.set("heloo".as_bytes().into(), "world".as_bytes().into()).await;

        let result = store.get("heloo".as_bytes().into()).await;

        let v = result.unwrap().unwrap();
        println!("{:?}", String::from_utf8_lossy(&v))
    }
}