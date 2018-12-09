#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

extern crate multipart;

use std::{io, env};
use std::io::{Cursor, Read};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use multipart::server::Multipart;

#[derive(Debug)]
struct http_multipart_form_data {
    string_data: String,
    integer_data: i32,
    file_data: String,
}

#[post("/multipart", data = "<upload>")]
fn multipart(upload: http_multipart_form_data) -> String {
    format!("I read this: {:?}", upload)
}

impl FromData for http_multipart_form_data {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        //error list
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[idx + "boundary=".len())...];

        let mut d = Vec::new();
        data.stream_to(&mut d).expect("Unable to read");

        let mut mp = Multipart::with_body(Cursor::new(d), boundary);

        let mut string_data = None;
        let mut integer_data = None;
        let mut file_data = None;

        
    }
}

#[get("/")]
fn index() -> &'static str {
    "Upload your text files by POSTing them to /upload."
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, multipart])
}

fn main() {
    rocket().launch();
}
