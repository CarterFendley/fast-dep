use std::collections::HashSet;
use std::mem;
use std::path::Path;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use log::{debug, info, warn};

use pyo3::prelude::*;
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};

use crate::importlib::*;
use crate::minimal_parser::*;
use super::types::*;

pub struct BuildMetadata {
    pub processed: usize,
    pub from_cache: usize,
}

impl BuildMetadata {
    pub fn new() -> BuildMetadata {
        BuildMetadata {
            processed: 0,
            from_cache: 0,
        }
    }
}

#[pyclass]
pub struct GraphBuilder {
    pub graph: DepGraph,
    processing: HashSet<String>,
    verbose: bool,
    cache: Option<DepGraph>,
    metadata: BuildMetadata,
    // For profiling
    pyro_server: Option<String>,
    pyro_app_name: String,
    pyro_invocation_count: u32,
}

#[pymethods]
impl GraphBuilder {
    #[new]
    pub fn new(
        verbose: Option<bool>,
        pyro_server: Option<String>,
        pyro_app_name: Option<String>,
    ) -> Self {
        let verbose = if let Some(verbose) = verbose {
            verbose
        } else {
            false
        };

        let pyro_app_name = if let Some(pyro_app_name) = pyro_app_name {
            pyro_app_name
        } else {
            "fast-dep".to_string()
        };

        let builder = GraphBuilder {
            graph: DepGraph::new(),
            processing: HashSet::new(),
            verbose: verbose,
            cache: None,
            metadata: BuildMetadata::new(),
            // For profiling
            pyro_server: pyro_server,
            pyro_app_name: pyro_app_name,
            pyro_invocation_count: 0,
        };

        builder
    }

    pub fn build(&mut self, source: &str, package: Option<String>) -> DepGraph {
        let profiling_agent = if let Some(pyro_server) = self.pyro_server.clone() {
            let session_name = format!("{}-{}", self.pyro_app_name, 
            self.pyro_invocation_count);
            info!(
                "Building pyroscope agent: {}",
                session_name
            );
            let agent = PyroscopeAgent::builder(
                pyro_server,
                session_name.clone()
            ).backend(pprof_backend(PprofConfig::new().sample_rate(100)))
            .build();

            let agent = match agent {
                Ok(agent) => Some(agent),
                Err(error) => {
                    warn!("Pyroscope agent build failed with error: {}", error);
                    // Trying to less "frowned on" by using unwraps here but this branching is odd. Ideally here I would just immediately set the outer `profiling_agent` to None, but I can't figure out how to do this. Instead, I have to react to this `None` value in code below, even though I know what I want `profiling_agent` to be and don't need to continue this if clause at all.
                    None
                }
            };

            self.pyro_invocation_count += 1;

            match agent {
                Some(agent) => {
                    // I also feel this tends to happen, a lot of nested matching. I love matching and hate that Python / C++ does not have them but when I have like 3+ matches, I start to go dizzy. Probably this one is especially cause I re-use `agent` here a bunch, but the alternative is to have a bunch of extra vars floating around.
                    match agent.start() {
                        Ok(agent) => {
                            info!(
                                "Pyroscope agent started: {}",
                                session_name
                            );
                            Some(agent)
                        }, // Happy path!
                        Err(error) => {
                            warn!("Pyroscope agent failed to start with error: {}", error);
                            None
                        }
                    }
                }
                None => None
            }
        } else {
            None
        };

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

        info!("Building graph...");
        self._process_imports(spec, source);

        if self.metadata.from_cache == 0 {
            info!(
                "Processed {} dependency relationships.",
                self.metadata.processed
            );
        } else {
            info!(
                "Processed {} dependency relationships ({} from cache).",
                self.metadata.processed,
                self.metadata.from_cache
            );
        }

        // Reset for next build
        let _ = mem::replace(&mut self.processing, HashSet::new());
        let _ = mem::replace(&mut self.metadata, BuildMetadata::new());
        let graph = mem::replace(&mut self.graph, DepGraph::new());

        // Cache all nodes
        let to_cache = graph.clone();
        if let Some(cache) = self.cache.as_mut() {
            cache.merge(to_cache)
        } else {
            self.cache = Some(to_cache)
        }

        if let Some(agent) = profiling_agent {
            // Note, not 100% sure why two calls are needed here
            let agent_ready = agent.stop().unwrap();
            agent_ready.shutdown();
        }

        return graph
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
                warn!(
                    "Unable to load extension for spec '{}' with origin '{}' skipping.",
                    node.spec.name,
                    path_str
                );
                return None
            }

            debug!("Loading file: {}", path_str);
            let mut source_file = File::open(Path::new(source_path));
            if let Err(err) = source_file {
                warn!(
                    "Unable to load file for spec '{}' with origin '{}' skipping.",
                    node.spec.name,
                    path_str,
                );
                eprintln!("System error: {err}");
                return None
            }
            let mut source = String::new();

            source_file.unwrap().read_to_string(&mut source).unwrap();
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
        // TODO: Example??
        if let Some(from) = from {
            if from == &name {
                return
            }
        }

        self.metadata.processed += 1;
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
        } else if name != "<terminal>" {
            // TODO: Can this happen before reaching out to python
            if !self.cache.is_none() {
                if self.cache.as_ref().unwrap().has_node(&name) {
                    // Process the parent and see if that adds the node first
                    self._process_parent(from, &name);

                    // Spooky!
                    // TODO: This logic is 1 to 1 with logic further down, combine?
                    if self.graph.has_node(&name) {
                        if let Some(from) = from {
                            self.graph.add_dependency(&from, &name);
                        }

                        // Done!
                        return
                    }

                    // Other wise need to add ourselves
                    let cache = self.cache.as_ref().unwrap();
                    if let Some(from) = from {
                        let subgraph = cache.clone_from(&name);

                        // Track metadata
                        let deps_added = subgraph.num_dependencies();
                        self.metadata.from_cache += deps_added + 1;
                        self.metadata.processed += deps_added;

                        self.graph.add_graph(
                            from,
                            &name,
                            subgraph
                        );

                        // Done!
                        return
                    } else {
                        panic!("Adding graph without `from` is not implemented")
                    }
                }
            }
        }

        // Process parent before anything else
        self._process_parent(from, &name);

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
        self.graph.add(new_node); // Can this be delayed, how about self reference?

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

    fn _process_parent(&mut self, from: Option<&String>, name: &str) {
        let names: Vec<&str> = name.split(".").collect();
        let parent = names[..names.len() - 1].join(".");
        if parent.len() != 0 {
            self._process_dependency(from, parent.as_str());
        }
    }
}