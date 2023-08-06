use pyo3::prelude::*;

use super::types::*;

pub fn find_spec(name: &str) -> Option<ModuleSpec> {
    let result = Python::with_gil(|py| -> PyResult<Option<ModuleSpec>> {
        let importlib_util = PyModule::import(py, "importlib.util")?;

        let spec: Option<ModuleSpec> = importlib_util
            .getattr("find_spec")?
            .call1((name, ))?
            .extract()?;

        return Ok(spec)
    });

    if let Err(_) = result {
        // Python::with_gil(|py| {
        //     err.print(py);
        // });
        // panic!("Error while calling `find_spec`");
        return None
    };

    return result.unwrap()
}

// Based on the following implementation:
// https://github.com/python/cpython/blob/v3.9.0/Lib/importlib/_bootstrap.py#L883
pub fn resolve_name(name: &String, package: &String, level: &usize) -> String {
    let bits: Vec<&str> = package.split('.').collect();

    if *level == 0 {
        panic!("This method has no meaning when level == 0");
    }

    // When level == 1 (".") no modification
    // When level == 2 ("..") strip one level of the package name
    // ....
    let include = bits.len().checked_sub(level - 1);
    if include.is_none() {
        panic!("Attempted relative import beyond top-level package");
    }

    if name == ""{
        bits[..include.unwrap()].join(".")
    } else {
        format!("{}.{}", bits[..include.unwrap()].join("."), name)
    }
}