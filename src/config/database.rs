use serde::Deserialize;
use crate::config::ValueOrPath;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub address: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigFileDatabaseTable {
    pub name: ValueOrPath,
    pub username: ValueOrPath,
    pub password: ValueOrPath,
    pub address: ValueOrPath
}

impl TryFrom<ConfigFileDatabaseTable> for DatabaseConfig {
    type Error = anyhow::Error;

    fn try_from(value: ConfigFileDatabaseTable) -> Result<Self, Self::Error> {
        let name = value.name.try_convert_to_value()?;
        let username = value.username.try_convert_to_value()?;
        let password = value.password.try_convert_to_value()?;
        let address = value.address.try_convert_to_value()?;

        Ok(DatabaseConfig{ name, username, password , address })
    }
}