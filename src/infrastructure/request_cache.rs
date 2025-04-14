use crate::infrastructure::config::Config;
use anyhow::{Context, Result};
use blake2::{Blake2b512, Digest};
use cached::{DiskCache, IOCached};
use log::warn;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cell::LazyCell;

const CACHE_LIFESPAN_SECONDS: u64 = 43200;

pub struct RequestCache<'a> {
    /// cached instance wrapped in a LazyCell so the cache does not get built when it is never used.
    instance: LazyCell<DiskCache<String, String>>,
    config: &'a Config,
}

impl<'a> RequestCache<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            instance: LazyCell::new(|| {
                let name = format!("{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                
                DiskCache::<String, String>::new(&name)
                    .set_lifespan(CACHE_LIFESPAN_SECONDS)
                    .set_sync_to_disk_on_cache_change(true)
                    .build()
                    .expect("Could not initialize cache.")
            }),
            config,
        }
    }

    fn key(&self, method: &str, url: &str) -> String {
        Blake2b512::new()
            .chain_update(&self.config.bearer)
            .chain_update(method)
            .chain_update(url)
            .finalize()
            .to_vec()
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn get_cached<Value>(
        &self,
        use_cache: bool,
        method: &str,
        url: &str,
        get_value: impl Fn() -> Result<Value>,
    ) -> Result<Value>
    where
        Value: DeserializeOwned,
        Value: Serialize,
    {
        let key = self.key(method, url);
        if use_cache {
            let cached_value = self
                .instance
                .cache_get(&key)
                .with_context(|| "Could not get value from cache")?;
            if let Some(cached_value) = cached_value {
                let deserialized = serde_json::from_str::<Value>(&cached_value);
                match deserialized {
                    Err(err) => {
                        warn!(
                            "Removing valueCould not deserialize value from cache: {:?}",
                            err
                        );
                        self.instance
                            .cache_remove(&key)
                            .with_context(|| "Could not remove value from cache.")?;
                    }
                    Ok(deserialized) => return Ok(deserialized),
                }
            }
        }
        let value = get_value()?;
        if use_cache {
            self.instance
                .cache_set(
                    key,
                    serde_json::to_string(&value)
                        .with_context(|| "Could not serialize value to cache.")?,
                )
                .with_context(|| "Could not write cache.")?;
        }
        Ok(value)
    }
}
