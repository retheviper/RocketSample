#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// use memory_db::{MemoryDB, HashKey};
// use keccak_hasher::KeccakHasher;

mod handler;

fn main() {
    // let mut memory_db = MemoryDB::<KeccakHasher, HashKey<_ >, Vec<u8> >::default ();

    rocket::ignite().mount("/", routes![
        handler::member_handler::get_member,
        handler::member_handler::list_member,
    ]).launch();
}