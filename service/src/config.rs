pub trait ServerConfig {
    

    /// Fill empty configuration fields with the values read from database.
    /// 
    /// This function requires `self.storage` field is filled beforehand.
    pub async fn with_database<T>(mut self, migrator: PhantomData<T>) -> Result<ParsedConfig, ConfigError>
    where T: MigratorTrait
    {
        let connection =  self.to_storage_config()?.to_database_connection(migrator).await?;
        let mut p2p_config = PartialP2pConfig::from(P2pConfig::from(P2pConfigModel::get_or_try_init(&connection).await?));
        if let Some(x) = self.p2p {
            (p2p_config.merge(x));
        } 
        self.p2p = Some(p2p_config);
        Ok(self)
    }
}