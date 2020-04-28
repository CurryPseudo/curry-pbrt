use std::borrow::Borrow;
use std::hash::Hash;
use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct AnyHashMap<K>(HashMap<K, Arc<dyn Any + 'static + Send + Sync>>);

impl<K: Eq + Hash> AnyHashMap<K> {
    pub fn insert<V: 'static + Send + Sync>(&mut self, key: K, value: V) -> Arc<V> {
        let value = Arc::new(value);
        self.0.insert(key, value.clone());
        value
    }
    pub fn or_insert_with<F: FnOnce() -> V, V: 'static + Send + Sync>(&mut self, key: K, f: F) -> Arc<V> {
        if let Some(r) = self.get(&key) {
            r
        }
        else {
            self.insert(key, f())
        }
    }
    pub fn get<Q: ?Sized + Eq + Hash, V: 'static + Send + Sync>(&self, key: &Q) -> Option<Arc<V>> where K: Borrow<Q> {
        let any = self.0.get(key)?.clone();
        match any.downcast::<V>() {
            Ok(r) => Some(r),
            _ => None,
        }
    }
}

