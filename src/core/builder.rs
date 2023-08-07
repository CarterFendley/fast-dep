use std::collections::HashSet;
use std::mem;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use log::{debug};

use pyo3::prelude::*;

use crate::importlib::*;
use crate::minimal_parser::*;
use super::types::*;

#[pyclass]
pub struct GraphBuilder {
    pub graph: DepGraph,
    processing: HashSet<String>,
    verbose: bool
}

#[pymethods]
impl GraphBuilder {
    #[new]
    pub fn new(verbose: Option<bool>) -> Self {
        let verbose = if let Some(verbose) = verbose {
            verbose
        } else {
            false
        };

        let builder = GraphBuilder {
            graph: DepGraph::new(),
            processing: HashSet::new(),
            verbose: verbose
        };

        builder
    }

    pub fn build(&mut self, source: &str, package: Option<String>) -> DepGraph {
        // Trying to make source look like a package
        let (package, dirs) = if let Some(package) = package {
            (package, Some(vec![]))
        } else {
            ("<terminal>".to_string(), None)
        };

        // Manually build spec / DepNode for first call
        let name = "<terminal>".to_string();
        let spec = ModuleSpec {
            name: name.clone(),
            origin: None,
            // Treating this as the main file which is not a package
            parent: package,
            submodule_search_locations: dirs
        };


        let node = DepNode::new(spec.clone(), Some(0));
        self.graph.add(node);
        self._process_imports(spec, source);

        let _ = mem::replace(&mut self.processing, HashSet::new());
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

            debug!("Loading file: {}", path_str);
            let mut source_file = File::open(Path::new(source_path)).unwrap();
            let mut source = String::new();

            source_file.read_to_string(&mut source).unwrap();
            return Some(source)
        }

        return None
    }

    pub fn _process_imports(&mut self, spec: ModuleSpec, source: &str) {
        debug!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        debug!("Expanding '{}'", spec.name);
        debug!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        // Circular check
        assert!(!self.processing.contains(&spec.name), "Double processing detected for name: '{}'", spec.name);
        self.processing.insert(spec.name.clone());

        let stmts = parse(source);
        if self.verbose {
            dump_imports(&stmts);
        }

        for stmt in stmts {
            match stmt {
                ImportStmt::Import { names } => {
                    for alias in names {
                        // Don't care about asname, we only need the import name to analyze dependencies
                        self._process_dependency(Some(&spec.name), &alias.name);
                    }
                },
                ImportStmt::ImportFrom { module, names, level } => {
                    if let (Some(module), Some(level)) = (module, level) {
                        let module_name = if level != 0 {
                            if spec.parent == "<terminal>" {
                                panic!("Attempted relative import from terminal node (no known parent package)");
                            }

                            // Resolve name relative to the current package (parent in ModuleSpec)
                            resolve_name (
                                &module,
                                &spec.parent,
                                &level
                            )
                        } else {
                            module
                        };

                        // Place dependency on the module
                        self._process_dependency(
                            Some(&spec.name),
                            &module_name
                        );

                        // If this is a package we need to process the names b/c they may be submodules
                        let module_spec: Option<ModuleSpec> = find_spec(
                            &module_name
                        );

                        if let Some(s) = module_spec {
                            if s.is_package() {
                                for alias in names {
                                    self._process_dependency(
                                        Some(&spec.name),
                                        &format!(
                                            "{}.{}",
                                            module_name,
                                            alias.name
                                        )
                                    )
                                }
                            }
                        }
                    } else {
                        panic!("Broken assumption of implementation, revist this block to see if there are issues.")
                    }
                }
            }
        }
        self.processing.remove(&spec.name);
        debug!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
        debug!("Done '{}'", spec.name);
        debug!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
    }

    pub fn _process_dependency(&mut self, from: Option<&String>, name: &str) {
        // Maybe expensive but some values will change names after find_spec()
        // TODO: Deal with this in another way?
        let spec: Option<ModuleSpec> = find_spec(name);
    
        if spec.is_none() {
            debug!("!!!! Unable to find spec for name: '{}' !!!!", name);
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
            debug!("Processing dependency: {} -> {}", from, name);
        } else {
            debug!("Processing dependency on: {}", name);
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
        // None for depth to allow that to be resolved by add_dependency(...)
        let new_node = DepNode::new(spec.clone(), None);
        let source = self._load_source(&new_node);
        self.graph.add(new_node);

        // Add dependency from current node, to this new one
        if let Some(from) = from {
            self.graph.add_dependency(
                &from,
                &name
            );
        }

        // Process all dependencies of new node
        if let Some(source) = source {
            self._process_imports(spec, source.as_str())
        }

        // Done!
    }
}