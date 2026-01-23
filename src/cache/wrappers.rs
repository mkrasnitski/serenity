//! Wrappers around library types for easier use.

use std::fmt::Debug;
use std::hash::Hash;
#[cfg(feature = "temp_cache")]
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use dashmap::mapref::one::{Ref, RefMut};
#[cfg(feature = "typesize")]
use typesize::TypeSize;

pub type HashMap<K, V> = DashMap<K, V, foldhash::fast::RandomState>;
#[cfg(feature = "temp_cache")]
pub type MokaCache<K, V> = mini_moka::sync::Cache<K, V, foldhash::fast::RandomState>;

/// A wrapper around Option<DashMap<K, V>> to ease disabling specific cache fields.
#[cfg_attr(feature = "typesize", derive(TypeSize))]
#[derive(Debug)]
pub struct MaybeMap<K, V>(pub Option<HashMap<K, V>>);

impl<K: Eq + Hash, V> MaybeMap<K, V> {
    pub fn iter(&self) -> impl Iterator<Item = RefMulti<'_, K, V>> {
        self.0.iter().flat_map(DashMap::iter)
    }

    pub fn get(&self, key: &K) -> Option<Ref<'_, K, V>> {
        self.0.as_ref()?.get(key)
    }

    pub fn get_mut(&self, key: &K) -> Option<RefMut<'_, K, V>> {
        self.0.as_ref()?.get_mut(key)
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.0.as_ref()?.insert(key, value)
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        self.0.as_ref()?.remove(key)
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, DashMap::len)
    }

    pub fn shrink_to_fit(&self) {
        if let Some(map) = self.0.as_ref() {
            map.shrink_to_fit();
        }
    }

    pub(crate) fn as_read_only(&self) -> ReadOnlyMapRef<'_, K, V> {
        ReadOnlyMapRef(self.0.as_ref())
    }
}

/// A wrapper around a reference to a MaybeMap, allowing for public inspection of the underlying
/// map without allowing mutation of internal cache fields, which could cause issues.
#[cfg_attr(feature = "typesize", derive(TypeSize))]
#[derive(Clone, Copy, Debug)]
pub struct ReadOnlyMapRef<'a, K, V>(Option<&'a HashMap<K, V>>);
impl<K: Eq + Hash, V> ReadOnlyMapRef<'_, K, V> {
    pub fn iter(&self) -> impl Iterator<Item = RefMulti<'_, K, V>> {
        self.0.into_iter().flat_map(DashMap::iter)
    }

    pub fn get(&self, k: &K) -> Option<Ref<'_, K, V>> {
        self.0?.get(k)
    }

    pub fn len(&self) -> usize {
        self.0.map_or(0, DashMap::len)
    }

    pub fn contains(&self, k: &K) -> bool {
        self.0.is_some_and(|m| m.contains_key(k))
    }
}

impl<'a, K, V> From<&'a MaybeMap<K, V>> for ReadOnlyMapRef<'a, K, V> {
    fn from(value: &'a MaybeMap<K, V>) -> Self {
        Self(value.0.as_ref())
    }
}

/// Wrapper around `SizableArc<T, Owned>`` with support for disabling typesize.
///
/// This denotes an Arc where T's size should be considered when calling `TypeSize::get_size`
#[derive(Debug)]
#[cfg(feature = "temp_cache")]
pub(crate) struct MaybeOwnedArc<T>(
    #[cfg(feature = "typesize")] typesize::ptr::SizableArc<T, typesize::ptr::Owned>,
    #[cfg(not(feature = "typesize"))] Arc<T>,
);

#[cfg(feature = "temp_cache")]
impl<T> MaybeOwnedArc<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(inner).into())
    }

    pub(crate) fn get_inner(self) -> Arc<T> {
        #[cfg(feature = "typesize")]
        let inner = self.0.0;
        #[cfg(not(feature = "typesize"))]
        let inner = self.0;

        inner
    }
}

#[cfg(all(feature = "typesize", feature = "temp_cache"))]
impl<T: typesize::TypeSize> typesize::TypeSize for MaybeOwnedArc<T> {
    fn extra_size(&self) -> usize {
        self.0.extra_size()
    }

    typesize::if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            self.0.get_collection_item_count()
        }
    }
}

#[cfg(feature = "temp_cache")]
impl<T> std::ops::Deref for MaybeOwnedArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "temp_cache")]
impl<T> Clone for MaybeOwnedArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone().into())
    }
}
