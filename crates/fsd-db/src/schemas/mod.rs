//! Schema definitions for the other FSN program databases.
//!
//! These modules define the SQL schemas that each program will use when they
//! initialize their own database. The schemas are centralized here so they
//! can be referenced consistently across the codebase.

pub mod conductor;
pub mod store;
pub mod core;
pub mod bus;
