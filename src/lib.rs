use pyo3::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(FromPyObject)]
pub struct ModuleSpec {
    name: String,
    origin: String
}

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

pub fn get_ast(file_path: &str) -> Result<(), Box<dyn Error>>{
    let mut file = File::open(Path::new(file_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Python::with_gil(|py| -> PyResult<()> {
        let importlib_util = PyModule::import(py, "importlib.util")?;

        let spec: ModuleSpec = importlib_util
            .getattr("find_spec")?
            .call1(("os.path",))?
            .extract()?;

        print!("{}", spec.origin);

        Ok(())
    })?;

    print!("{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Later
    }
}
