use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub struct Alias {
    pub name: String,
    pub asname: Option<String>
}

impl IntoPy<PyObject> for Alias {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);

        dict.set_item("name", self.name).unwrap();

        if let Some(asname) = self.asname {
            dict.set_item("asname", asname).unwrap();
        }

        dict.into()
    }
}

pub enum ImportStmt {
    Import {
        names: Vec<Alias>
    },
    ImportFrom {
        module: Option<String>,
        names: Vec<Alias>,
        level: Option<usize>
    },
}

fn alias_vec_to_list(py: Python<'_>, names: Vec<Alias>) -> PyObject {
    let list = PyList::empty(py);

    for name in names {
        list.append(name.into_py(py)).unwrap();
    }

    return list.into()
}

impl IntoPy<PyObject> for ImportStmt {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);

        match self {
            ImportStmt::Import { names } => {
                dict.set_item("type", "import").unwrap();

                if names.len() != 0 {
                    dict.set_item(
                        "names", 
                        alias_vec_to_list(
                            py, 
                            names
                    )).unwrap()
                }
            },
            ImportStmt::ImportFrom { module, names, level } => {
                dict.set_item("type", "import_from").unwrap();

                if let Some(module) = module {
                    dict.set_item("module", module).unwrap();
                }
                if let Some(level) = level {
                    dict.set_item("level", level).unwrap();
                }

                if names.len() != 0 {
                    dict.set_item(
                        "names", 
                        alias_vec_to_list(
                            py, 
                            names
                    )).unwrap()
                }
            }
        }
        dict.into()
    }
}
