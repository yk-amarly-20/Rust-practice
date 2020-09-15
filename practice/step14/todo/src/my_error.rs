// エラー
use thiserror::Error;
use actix_web::ResponseError;
use askama;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error)
}

impl ResponseError for MyError {}
