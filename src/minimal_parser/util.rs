use super::types::*;

pub fn dump_alias(alias: &Vec<Alias>, tab_level: usize) {
    let tab_level = "  ".repeat(tab_level);

    println!("{}{{", tab_level);
    for a in alias {
        println!("{}  name: {}", tab_level, a.name);
        if let Some(asname) = &a.asname {
            println!("{}  asname: {}", tab_level, asname);
        }
    }
    println!("{}}}", tab_level);
}

pub fn dump_imports(stmts: &Vec<ImportStmt>) {
    for stmt in stmts {
        match stmt {
            ImportStmt::Import { names } => {
                println!("Import: {{");
                dump_alias(names, 1);
                println!("}}");
            },
            ImportStmt::ImportFrom { module, names, level } => {
                println!("ImportFrom: {{");
                if let Some(level) = level {
                    println!("  level: {}", level)
                }
                if let Some(module) = module {
                    println!("  module: {}", module)
                }
                dump_alias(names, 1);
                println!("}}");
            }
        }
    }
}
