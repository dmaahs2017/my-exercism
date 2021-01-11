pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for node in nodes.to_vec() {
                self.nodes.push(node)
            }
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for edge in edges.to_vec() {
                self.edges.push(edge);
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.to_vec().iter().fold(&mut self.attrs, |acc, (k, v)| {
                acc.insert(k.to_string(), v.to_string());
                acc
            });
            self
        }

        pub fn get_node(&self, node: &str) -> Option<Node> {
            self.nodes.iter().find(|&n| n.name == node ).map(|x| x.clone())
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.to_vec().iter().fold(&mut self.attrs, |acc, (k, v)| {
                        acc.insert(k.to_string(), v.to_string());
                        acc
                    });
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &s[..])
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                vertices: (String, String),
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(v1: &str, v2: &str) -> Self {
                    Edge {
                        vertices: (v1.to_string(), v2.to_string()),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.to_vec().iter().fold(&mut self.attrs, |acc, (k, v)| {
                        acc.insert(k.to_string(), v.to_string());
                        acc
                    });
                    self
                }
            }
        }
    }
}
