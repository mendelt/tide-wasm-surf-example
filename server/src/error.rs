use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    ServerError(async_std::io::Error),
    ConfigError { setting: String },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(err) => write!(formatter, "Error accessing database; {}", err),
            Error::ServerError(err) => write!(formatter, "Error starting server; {}", err),
            Error::ConfigError { setting } => write!(
                formatter,
                "Error reading configuration, no environment variable found with name {}",
                setting
            ),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl From<async_std::io::Error> for Error {
    fn from(err: async_std::io::Error) -> Self {
        Error::ServerError(err)
    }
}
