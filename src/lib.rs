use pyo3::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// TODO: Do tests actually need this?
pub mod minimal_parser;
pub use minimal_parser::*;

mod core;
mod importlib;

#[pymodule]
fn fast_dep(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<core::DepNode>()?;
    m.add_class::<core::DepGraph>()?;
    m.add_class::<core::GraphBuilder>()?;

    let parser_module = PyModule::new(_py, "parser")?;
    parser_module.add_function(wrap_pyfunction!(parse, parser_module)?)?;

    m.add_submodule(parser_module)?;
    Ok(())
}