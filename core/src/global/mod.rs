use std::{any::type_name, collections::HashMap, net::{IpAddr, Ipv4Addr}, path::{Path, PathBuf}, sync::LazyLock};

use crate::{config::{P2pConfig, PartialP2pConfig, StorageConfig}, error::Error }; 
#[cfg(any(test, feature="test"))]
use crate::tests::{GlobalTestDefault, TestDefault};
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId};
use sea_orm::{prelude::*, Database};
use sea_orm_migration::MigratorTrait;
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};

mod config;
pub use config::*;
mod database_connection;
pub use database_connection::*;
use uuid::{ContextV7, Timestamp, Uuid};

pub fn generate_uuid() -> Uuid {
    Uuid::new_v7(Timestamp::now(ContextV7::new()))
}

pub static PRODUCT_NAME: LazyLock<String> = LazyLock::new(|| {
    env!("CARGO_PKG_NAME").to_string()
});

pub static DEFAULT_LISTEN_IPS: &[IpAddr] = &[IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))];

pub static DEFAULT_CONFIG_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".toml")
});


pub static DEFAULT_DATABASE_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".sqlite")
});
fn uninitialized_message<T>(var: T) -> String {
    format!("{} is uninitialized!", &stringify!(var))
}

pub struct GlobalConstant<T> {
    pub name: &'static str,
    inner: OnceCell<T>
}

impl<T> GlobalConstant<T> { 
    pub const fn const_new(name: &'static str ) -> Self {
        Self{
            name: name,
            inner: OnceCell::const_new()
        }
    }
    pub async fn get_or_init(&'static self, source: T) -> &'static T {
        self.inner.get_or_init(|| async {
            source
        }).await
    }
    pub fn get(&'static self) -> Option<&'static T> {
        self.inner.get()
    }
    pub fn get_and_unwrap(&'static self) -> &'static T {
        self.get().expect(&format!("{} is uninitialized!", self.name))
    }
}

#[cfg(any(test, feature="test"))]
impl<T> GlobalTestDefault<T> for GlobalConstant<T>
where
    T: TestDefault + 'static
{
    async fn get_or_init_test_default(&'static self) -> &'static T {
        self.get_or_init(T::test_default()).await
    }
}

struct GlobalRwLock<T> {
    pub name: &'static str,
    inner: OnceCell<RwLock<T>>
}

impl<T> GlobalRwLock<T> {
    pub const fn const_new(name: &'static str) -> Self {
        Self{
            name: name, 
            inner: OnceCell::const_new()
        }
    }
    pub fn get(&'static self) -> &'static RwLock<T> {
        self.inner.get().expect(&format!("{} is uninitialized", self.name))
    }
    pub async fn write(&'static self) -> RwLockWriteGuard<'_ ,T> {
        self.get().write().await
    }
    pub async fn read(&'static self) -> RwLockReadGuard<'_, T> {
        self.get().read().await
    }
}

#[cfg(test)]
mod tests {

}
