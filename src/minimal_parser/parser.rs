use pyo3::prelude::*;

use regex::Regex;

use super::types::*;

#[pyfunction]
pub fn parse(source: &str) -> Vec<ImportStmt> {
    let mut stmts: Vec<ImportStmt> = Vec::new();

    // Notes:
    // - (?m) - Enables multi-line mode
    // - ([\s&&[^\n]][^\n]*)?$ Anything else separates by a whitespace char whic is not a new line. Very much complicated by two things:
    //      - \s can match new lines 
    //      - $ can match on a line different than ^
    let re_import = Regex::new(r"(?m)^[\s]*import\s+(?<name>[\S&&[^#]]+)([\s&&[^\n]][^\n]*)?$").unwrap();
    // Should only be applied after re_import
    let re_import_as = Regex::new(r"as\s(?<asname>[\S&&[^#]]+)").unwrap();
    for cap in re_import.captures_iter(&source) {
        let text = cap.get(0).unwrap().as_str();

        let alias = if let Some(as_cap) = re_import_as.captures(&text) {
            Alias {
                name: cap["name"].to_string(),
                asname: Some(as_cap["asname"].to_string())
            }
        } else {
            Alias {
                name: cap["name"].to_string(),
                asname: None
            }
        };

        stmts.push(ImportStmt::Import { names: vec![alias] })
    }


    let re_from = Regex::new(r"(?m)^[\s]*from\s+(?<from>[\S&&[^#]]+)\s+import\s+(?<import>[\S&&[^#]]+)([\s&&[^\n]][^\n]*)?$").unwrap();
    // Note: [\S&&[^#\.][\S&&[^#]*
    // - One visible char which is not # or . then any number of chars which is visible and not # (need to be able to parse `os.path` like stuff where there is a dot in the middle of the module name)
    let re_dots_module = Regex::new(r"^(?<dots>\.*)(?<module>[\S&&[^#\.]][\S&&[^#]]*)$").unwrap();
    for cap in re_from.captures_iter(&source) {
        if let Some(dots_module) = re_dots_module.captures(&cap["from"]) {
            let alias = Alias {
                name: cap["import"].to_string(),
                asname: None
            };

            let import_from = ImportStmt::ImportFrom {
                module: Some(dots_module["module"].to_string()),
                names: vec![alias],
                level: Some(dots_module["dots"].len())
            };

            stmts.push(import_from);
        } else {
            panic!("Unable to parse module section in statement: `{}`", &cap["from"]);
        }
    };

    return stmts
}