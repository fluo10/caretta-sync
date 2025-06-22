use crate::{config::{P2pConfig, StorageConfig}, error::Error, global::GlobalConstant};

pub static STORAGE_CONFIG: GlobalConstant<StorageConfig> = GlobalConstant::const_new(stringify!(STORAGE_CONFIG));
pub static P2P_CONFIG: GlobalConstant<P2pConfig> = GlobalConstant::const_new(stringify!(P2P_CONFIG));

#[cfg(test)]
mod tests {
    use crate::global::{config::P2P_CONFIG, STORAGE_CONFIG};

    #[test]
    fn test_global_constant_names() {
        assert_eq!(STORAGE_CONFIG.name, stringify!(STORAGE_CONFIG));
        assert_eq!(P2P_CONFIG.name, stringify!(P2P_CONFIG));
    }
}