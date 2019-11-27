#![feature(core_intrinsics)]

extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;

pub mod server;
pub mod config;
pub mod handler;
pub mod error;
pub mod api;