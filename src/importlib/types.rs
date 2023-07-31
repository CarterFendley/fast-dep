use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct ModuleSpec {
    pub name: String,
    pub origin: Option<String>,
}

