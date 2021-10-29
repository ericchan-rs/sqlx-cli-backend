use anyhow::Error as OpaqueError;
use log::debug;
use repository::SqlxError;
pub fn search_to_domain_error(e: meilisearch_sdk::errors::Error) -> DomainError {
    DomainError::from(OpaqueError::from(e))
}

pub fn database_to_domain_error(e: SqlxError) -> DomainError {
    debug!("database_to_domain_error: {}", e);
    DomainError::from(OpaqueError::from(e))
}

#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DomainError {
    #[from]
    source: anyhow::Error,
}
