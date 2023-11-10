#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod core;
mod adapter;

fn main() {
    adapter::api::server::main();
}
