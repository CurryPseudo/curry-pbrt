use std::borrow::Borrow;
use std::hash::Hash;
use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct AnyHashMap<K>(HashMap<K, Arc<dyn Any + 'static + Send + Sync>>);

impl<K: Eq + Hash> AnyHashMap<K> {
    pub fn insert<V: 'static + Send + Sync>(&mut self, key: K, value: V) {
        self.0.insert(key, Arc::new(value));
    }
    pub fn get<Q: ?Sized + Eq + Hash, V: 'static + Send + Sync>(&self, key: &Q) -> Option<Arc<V>> where K: Borrow<Q> {
        let any = self.0.get(key)?.clone();
        match any.downcast::<V>() {
            Ok(r) => Some(r),
            _ => None,
        }
    }
}
