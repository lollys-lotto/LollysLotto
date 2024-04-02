pub mod data_types;
pub mod query_types;

use crate::{Result, SSLv2DatabaseError};
use log::info;
use sqlx::{
    postgres::{PgArguments, PgConnectOptions, PgQueryResult, PgRow, PgSslMode},
    query::{Query, QueryAs},
    FromRow, Pool, Postgres,
};
use std::{path::Path, str::FromStr};

pub use data_types::*;
pub use query_types::*;

pub type TypedQuery<T> = QueryAs<'static, Postgres, T, PgArguments>;
pub type UntypedQuery = Query<'static, Postgres, PgArguments>;

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: &str, ssl_cert: Option<&str>) -> Result<Self> {
        info!("Attempting to connect to Postgres DB");
        let pool = if let Some(ssl_cert) = ssl_cert {
            let options = PgConnectOptions::from_str(url)
                .map_err(|e| {
                    SSLv2DatabaseError::InvalidDatabaseUrl(url.to_string(), e.to_string())
                })?
                .ssl_root_cert(Path::new(ssl_cert))
                .ssl_mode(PgSslMode::Require);
            Pool::<Postgres>::connect_with(options).await
        } else {
            Pool::<Postgres>::connect(url).await
        }
        .map(|pool: Pool<Postgres>| {
            info!("Successfully connected to Postgres DB");
            pool
        })
        .map_err(|e| SSLv2DatabaseError::DatabaseConnectionError(e))?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }

    pub async fn fetch_one<T, Raw: Into<T> + Send + Unpin + for<'r> FromRow<'r, PgRow>>(
        &self,
        query: TypedQuery<Raw>,
    ) -> Result<T> {
        query
            .fetch_one(&self.pool())
            .await
            .map(|row_raw| row_raw.into())
            .map_err(|e| e.into())
    }

    pub async fn fetch_optional<T, Raw: Into<T> + Send + Unpin + for<'r> FromRow<'r, PgRow>>(
        &self,
        query: TypedQuery<Raw>,
    ) -> Result<Option<T>> {
        query
            .fetch_optional(&self.pool())
            .await
            .map(|row_raw| row_raw.map(|r| r.into()))
            .map_err(|e| e.into())
    }

    pub async fn fetch_all<T, Raw: Into<T> + Send + Unpin + for<'r> FromRow<'r, PgRow>>(
        &self,
        query: TypedQuery<Raw>,
    ) -> Result<Vec<T>> {
        query
            .fetch_all(&self.pool())
            .await
            .map(|rows_raw| rows_raw.into_iter().map(|r| r.into()).collect())
            .map_err(|e| e.into())
    }

    pub async fn execute(&self, query: UntypedQuery) -> Result<PgQueryResult> {
        query.execute(&self.pool()).await.map_err(|e| e.into())
    }
}

/// You can copy this module as a template for the data_types tests
#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::connect_to_test_db;

    #[tokio::test]
    async fn test_db() {
        let _db = connect_to_test_db().await;
    }
}
