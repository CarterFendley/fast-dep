use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

use pyo3::prelude::*;

use crate::importlib;

#[pyclass]
pub struct DepNode {
    pub name: String,
    spec: importlib::ModuleSpec,
    dependencies: i32,
    // The dependencies by spec.name
    dependents: HashSet<String>
}

impl DepNode {
    pub fn new(spec: importlib::ModuleSpec) -> DepNode {
        DepNode {
            name: spec.name.clone(),
            spec: spec,
            dependencies: 0,
            dependents: HashSet::new()
        }
    }

    fn is_root(&self) -> bool {
        self.dependencies == 0
    }
}

#[pyclass]
pub struct DepGraph {
    nodes: HashMap<String, RefCell<DepNode>>,
    root_nodes: HashSet<String>

}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            nodes: HashMap::new(),
            root_nodes: HashSet::new(),
        }
    }

    pub fn add_dependent(&self, from: &str, on: &str) {
        // Before performing an operation on either, make sure both exist
        assert!(self.nodes.contains_key(from));
        assert!(self.nodes.contains_key(on));

        let mut on_node = self.nodes.get(on).unwrap().borrow_mut();
        on_node.dependents.insert(from.to_string());

        let mut from_node = self.nodes.get(on).unwrap().borrow_mut();
        from_node.dependencies += 1;
    }

    pub fn add(&mut self, node: DepNode) {
        assert!(!self.nodes.contains_key(&node.name));

        let name = node.name.clone(); // TODO: Better way?

        self.nodes.insert(
            name.clone(),
            RefCell::new(node)
        );
        self.root_nodes.insert(name);
    }

    pub fn has_node(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
    }
}


#[pymethods]
impl DepGraph {
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn keys(&self) -> HashSet<String> {
        // TODO: Probably expensive, faster to write a conversion for graph.keys() and Keys type?
        self.nodes.iter().map(|(key, _)| key.to_string()).collect()
    }
}
