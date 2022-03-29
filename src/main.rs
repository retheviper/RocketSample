#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod handler;

fn main() {
    rocket::ignite().mount("/", routes![handler::member_handler::get_member]).launch();
}