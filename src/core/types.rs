use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, Ref};
use std::ops::Deref;
use log::{debug};

use pyo3::prelude::*;

use crate::importlib;

#[pyclass]
#[derive(Clone)]
pub struct DepNode {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub spec: importlib::ModuleSpec,
    #[pyo3(get)]
    dependencies: i32,
    // The dependencies by spec.name
    #[pyo3(get)]
    dependents: HashSet<String>,
    #[pyo3(get)]
    depth: Option<i32>
}

impl DepNode {
    pub fn new(spec: importlib::ModuleSpec, depth: Option<i32>) -> DepNode {
        DepNode {
            name: spec.name.clone(),
            spec: spec,
            dependencies: 0,
            dependents: HashSet::new(),
            depth: depth // Allow for uninitialized depths
        }
    }

    #[allow(dead_code)]
    fn is_root(&self) -> bool {
        self.dependencies == 0
    }
}

// impl IntoPy<PyObject> for DepNode {
//     fn into_py(self, py: Python<'_>) -> PyObject {
//         let dict = PyDict::new(py);

//         dict.set_item("name", self.name).unwrap();
//         dict.set_item("spec", self.spec).unwrap();

//         let dependents = PyList::new(py);
//         for dependent in self.dependents {
//             dependents.append(dependent).unwrap();
//         }
//         dict.set_item("dependents", dependents).unwrap();
//         dict.set_item("dependencies", self.dependencies).unwrap();

//         dict.into()
//     }
// }

#[pyclass]
pub struct DepGraph {
    pub nodes: HashMap<String, RefCell<DepNode>>,
    root_nodes: HashSet<String>

}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            nodes: HashMap::new(),
            root_nodes: HashSet::new(),
        }
    }

    pub fn add_dependency(&self, from: &str, on: &str) {
        // This method will take a node which is currently under construction and updated it's dependencies.
        // 
        // **NOTE:** It is imperative that the `from` which is taken in and returned from this method is added to the graph at some point.
        debug!("Add dependency '{}' -> '{}'", from, on);

        // Make sure we have the `on` node
        assert!(
            self.nodes.contains_key(from),
            "Node does not exist on graph: {}", from
        );
        assert!(
            self.nodes.contains_key(on),
            "Node does not exist on graph: {}", on
        );

        let mut on = self.nodes.get(on).unwrap().borrow_mut();
        on.dependents.insert(from.to_string());

        let mut from = self.nodes.get(from).unwrap().borrow_mut();
        from.dependencies += 1;

        // Update depth relative to terminal node
        assert!(!from.depth.is_none(), "Attempted to add dependency from node with uninitialized depth named: {}", from.name);

        let current_depth = from.depth.unwrap() + 1 ;
        if let Some(depth) = on.depth {
            if depth > current_depth {
                debug!("Found shorter depth to '{}' new depth is {}", on.name, current_depth);
                on.depth = Some(current_depth);
            }
        } else {
            debug!("Initializing depth of node '{}' to {}", on.name, current_depth);
            // If uninitialized, initialize 
            on.depth = Some(current_depth);
        }
    }

    pub fn add(&mut self, node: DepNode) -> Ref<DepNode> {
        assert!(!self.nodes.contains_key(&node.name));
        debug!("Adding node to graph: {}", node.name);

        let name = node.name.clone(); // TODO: Better way?

        self.nodes.insert(
            name.clone(),
            RefCell::new(node)
        );
        self.root_nodes.insert(name.clone());

        self.nodes.get(&name).unwrap().borrow()
    }

    // TODO: Read up on the `where` syntax
    pub fn with<F>(self, name: &str, f: F) where F: Fn(&DepNode) {
        let node = self.nodes.get(name).unwrap().borrow();
        // TODO: Not sure I understand the deref part
        f(node.deref())
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

    pub fn get(&self, name: &str) -> PyResult<DepNode> {
        let node = self.nodes.get(name).unwrap().borrow();

        return Ok( node.clone() )
        //Ok( node.into_py(py) )
    }

    pub fn origins(&mut self) -> Vec<String> {
        let mut origins = vec![];

        for (_, node_cell) in &self.nodes {
            let node = node_cell.borrow_mut();
            if let Some(origin) = &node.spec.origin {
                origins.push(origin.clone());
            }
        }

        return origins
    }

    pub fn names(&mut self) -> Vec<String> {
        let mut names = vec![];

        for (_, node_cell) in &self.nodes {
            let node = node_cell.borrow_mut();
            names.push(node.name.clone());
        }

        return names
    }
}
