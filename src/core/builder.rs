use std::mem;

use pyo3::prelude::*;

use crate::importlib;
use crate::minimal_parser::*;
use super::types::*;

struct LoadedNode {
    node: DepNode, 

}

#[pyclass]
pub struct GraphBuilder {
    pub graph: DepGraph,
}

#[pymethods]
impl GraphBuilder {
    #[new]
    pub fn new() -> Self {
        let builder = GraphBuilder {
            graph: DepGraph::new(),
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

        mem::replace(&mut self.graph, DepGraph::new())
    }
}

impl GraphBuilder {
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
        let name = spec.name.as_str();

        if self.graph.has_node(&name) {
            // Just need to update the dependencies if required
            if let Some(node) = node {
                self.graph.add_dependent(&name, &node.name)
            }
        }

        // Otherwise, need to build it
    }
}