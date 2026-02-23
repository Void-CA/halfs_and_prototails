use std::collections::HashMap;

use super::models::{BellmanFordResult, BellmanFordTable, Graph};

impl Graph {

    pub fn bellman_ford<'a>(&'a self, source: &'a str) -> BellmanFordResult<'a> {

        let mut distances: HashMap<&str, f32> = HashMap::new();
        let mut predecessors: HashMap<&str, Option<&str>> = HashMap::new();

        let nodes = self.nodes();
        let n = nodes.len();

        for node in &nodes {
            distances.insert(node, f32::INFINITY);
            predecessors.insert(node, None);
        }

        distances.insert(source, 0.0);

        let mut tables = Vec::new();
        
        // Relajación
        for i in 0..(n - 1) {
            for edge in &self.edges {

                let u = edge.from.as_str();
                let v = edge.to.as_str();

                if distances[u] != f32::INFINITY &&
                   distances[u] + edge.weight < distances[v] {

                    distances.insert(v, distances[u] + edge.weight);
                    predecessors.insert(v, Some(u));
                }

                let table = BellmanFordTable::new(i, distances.clone(), predecessors.clone());
                tables.push(table);
            }
        }

        // Detección ciclo negativo
        for edge in &self.edges {

            let u = edge.from.as_str();
            let v = edge.to.as_str();

            if distances[u] != f32::INFINITY &&
               distances[u] + edge.weight < distances[v] {

                let mut x = v;
                for _ in 0..n {
                    x = predecessors[x].unwrap();
                }

                let mut cycle = vec![x];
                let mut current = predecessors[x].unwrap();

                while current != x {
                    cycle.push(current);
                    current = predecessors[current].unwrap();
                }

                cycle.reverse();

                return BellmanFordResult {
                    distances,
                    predecessors,
                    tables,
                    negative_cycle: Some(cycle),
                };
            }
        }

        BellmanFordResult {
            distances,
            predecessors,
            tables,
            negative_cycle: None,
        }
    }
}
