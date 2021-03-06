#![allow(unused)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate mysql;

#[macro_use]
mod std_ext;
mod errors;
mod global;
mod models;

mod backend;
mod data;

pub mod server;
