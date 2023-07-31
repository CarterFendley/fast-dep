use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct ModuleSpec {
    name: String,
    origin: Option<String>,
}