use pyo3::prelude::*;

use super::types::*;

pub fn find_spec(name: &str) -> PyResult<ModuleSpec> {
    Python::with_gil(|py| {
        let importlib_util = PyModule::import(py, "importlib.util")?;

        let spec: ModuleSpec = importlib_util
            .getattr("find_spec")?
            .call1((name, ))?
            .extract()?;

        Ok(spec)
    })
}

pub fn resolve_name(name: &str, package: &str, level: &str) {
    
}