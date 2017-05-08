#![allow(unused)]

pub mod core;
pub mod raw_mutex;

pub use self::core as parking_lot_core;
pub use self::raw_mutex::RawMutex;
