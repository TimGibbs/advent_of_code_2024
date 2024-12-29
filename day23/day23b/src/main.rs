use std::{fs, io};
use std::collections::{HashMap, HashSet};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let edges: Vec<(String,String)> = input_data.lines().map(|s| {
        let q: Vec<&str> = s.split('-').collect();
        (q[0].to_string(), q[1].to_string())
    }).collect();

    let mut graph: Graph = HashMap::new();
    for (a, b) in edges {
        graph.entry(a.to_string()).or_insert_with(HashSet::new).insert(b.to_string());
        graph.entry(b.to_string()).or_insert_with(HashSet::new).insert(a.to_string());
    }

    // Find all cliques
    let cliques = find_cliques(&graph);

    // Find the maximum clique
    let max_clique: &Vec<String> = cliques.iter().max_by_key(|c| c.len()).unwrap();
    
    println!("{:?}", max_clique);
    
    Ok(())
}

type Graph = HashMap<String, HashSet<String>>;

fn bron_kerbosch(
    r: &mut HashSet<String>,    // Current clique
    p: &mut HashSet<String>,    // Potential candidates
    x: &mut HashSet<String>,    // Already processed
    graph: &Graph,
    cliques: &mut Vec<Vec<String>>, // Store cliques here
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        cliques.push(r.iter().cloned().collect());
        return;
    }

    for v in p.clone() {
        let mut r_new = r.clone();
        r_new.insert(v.clone());

        let neighbors = match graph.get(&v) {
            Some(neigh) => neigh,
            None => &HashSet::new(),
        };
        let mut p_new = p.intersection(neighbors).cloned().collect();
        let mut x_new = x.intersection(neighbors).cloned().collect();

        bron_kerbosch(&mut r_new, &mut p_new, &mut x_new, graph, cliques);

        // Move v from P to X
        p.remove(&v);
        x.insert(v);
    }
}

// Helper function to find cliques in a graph
fn find_cliques(graph: &Graph) -> Vec<Vec<String>> {
    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<String> = graph.keys().cloned().collect();
    let mut x = HashSet::new();

    bron_kerbosch(&mut r, &mut p, &mut x, graph, &mut cliques);
    // Sort each clique alphabetically
    for clique in &mut cliques {
        clique.sort();
    }
    
    cliques
}