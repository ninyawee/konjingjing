//! Data modules for Thai National ID meaning extraction.

pub mod amphoes;
pub mod person_types;
pub mod provinces;

pub use amphoes::{get_amphoe, Amphoe};
pub use person_types::{get_person_type, PersonType};
pub use provinces::{get_province, Province};
