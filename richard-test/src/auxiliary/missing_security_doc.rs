// aux-build:frame-support.rs
#![warn(
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::todo,
    clippy::unwrap_used,
    clippy::panic
)]


use crate::auxiliary::frame_support::{
    storage::{IterableStorageDoubleMap, IterableStorageMap, StorageDoubleMap, StorageMap as Bar},
    Identity, Twox64Concat,
};

/// # Security
///
/// Twox64Concat is allowed because this is a test
#[allow(bare_trait_objects)]
pub type Foo<K, V> = dyn Bar<Twox64Concat, K, V, Query = ()>;

#[allow(bare_trait_objects)]
pub type Foo2<K, V> = dyn Bar<Twox64Concat, K, V, Query = ()>;

fn main() {}
