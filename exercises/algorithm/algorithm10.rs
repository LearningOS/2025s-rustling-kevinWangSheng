/*
	graph
	This problem requires you to implement a basic graph functio
*/
// I AM NOT DONE

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from_node, to_node, weight) = edge;

        // Ensure both nodes exist in the graph
        self.add_node(from_node);
        self.add_node(to_node);

        // Add the edge in both directions (since it's an undirected graph)

        // Add edge from 'from_node' to 'to_node'
        let from_neighbors = self.adjacency_table_mutable()
            .get_mut(from_node)
            .unwrap(); // Safe because we just ensured the node exists

        // Check if edge already exists, and if so, update its weight
        let mut edge_exists = false;
        for neighbor in from_neighbors.iter_mut() {
            if neighbor.0 == to_node {
                neighbor.1 = weight;
                edge_exists = true;
                break;
            }
        }

        // If edge doesn't exist, add it
        if !edge_exists {
            from_neighbors.push((String::from(to_node), weight));
        }

        // Add edge from 'to_node' to 'from_node'
        let to_neighbors = self.adjacency_table_mutable()
            .get_mut(to_node)
            .unwrap(); // Safe because we just ensured the node exists

        // Check if edge already exists, and if so, update its weight
        let mut edge_exists = false;
        for neighbor in to_neighbors.iter_mut() {
            if neighbor.0 == from_node {
                neighbor.1 = weight;
                edge_exists = true;
                break;
            }
        }

        // If edge doesn't exist, add it
        if !edge_exists {
            to_neighbors.push((String::from(from_node), weight));
        }
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        // If the node already exists in the graph, return false
        if self.contains(node) {
            return false;
        }

        // Add the node to the adjacency table with an empty vector of neighbors
        self.adjacency_table_mutable().insert(String::from(node), Vec::new());
        true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}