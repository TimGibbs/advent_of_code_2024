use std::{fs, io};
use std::collections::{BTreeSet, HashMap, HashSet};


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
    let cliques = find_3_node_cliques(&graph);

    // Filter cliques of size >= 3
    let large_cliques: Vec<_> = cliques.into_iter().filter(|c| c.len() >= 3).collect();
    
    let ts: Vec<&HashSet<String>> = large_cliques.iter().filter(|c| c.iter().any(|x| x.starts_with("t"))).collect();

    // Convert HashSet to BTreeSet for comparison
    let mut distinct: HashSet<BTreeSet<String>> = HashSet::new();

    for set in ts {
        distinct.insert(set.iter().cloned().collect());
    }

    println!("{:?}", distinct);
    println!("{:?}", distinct.iter().count());
    Ok(())
}

type Graph = HashMap<String, HashSet<String>>;


// Bron–Kerbosch recursive function
fn bron_kerbosch_3(
    r: &mut HashSet<String>,    // Current clique
    p: &mut HashSet<String>,    // Potential candidates
    x: &mut HashSet<String>,    // Already processed
    graph: &Graph,
    cliques: &mut Vec<HashSet<String>>, // Store cliques here
) {
    if p.is_empty() && x.is_empty() {
        // Store all cliques of size 3
        if r.len() >= 3 {
            cliques.push(r.clone());
        }
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

        bron_kerbosch_3(&mut r_new, &mut p_new, &mut x_new, graph, cliques);

        // Move v from P to X
        p.remove(&v);
        x.insert(v);
    }
}

// Helper function to find all 3-node cliques (including subsets of larger cliques)
fn find_3_node_cliques(graph: &Graph) -> Vec<HashSet<String>> {
    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<String> = graph.keys().cloned().collect();
    let mut x = HashSet::new();

    bron_kerbosch_3(&mut r, &mut p, &mut x, graph, &mut cliques);

    // Extract all 3-node cliques (subsets of larger cliques)
    let mut three_node_cliques = Vec::new();
    for clique in cliques {
        if clique.len() == 3 {
            three_node_cliques.push(clique.clone());
        } else if clique.len() > 3 {
            // Generate all 3-combinations of the clique
            let nodes: Vec<String> = clique.into_iter().collect();
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    for k in (j + 1)..nodes.len() {
                        three_node_cliques.push(
                            [nodes[i].clone(), nodes[j].clone(), nodes[k].clone()]
                                .into_iter()
                                .collect(),
                        );
                    }
                }
            }
        }
    }
    three_node_cliques
}