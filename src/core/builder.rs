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
        let spec = importlib::ModuleSpec {
            name: "<terminal>".to_string(),
            origin: None,
        };

        let node = DepNode::new(spec);
        self._process_imports(&node, source);

        mem::replace(&mut self.processing, HashSet::new());
        mem::replace(&mut self.graph, DepGraph::new())
    }
}

impl GraphBuilder {
    pub fn _process_node(&mut self, node: &DepNode) {
        if let Some(source_path) = &node.spec.origin {
            let mut source_file = File::open(Path::new(source_path)).unwrap();
            let mut source = String::new();

            source_file.read_to_string(&mut source).unwrap();
            self._process_imports(node, &source)
        }

        // If no valid source path, do not process the node
    }

    pub fn _process_imports(&mut self, node: &DepNode, source: &str) {
        let stmts = parse(source);
        dump_imports(&stmts);

        for stmt in stmts {
            match stmt {
                ImportStmt::Import { names } => {
                    for alias in names {
                        self._process_dependent(&alias.name, Some(node));
                    }
                },
                ImportStmt::ImportFrom { module, names, level } => {

                }
            }
        }
    }

    pub fn _process_dependent(&mut self, name: &str, node: Option<&DepNode>) {
        // Maybe expensive but some values will change names after find_spec()
        // TODO: Deal with this in another way?
        let spec: PyResult<importlib::ModuleSpec> = importlib::find_spec(name);
    
        if let Err(_) = spec {
            return
        }

        // Rebind spec & name to make things easier going forward
        let spec = spec.unwrap();
        let name = spec.name.clone();

        // Circular 
        assert!(!self.processing.contains(&name));

        if self.graph.has_node(&name) {
            // Just need to update the dependencies if required
            if let Some(node) = node {
                self.graph.add_dependent(&node.name, &name)
            }

            // Done!
            return
        }

        // Process parent before anything else
        let names: Vec<&str> = name.split(".").collect();
        let parent = names[..names.len() - 1].join(".");
        self._process_dependent(&parent.as_str(), node);

        // During processing of the parent, the current name may be added to the graph
        if self.graph.has_node(&name) {
            // Same as above, only update dependencies
            if let Some(node) = node {
                self.graph.add_dependent(&node.name, &name)
            }

            // Done!
            return
        }

        // At this point we must add the node ourselves
        let new_node = DepNode::new(spec);

        self.processing.insert(new_node.name.clone());
        self._process_node(&new_node);
        self.processing.remove(&new_node.name);

        // Finish by putting on graph and updating dependencies
        self.graph.add(new_node);
        if let Some(node) = node {
            self.graph.add_dependent(
                &node.name,
                &name
            );
        }
    }
}