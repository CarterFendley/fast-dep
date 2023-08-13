use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, Ref};
use std::ops::Deref;
use log::{debug, info};

use pyo3::prelude::*;

use crate::importlib;

#[pyclass]
#[derive(Clone)]
pub struct DepNode {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub spec: importlib::ModuleSpec,
    // The dependencies & dependents by spec.name
    dependencies: HashSet<String>,
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
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
            depth: depth // Allow for uninitialized depths
        }
    }

    fn merge(&mut self, other: DepNode) {
        // Few sanity checks
        assert!(self.name == other.name);
        if let Some(origin) = &self.spec.origin {
            assert!(origin == &other.spec.origin.unwrap());
        }

        // Merge data
        self.dependencies.extend(other.dependencies);
        self.dependents.extend(other.dependents);
        if other.depth < self.depth {
            self.depth = other.depth
        }
    }

    #[allow(dead_code)]
    fn is_root(&self) -> bool {
        self.dependencies.len() == 0
    }
}

#[pymethods]
impl DepNode {
    #[getter]
    fn dependencies(&self) -> PyResult<usize> {
        println!("{}", self.dependencies.len());
        Ok(self.dependencies.len())
    }
}

#[pyclass]
#[derive(Clone)]
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

        let mut on_node = self.nodes.get(on).unwrap().borrow_mut();
        on_node.dependents.insert(from.to_string());

        let mut from_node = self.nodes.get(from).unwrap().borrow_mut();
        from_node.dependencies.insert(on.to_string());

        // Update depth relative to terminal node
        assert!(!from_node.depth.is_none(), "Attempted to add dependency from node with uninitialized depth named: {}", from_node.name);

        let current_depth = from_node.depth.unwrap() + 1 ;
        if let Some(depth) = on_node.depth {
            if depth > current_depth {
                debug!("Found shorter depth to '{}' new depth is {}", on_node.name, current_depth);
                on_node.depth = Some(current_depth);
            }
        } else {
            debug!("Initializing depth of node '{}' to {}", on_node.name, current_depth);
            // If uninitialized, initialize 
            on_node.depth = Some(current_depth);
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

    pub fn add_graph(&mut self, from: &str, on: &str, graph: DepGraph) {
        debug!(
            "Adding graph with {} nodes linked by {} -> {}",
            graph.nodes.len(),
            from,
            on
        );
        assert!(
            self.nodes.contains_key(from),
            "Node does not exist on graph: {}", from
        );
        assert!(
            graph.nodes.contains_key(on),
            "Node does not exist on graph: {}", on
        );

        // Uninitialize depth from previous graph
        for (_, node_cell) in &graph.nodes {
            let mut node = node_cell.borrow_mut();
            node.depth = None;
        }

        // TODO: Really should be a better way than cloning here
        let from_node = self.nodes.get(from).unwrap().borrow().clone();
        let on_node = graph.nodes.get(on).unwrap().borrow().clone();

        // Construct the depth from the 
        let mut current_depth = from_node.depth.unwrap();
        let mut to_process = vec![on_node.name.clone()];
        while let Some(name) = to_process.pop() {
            let mut node = graph.nodes.get(&name).unwrap().borrow_mut();

            // If node depth is already set it has been set by a previous path
            if node.depth.is_none() {
                node.depth = Some(current_depth);

                // Make sure we process the dependencies of this node
                to_process.extend(node.dependencies.iter().cloned());
                current_depth += 1;
            }
        }

        // Finally merge graphs and add dependency on proper nodes
        self.merge(graph);
        self.add_dependency(from, on); // On will now be a part of the graph
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

    pub fn clone_from(&self, name: &str) -> DepGraph {
        assert!(self.has_node(name));
        let mut clone = DepGraph::new();

        let mut to_clone = vec![name.to_string()];
        while let Some(name) = to_clone.pop() {
            let node = self.nodes.get(&name).unwrap().borrow();
            clone.add(node.clone());

            // Mark all dependencies which are a not yet in the cloned graph as needed to clone
            for dep in &node.dependencies {
                if !clone.has_node(&dep) {
                    to_clone.push(dep.clone());
                }
            }
        }

        // We now have a subset of nodes, remove any node `dependents` which were not copied over
        for (_, node_cell) in &clone.nodes {
            let mut node = node_cell.borrow_mut();

            // Remove non existent dependents
            // TODO: Docs show |&dep| like signature, but that failed?
            node.dependents.retain(|dep| clone.has_node(dep.as_str()));
        }

        return clone
    }

    pub fn merge(&mut self, other: DepGraph) {
        for (name, node_cell) in other.nodes {
            if self.nodes.contains_key(&name) {
                // Just update the data
                let mut existing = self.nodes.get(&name).unwrap().borrow_mut();
                existing.merge(node_cell.into_inner());
            } else {
                // Add the whole node
                self.add(node_cell.into_inner());
            }
        }
    }
}


#[pymethods]
impl DepGraph {
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_dependencies(&self) -> usize {
        let mut acc = 0;
        for (_, node_cell) in &self.nodes {
            let node = node_cell.borrow();
            info!("{}", node.name);
            info!("{}", node.dependencies.len());
            acc += node_cell.borrow().dependencies.len();
        }

        return acc
    }

    pub fn keys(&self) -> HashSet<String> {
        // TODO: Probably expensive, faster to write a conversion for graph.keys() and Keys type?
        self.nodes.iter().map(|(key, _)| key.to_string()).collect()
    }

    pub fn get(&self, name: &str) -> PyResult<DepNode> {
        assert!(
            self.nodes.contains_key(name),
            "Node does not exist on graph: {}", name
        );
        let node = self.nodes.get(name).unwrap().borrow();

        return Ok( node.clone() )
    }

    pub fn get_all_scoped(&self, scope: &str) -> PyResult<Vec<DepNode>> {
        let mut nodes = vec![];

        for (_, node_cell) in &self.nodes {
            let node = node_cell.borrow();
            if node.name.starts_with(scope) {
                nodes.push(node.clone());
            }
        }

        return Ok(nodes)
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
