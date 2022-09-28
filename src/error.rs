use thiserror::Error;

use rocket::Response;
use rocket::request::Request;
use rocket::response::{self, Responder};

use std::io::Cursor;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP Error {source:?}")]
    Reqwest {
        #[from] source: reqwest::Error,
    },
    #[error("Failed to parse URL {source:?}")]
    Url {
        #[from] source: url::ParseError,
    },
    #[error("VarError {source:?}")]
    VarError {
        #[from] source: std::env::VarError,
    },
    #[error("Wrong content-type")]
    ContentType,
    #[error("Header is is not a string {source:?}")]
    ToStr {
        #[from] source: reqwest::header::ToStrError,
    },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {

        dbg!(self);

        let bytes = std::fs::read("default.png").unwrap();    

        Response::build()
            .streamed_body(Cursor::new( bytes))
            .raw_header("content-type", "image/png")
            .ok()
    }
}