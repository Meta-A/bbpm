use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Could not configure blockchain connection properly")]
    ConnectionConfig,
    #[error("Could not establish connection to blockchain")]
    ConnectionFailure,
    #[error("No packages data")]
    NoPackagesData,
}