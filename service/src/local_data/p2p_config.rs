use caretta_sync_core::{config::{P2pConfig, StorageConfig}, context::{ServiceContext, ServiceContextExt}};
use iroh::{PublicKey, SecretKey};
use redb::{Database, ReadTransaction, ReadableDatabase, TableDefinition, TransactionError};

const LOCAL_P2P_CONFIG_TABLE: TableDefinition<(), LocalP2pConfig> = TableDefinition::new("p2p_config");

type LocalP2pConfig = P2pConfig;


/// Extension trait for [`LocalP2pConfig`] to access database
pub trait LocalP2pConfigExt: Sized {
    fn get_from_db(db: &Database) -> Option<Self>;
    fn get_or_init_db(db: &Database) -> Self;
    fn get<T>(context: &T) -> Option<Self>
    where 
        T: ServiceContextExt {
            Self::get_from_db(context.as_local_database())
        }
    fn get_or_init<T>(context: &T) -> Self
    where 
        T: ServiceContextExt {
            Self::get_or_init_db(context.as_local_database())
        }        
} 

impl LocalP2pConfigExt for LocalP2pConfig {
    fn get_from_db(db: &Database) -> Option<Self> {
        let read_txn = db.begin_read().expect("Failed to get transaction error");
        match read_txn.open_table(LOCAL_P2P_CONFIG_TABLE) {
            Ok(table) => {
                table.get(()).expect("Failed to get p2p_config").map(|x| {
                    x.value()
                })
            },
            Err(redb::TableError::TableDoesNotExist(_)) => None,
            Err(x) => panic!("{}", x)
        }
    }
    fn get_or_init_db(db: &Database) -> Self {
        if let Some(x) = Self::get_from_db(db){
            x
        } else {
            let write_txn = db.begin_write().expect("Failed to start write transaction");
            let mut table = write_txn.open_table(LOCAL_P2P_CONFIG_TABLE).expect("Failed to open p2p_config table");
            if let Some(x) = table.get_mut(()).expect("Failed to get p2p_config record").map(|x| x.value()) {
                drop(table);
                write_txn.abort().unwrap();
                x
            } else {
                let _ = table.insert((), LocalP2pConfig {
                    enabled: true,
                    enable_mdns: true,
                    enable_n0: true,
                    secret_key: SecretKey::generate(&mut rand::rng())
                }).expect("Failed to write");
                drop(table);
                write_txn.commit().expect("Failed to commit");
                Self::get_from_db(db).unwrap()
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use iroh::SecretKey;
    use rand::Rng;

    use super::*;
    #[tokio::test]
    async fn get_or_try_init() {
        let context = crate::tests::service_conext().await;
        assert_eq!(LocalP2pConfig::get_or_init(context), LocalP2pConfig::get_or_init(context));
    
    }
}
