#![feature(generic_associated_types)]

#[macro_use]
extern crate diesel;

pub mod db_backend;
mod geo_bbox;
mod keywords;
pub use geo_bbox::GeoBBox2D;
pub use keywords::Keywords;

#[cfg(feature = "test-utils")]
pub mod db_tester;
