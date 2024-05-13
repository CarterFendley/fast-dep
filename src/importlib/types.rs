use pyo3::prelude::*;
use pyo3::types::{PyDict};

#[derive(FromPyObject)]
#[derive(Clone)]
pub struct ModuleSpec {
    pub name: String,
    pub origin: Option<String>,
    pub parent: String,
    pub submodule_search_locations: Option<Vec<String>>
}

impl ModuleSpec {
    pub fn is_package(&self) -> bool {
        match self.submodule_search_locations {
            Some(_) => true,
            None => false
        }
    }
}

impl IntoPy<PyObject> for ModuleSpec {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);

        dict.set_item("name", self.name).unwrap();
        dict.set_item("origin", self.origin).unwrap();

        dict.into()
    }
}

