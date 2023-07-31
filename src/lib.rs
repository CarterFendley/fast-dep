use pyo3::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

mod types;
use types::*;

// TODO: Do tests actually need this?
pub mod minimal_parser;
pub use minimal_parser::*;

pub fn find_spec(name: &str) -> PyResult<types::ModuleSpec> {
    Python::with_gil(|py| {
        let importlib_util = PyModule::import(py, "importlib.util")?;

        let spec: types::ModuleSpec = importlib_util
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

    let imports = parse(&contents);
    dump_imports(&imports);

    // if let Some(caps) = re.captures(&contents) {
    //     for cap in caps.iter() {
    //         println!("Name: {}", &cap["name"])
    //     }
    // }

    // Python::with_gil(|py| -> PyResult<()>  {
    //     let module_ast = PyModule::import(py, "ast")?;

    //     let file: PrintIt = module_ast
    //         .getattr("parse")?
    //         .call1((&contents, ))?
    //         .extract()?;
    //     Ok(())
    // })?;

    // Python::with_gil(|py| -> PyResult<()> {
    //     let importlib_util = PyModule::import(py, "importlib.util")?;

    //     let spec: ModuleSpec = importlib_util
    //         .getattr("find_spec")?
    //         .call1(("os.path",))?
    //         .extract()?;

    //     print!("{}", spec.origin);

    //     Ok(())
    // })?;

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
