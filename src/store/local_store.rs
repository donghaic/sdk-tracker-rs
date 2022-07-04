use crate::store::format::{deserialize, Format, serialize};

use super::error::Result;

#[derive(Debug, Clone)]
pub struct LocalStore {
    format: Format,
    store: super::sled::SledStore,
}

impl LocalStore {
    pub fn from(store: super::sled::SledStore) -> Self {
        LocalStore { format: super::format::Format::Json, store }
    }
}

impl LocalStore {
    pub async fn set<V>(&self, key: impl AsRef<[u8]>, value: &V) -> Result<()>
        where V: serde::Serialize {
        self.store.set(
            key.as_ref().into(),
            serialize(value, &self.format)?.into(),
        ).await
    }

    pub async fn get<K, V>(&self, key: K) -> Result<Option<V>>
        where
            K: AsRef<[u8]>,
            V: serde::de::DeserializeOwned,
    {
        let val = self
            .store
            .get(key.as_ref().into())
            .await?;
        val.map(|val| deserialize(val.as_ref(), &self.format))
            .transpose()
    }
}


#[cfg(test)]
mod tests {
    use crate::models::ad_event::Status;
    use crate::store::sled::SledStore;

    use super::*;

    #[actix_rt::test]
    async fn test_store() {
        //let local_store = LocalStore { format: super::Format::Json, store: SledStore::open("./ads_db").unwrap() };
        let local_store = LocalStore::from(SledStore::open("./ads_db").unwrap());
        local_store.set("ad_status", &Status {
            sid: 1,
            req: 2,
            bid: 3,
            price: 4,
        }).await;
        let status: Status = local_store.get("ad_status").await.unwrap().unwrap();
        println!("{:?}", status)
    }
}