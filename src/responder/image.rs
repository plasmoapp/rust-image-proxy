use rocket::Response;
use rocket::request::Request;
use rocket::response::{self, Responder};

use std::io::Cursor;

pub struct Image {
    pub bytes: Vec<u8>,
    pub content_type: String,
}

impl<'r> Responder<'r, 'static> for Image {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .streamed_body(Cursor::new( self.bytes))
            .raw_header("content-type", self.content_type)
            .ok()
    }
}