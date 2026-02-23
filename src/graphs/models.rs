#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: f32,
}

use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub adj_list: HashMap<String, HashMap<String, f32>>,
    pub edges: Vec<Edge>,
}


impl Graph {
    pub fn new(adj_list: HashMap<String, HashMap<String, f32>>) -> Self {
        let mut edges = Vec::new();

        for (origin, adjacency) in &adj_list {
            for (destiny, weight) in adjacency {
                edges.push(Edge {
                    from: origin.clone(),
                    to: destiny.clone(),
                    weight: *weight,
                });
            }
        }

        Graph { adj_list, edges }
    }

    pub fn nodes(&self) -> Vec<&str> {
        self.adj_list.keys().map(|k| k.as_str()).collect()
    }

    pub fn plot(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        use std::collections::HashSet;

        let mut file = File::create(filename)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "    rankdir= LR;")?;
        writeln!(file, "    node [shape=circle, fontname=\"Arial\"];")?;
        writeln!(file, "    edge [fontname=\"Arial\"];")?;

        let mut drawn_edges: HashSet<(String, String)> = HashSet::new();

        for edge in &self.edges {
            let pair = (edge.from.clone(), edge.to.clone());
            let reverse_pair = (edge.to.clone(), edge.from.clone());

            // Si ya dibujamos la inversa, saltamos
            if drawn_edges.contains(&reverse_pair) {
                continue;
            }

            // Formateamos el label: sin decimales si es entero
            let label = if edge.weight.fract() == 0.0 {
                format!("{}", edge.weight as i32)
            } else {
                format!("{:.2}", edge.weight)
            };

            writeln!(
                file,
                "    \"{}\" -> \"{}\" [label=\"{}\"];", 
                edge.from, edge.to, label
            )?;

            drawn_edges.insert(pair);
        }

        writeln!(file, "}}")?;
        
        println!("Archivo {} generado con éxito.", filename);
        Ok(())
    }

}

#[derive(Debug)]
pub struct BellmanFordResult<'a> {
    pub distances: HashMap<&'a str, f32>,
    pub predecessors: HashMap<&'a str, Option<&'a str>>,
    pub tables: Vec<BellmanFordTable<'a>>,
    pub negative_cycle: Option<Vec<&'a str>>,
}

impl BellmanFordResult<'_> {
    pub fn show_distances(&self) {
        println!("Distancias desde el nodo fuente:");
        for (node, distance) in &self.distances {
            println!("{}: {}", node, distance);
        }
    }

    pub fn show_predecessors(&self) {
        println!("Predecesores de cada nodo:");
        for (node, predecessor) in &self.predecessors {
            let pred_str = match predecessor {
                Some(p) => format!("{}", p),
                None => String::from("Source"),
            };
            println!("{}: {}", node, pred_str);
        }
    }

        pub fn show_tables(&self) {
            for table in &self.tables {
                println!("{}", table);
            }
        }

        pub fn show_negative_cycle(&self) {
            if let Some(cycle) = &self.negative_cycle {
                println!("Ciclo negativo detectado: {:?}", cycle);
            } else {
                println!("No se detectó ningún ciclo negativo.");
            }
        }
}
#[derive(Debug)]
pub struct BellmanFordTable<'a> {
    pub iteration: usize,
    pub distances: HashMap<&'a str, f32>,
    pub predecessors: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> BellmanFordTable<'a> {
    pub fn new(iteration: usize, distances: HashMap<&'a str, f32>, predecessors: HashMap<&'a str, Option<&'a str>>) -> Self {
        BellmanFordTable {
            iteration,
            distances,
            predecessors,
        }
    }
}

impl<'a> fmt::Display for BellmanFordTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Iteration: {}", self.iteration)?;
        writeln!(f, "{:<10} | {:<10} | {:<15}", "Node", "Distance", "Predecessor")?;
        writeln!(f, "{:-<10}-+-{:-<10}-+-{:-<15}", "", "", "")?;
        // Para mantener el mismo orden, iteramos sobre las claves de distances
        for node in self.distances.keys() {
            let distance = self.distances.get(node).unwrap();
            let predecessor = self.predecessors.get(node).unwrap();
            let pred_str = match predecessor {
                Some(p) => format!("{}", p),
                None => String::from("None"),
            };
            writeln!(f, "{:<10} | {:<10} | {:<15}", node, distance, pred_str)?;
        }
        Ok(())
    }
}