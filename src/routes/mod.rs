use std::string::FromUtf8Error;

use fiscalidade::{DfeError, Pkcs12CertificateError};
use rocket::{
    http::Status,
    request::Request,
    response::{self, status, Responder},
};
use thiserror::Error;

pub mod auth;
pub mod cache;
pub mod nfe;
pub mod service_auth;
pub mod services;
pub mod taxpayer;
pub mod taxpayer_service;

use crate::db;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Db(#[from] db::Error),
    #[error(transparent)]
    Pkcs12(#[from] Pkcs12CertificateError),
    #[error(transparent)]
    Dfe(#[from] DfeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        status::Custom(Status::UnprocessableEntity, json_error!(self.to_string())).respond_to(req)
    }
}
