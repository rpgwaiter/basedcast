#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate diesel_derive_enum;

pub mod app;
pub mod db;
pub mod graphql;