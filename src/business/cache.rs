use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::sync::RwLock;

use crate::business::error::CoreError;

// Global in-memory cache keyed by a concatenation of service_name and key.
// This keeps the cache simple and avoids nested maps or per-service type registries.
static CACHE: Lazy<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// Namespacing constant for field type maps, as per requirement
pub const FIELD_TYPE_MAP: &str = "FIELD_TYPE_MAP";
pub const ATTRIBUTE_TYPE_MAP: &str = "ATTRIBUTE_TYPE_MAP";

/// Get value from cache by (service_name, key); if missing, compute and store it via the provided async closure.
/// The cache stores a single value per combined key ("service_name:key").
pub async fn cache_get_or_compute<V, F, Fut>(
    service_name: &str,
    key: &str,
    compute: F,
) -> Result<V, CoreError>
where
    V: Clone + Send + Sync + 'static,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<V, CoreError>>,
{
    let combined_key = format!("{}:{}", service_name, key);

    // Fast path: try read lock and downcast to the expected value type
    if let Some(any_box) = CACHE.read().unwrap().get(&combined_key) {
        if let Some(val) = any_box.downcast_ref::<V>() {
            return Ok(val.clone());
        }
    }

    // Miss: compute without holding locks
    let computed: V = compute().await?;

    // Store with write lock
    let mut guard = CACHE.write().unwrap();
    guard.insert(combined_key, Box::new(computed.clone()));

    Ok(computed)
}

/// Update/insert a cache value for (service_name, key) using a supplier function.
/// This will overwrite any existing cached value for the same combined key.
/// Using a function makes it flexible and allows lazy construction of the value.
pub fn cache_update<V, F>(service_name: &str, key: &str, make: F)
where
    V: Any + Send + Sync + 'static,
    F: FnOnce() -> V,
{
    let combined_key = format!("{}:{}", service_name, key);
    let mut guard = CACHE.write().unwrap();
    let value = make();
    guard.insert(combined_key, Box::new(value));
}
