mod graphs;
mod distance_vector;


fn main() {
    ex1();

    ex2();
}

fn ex1() {
    println!("Ejercicio 1:");
    let g = build_net! {
        R1 => {R2 : 10, R3 : 10},
        R2 => {R1 : 10, R4 : 5, R5 : 3},
        R3 => {R1 : 10, R4 : 50},
        R4 => {R2 : 5, R3 : 50, R5 : 10, R7 : 8},
        R5 => {R2 : 3, R4 : 10, R6 : 14, R7: 30},
        R6 => {R5 : 14},
        R7 => {R4 : 8, R5 : 30}
    };

    let result = g.bellman_ford("R1");
    g.plot("ejercicio1.dot").unwrap();
    result.show_distances();
    result.show_predecessors();
}

fn ex2() {
    println!("\nEjercicio 2:");
    let g = build_net! {
        R1 => {R2 : 2, R3 : 5},
        R2 => {R1 : 2, R3 : 1, R4: 2},
        R3 => {R1 : 5, R2 : 1, R4 : 3, R5: 6},
        R4 => {R2 : 2, R3 : 3, R5 : 1},
        R5 => {R3 : 6, R4 : 1}

    };

    let result = g.bellman_ford("R1");
    g.plot("ejercicio2.dot").unwrap();
    result.show_distances();    
    result.show_predecessors();
}