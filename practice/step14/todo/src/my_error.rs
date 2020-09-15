// エラー
use thiserror::Error;
use actix_web::ResponseError;

#[derive(Error, Debug)]
pub enum MyError {}

impl ResponseError for MyError {}
