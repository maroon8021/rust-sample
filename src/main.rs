#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::http::RawStr;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/echo")]
fn simple_echo() -> &'static str {
    "You can add /echo/<text>"
}

#[get("/echo/<text>")]
fn echo(text: &RawStr) -> String {
    format!("Hello, world!: {} ", text.as_str())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, echo, simple_echo])
        .launch();
}
