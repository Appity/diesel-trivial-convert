#[macro_use]
extern crate diesel;

pub mod db;

pub mod models;

mod label;
pub use label::Label;
pub use label::LabelError;

pub mod schema;
