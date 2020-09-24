#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/echo/<text>")]
fn echo(text: u32) -> String {
    text.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index, echo]).launch();
}
