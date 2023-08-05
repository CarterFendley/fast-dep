use std::collections::HashSet;
use std::mem;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use pyo3::prelude::*;

use crate::importlib;
use crate::minimal_parser::*;
use super::types::*;

#[pyclass]
pub struct GraphBuilder {
    pub graph: DepGraph,
    processing: HashSet<String>
}

#[pymethods]
impl GraphBuilder {
    #[new]
    pub fn new() -> Self {
        let builder = GraphBuilder {
            graph: DepGraph::new(),
            processing: HashSet::new()
        };

        builder
    }

    pub fn build(&mut self, source: &str) -> DepGraph {
        // Manually build spec / DepNode for first call
        let name = "<terminal>".to_string();
        let spec = importlib::ModuleSpec {
            name: name.clone(),
            origin: None,
        };


        let node = DepNode::new(spec);
        self.graph.add(node);
        self._process_imports(&name, source);

        mem::replace(&mut self.processing, HashSet::new());
        mem::replace(&mut self.graph, DepGraph::new())
    }
}

impl GraphBuilder {
    pub fn _load_source(&mut self, node: &DepNode) -> Option<String> {
        if let Some(path_str) = &node.spec.origin {
            // Some origins we would be able to parse
            if path_str == "built-in"  || path_str == "frozen" {
                return None
            }

            let source_path = Path::new(path_str);
            // Only load python files
            if let Some(ext) = source_path.extension() {
                if ext != "py" {
                    return None
                }
            } else {
                panic!("Unable to load extension for path: {}", path_str);
            }

            println!("Loading file: {}", path_str);
            let mut source_file = File::open(Path::new(source_path)).unwrap();
            let mut source = String::new();

            source_file.read_to_string(&mut source).unwrap();
            return Some(source)
        }

        return None
    }

    pub fn _process_imports(&mut self, name: &String, source: &str) {
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!("Expanding '{}'", name);
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        // Circular check
        assert!(!self.processing.contains(name), "Double processing detected for name: '{}'", name);
        self.processing.insert(name.clone());

        let stmts = parse(source);
        dump_imports(&stmts);

        for stmt in stmts {
            match stmt {
                ImportStmt::Import { names } => {
                    for alias in names {
                        // Don't care about asname, we only need the import name to analyze dependencies
                        self._process_dependency(Some(name), &alias.name);
                    }
                },
                ImportStmt::ImportFrom { module, names, level } => {

                }
            }
        }
        self.processing.remove(name);
        println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
        println!("Done '{}'", name);
        println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
    }

    pub fn _process_dependency(&mut self, from: Option<&String>, name: &str) {
        // Maybe expensive but some values will change names after find_spec()
        // TODO: Deal with this in another way?
        let spec: PyResult<importlib::ModuleSpec> = importlib::find_spec(name);
    
        if let Err(_) = spec {
            println!("!!!! Unable to find spec for name: '{}' !!!!", name);
            return
        }

        // Rebind spec & name to make things easier going forward
        let spec = spec.unwrap();
        let name = spec.name.clone();

        // Double borrow in add_dependency will fail, self references are valid python, just going to not track them for now.
        if let Some(from) = from {
            if from == &name {
                return
            }
        }

        if let Some(from) = from {
            println!("Processing dependency: {} -> {}", from, name);
        } else {
            println!("Processing dependency on: {}", name);
        }


        if self.graph.has_node(&name) {
            // Just need to update the dependencies if required
            if let Some(from) = from {
                self.graph.add_dependency(from, &name)
            }

            // Done!
            return
        }

        // Process parent before anything else
        let names: Vec<&str> = name.split(".").collect();
        let parent = names[..names.len() - 1].join(".");
        if parent.len() != 0 {
            self._process_dependency(from, parent.as_str());
        }

        // During processing of the parent, the current name may be added to the graph
        if self.graph.has_node(&name) {
            // Same as above, only update dependencies
            if let Some(from) = from {
                self.graph.add_dependency(&from, &name)
            }

            // Done!
            return
        }

        // At this point we must add the node ourselves
        let new_node = DepNode::new(spec);
        let source = self._load_source(&new_node);
        self.graph.add(new_node);

        if let Some(source) = source {
            self._process_imports(&name.to_string(), source.as_str())
        }

        // Finish by putting on graph and updating marking
        if let Some(from) = from {
            self.graph.add_dependency(
                &from,
                &name
            );
        }

        // Done!
    }
}