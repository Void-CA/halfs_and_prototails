use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RoutingTable<'a> {
    pub owner: &'a str,
    // El "Vector de Distancia": Destino -> Costo
    pub distances: HashMap<&'a str, f32>,
}

impl<'a> std::fmt::Display for RoutingTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1. Obtener y ordenar los destinos para que la tabla sea consistente
        let mut destinations: Vec<&&str> = self.distances.keys().collect();
        destinations.sort();

        // 2. Imprimir el encabezado (los nombres de los nodos destino)
        write!(f, "Router {:<2} | ", self.owner)?;
        for dest in &destinations {
            write!(f, "{:^5} ", dest)?;
        }
        writeln!(f)?;

        // 3. Imprimir una línea separadora
        write!(f, "-----------|-")?;
        for _ in &destinations {
            write!(f, "------")?;
        }
        writeln!(f)?;

        // 4. Imprimir los costos
        write!(f, "Costos    | ")?;
        for dest in &destinations {
            let cost = self.distances.get(*dest).unwrap_or(&f32::INFINITY);
            if cost.is_infinite() {
                write!(f, "{:^5} ", "∞")?;
            } else {
                write!(f, "{:^5.0} ", cost)?; // .0 para que se vea como entero si prefieres
            }
        }
        writeln!(f)?;
        Ok(())
    }
}