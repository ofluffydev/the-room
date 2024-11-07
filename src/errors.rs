use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, Error, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Debug, Display, Error, From)]
pub enum CustomErrors {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
    DatabaseError
}

impl ResponseError for CustomErrors {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomErrors::NotFound => HttpResponse::NotFound().finish(),
            CustomErrors::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}