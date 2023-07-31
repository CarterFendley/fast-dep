pub struct Alias {
    pub name: String,
    pub asname: Option<String>
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
