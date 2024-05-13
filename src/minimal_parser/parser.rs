use pyo3::prelude::*;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use super::types::*;

#[derive(Parser)]
#[grammar = "minimal_parser/grammar.pest"]
struct PESTParser;


#[pyfunction]
pub fn parse(source: &str) -> Vec<ImportStmt> {
    let pairs = PESTParser::parse(Rule::python, source).expect("Could not parse file");

    let mut stmts: Vec<ImportStmt> = Vec::new();
    for pair in pairs.flatten() {
        match pair.as_rule() {
            Rule::import => {
                // Pull the first inner value and validate it is a alias_list
                let alias_list = pair.into_inner().next().unwrap();
                assert_eq!(Rule::alias_list, alias_list.as_rule());


                let mut names: Vec<Alias> = Vec::new();
                for alias in alias_list.into_inner() {
                    names.push(parse_alias(alias))
                }

                stmts.push(ImportStmt::Import { names: names });
            },
            Rule::import_from => {
                let mut inner = pair.into_inner();

                let module_spec = inner.next().unwrap();
                let (level, module) = parse_module_spec(module_spec);

                let from_alias_list = inner.next().unwrap();
                assert_eq!(Rule::from_alias_list, from_alias_list.as_rule());

                let mut names: Vec<Alias> = Vec::new();
                let list_inner = from_alias_list.into_inner();
                for list_element in list_inner {
                    match list_element.as_rule() {
                        Rule::alias => names.push(parse_alias(list_element)),
                        Rule::additional => {
                            // Many fields in additional, just pull out aliases
                            for additional_element in list_element.into_inner() {
                                match additional_element.as_rule() {
                                    Rule::alias => names.push(parse_alias(additional_element)),
                                    _ => ()
                                }
                            }
                        },
                        Rule::alias_list => {
                            for alias in list_element.into_inner() {
                                names.push(parse_alias(alias))
                            }
                        },
                        _ => ()
                    }
                }

                stmts.push( ImportStmt::ImportFrom {
                    module: Some(module),
                    names: names,
                    level: Some(level)
                })
            }
            _ => ()
        }
    }

    return stmts
}

fn parse_alias(alias: Pair<Rule>) -> Alias {
    assert_eq!(Rule::alias, alias.as_rule());

    let mut contents = alias.into_inner();
    // Should always have a name
    let qualified_name = contents.next().unwrap().as_str();
    // May have a "asname"
    let asname = match contents.next() {
        Some(pair) => Some(pair.as_str().to_string()),
        None => None
    };

    Alias {
        name: qualified_name.to_string(),
        asname: asname
    }
}

// TODO: Not 100% sure about the `Rule`
fn parse_module_spec(module_spec: Pair<Rule>) -> (usize, String) {
    assert_eq!(Rule::module_spec, module_spec.as_rule());
    let mut module_spec_inner = module_spec.into_inner();

    let mut level = 0;
    let mut module = "";

    let pair = module_spec_inner.next().unwrap();
    match pair.as_rule() {
        Rule::dots => {
            // Count dots and parse name if present
            level = pair.as_str().len();

            // May also have a module name
            if let Some(pair) = module_spec_inner.next() {
                module = pair.as_str()
            }
        },
        Rule::qualified_name => {
            module = pair.as_str()
        }
        _ => panic!("Unexpected rule while parsing module spec: {:?}", pair)
    }

    return (level, module.to_string())
}


// pub fn parse(source: &str) -> Vec<ImportStmt> {
//     let mut stmts: Vec<ImportStmt> = Vec::new();

//     // Notes:
//     // - (?m) - Enables multi-line mode
//     // - ([\s&&[^\n]][^\n]*)?$ Anything else separates by a whitespace char whic is not a new line. Very much complicated by two things:
//     //      - \s can match new lines 
//     //      - $ can match on a line different than ^
//     let re_import = Regex::new(r"(?m)^[\s]*import\s+(?<name>[\S&&[^#]]+)([\s&&[^\n]][^\n]*)?$").unwrap();
//     // Should only be applied after re_import
//     let re_import_as = Regex::new(r"as\s(?<asname>[\S&&[^#]]+)").unwrap();
//     for cap in re_import.captures_iter(&source) {
//         let text = cap.get(0).unwrap().as_str();

//         let alias = if let Some(as_cap) = re_import_as.captures(&text) {
//             Alias {
//                 name: cap["name"].to_string(),
//                 asname: Some(as_cap["asname"].to_string())
//             }
//         } else {
//             Alias {
//                 name: cap["name"].to_string(),
//                 asname: None
//             }
//         };

//         stmts.push(ImportStmt::Import { names: vec![alias] })
//     }


//     let re_from = Regex::new(r"(?m)^[\s]*from\s+(?<from>[\S&&[^#]]+)\s+import\s+(?<import>[\S&&[^#]]+)([\s&&[^\n]][^\n]*)?$").unwrap();
//     // Note: [\S&&[^#\.][\S&&[^#]*
//     // - One visible char which is not # or . then any number of chars which is visible and not # (need to be able to parse `os.path` like stuff where there is a dot in the middle of the module name)
//     let re_dots_module = Regex::new(r"^(?<dots>\.*)(?<module>[\S&&[^#\.]][\S&&[^#]]*)$").unwrap();
//     for cap in re_from.captures_iter(&source) {
//         if let Some(dots_module) = re_dots_module.captures(&cap["from"]) {
//             let alias = Alias {
//                 name: cap["import"].to_string(),
//                 asname: None
//             };

//             let import_from = ImportStmt::ImportFrom {
//                 module: Some(dots_module["module"].to_string()),
//                 names: vec![alias],
//                 level: Some(dots_module["dots"].len())
//             };

//             stmts.push(import_from);
//         } else {
//             panic!("Unable to parse module section in statement: `{}`", &cap["from"]);
//         }
//     };

//     return stmts
// }