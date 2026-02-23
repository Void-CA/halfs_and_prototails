use std::collections::HashMap;
use super::RoutingTable;
use crate::graphs::Graph;

impl Graph {
    pub fn distance_vector_simulation(&self) -> Vec<HashMap<String, RoutingTable<'_>>> {
        let nodes = self.nodes();
        // Estado inicial: Cada nodo solo conoce a sus vecinos
        let mut network_state: HashMap<String, RoutingTable> = HashMap::new();

        for &node in &nodes {
            let mut distances = HashMap::new();
            for &target in &nodes {
                distances.insert(target, f32::INFINITY);
            }
            distances.insert(node, 0.0);
            
            // Añadir distancias a vecinos directos iniciales
            if let Some(adj) = self.adj_list.get(node) {
                for (neighbor, &weight) in adj {
                    distances.insert(neighbor, weight);
                }
            }
            
            network_state.insert(node.to_string(), RoutingTable { owner: node, distances });
        }

        let mut history = vec![network_state.clone()];
        let mut changed = true;

        // Bucle de convergencia (pasos de la pizarra)
        while changed {
            changed = false;
            let mut next_state = network_state.clone();

            for u in &nodes {
                for v in &nodes {
                    // Si 'u' y 'v' son vecinos, 'u' recibe el vector de 'v'
                    if let Some(weight_u_v) = self.get_edge_weight(u, v) {
                        let v_table = network_state.get(*v).unwrap().distances.clone();
                        
                        let u_table = next_state.get_mut(*u).unwrap();
                        
                        for (destination, &dist_v_to_dest) in &v_table {
                            let new_route = weight_u_v + dist_v_to_dest;
                            let current_route = *u_table.distances.get(destination).unwrap_or(&f32::INFINITY);
                            
                            if new_route < current_route {
                                u_table.distances.insert(destination, new_route);
                                changed = true;
                            }
                        }
                    }
                }
            }
            network_state = next_state;
            history.push(network_state.clone());
        }
        history
    }

    fn get_edge_weight(&self, from: &str, to: &str) -> Option<f32> {
        self.adj_list.get(from)?.get(to).copied()
    }
}