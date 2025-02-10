use std::{collections::HashSet, fs::{self, File}};

use rand::Rng;
use star::data_structures::graph::Graph;

fn main() {
    let mut temp_graphs = Vec::new();
    let folder = fs::read_dir("erdos-renyi_graphs_1").expect("Error reading folder");
    folder.for_each(|fe| {
        let path = fe.unwrap().path();
        let path_str = path.to_str().unwrap();
        let graph = read_graph(path_str);
        let time_labels = generate_time_labels(&graph);
        temp_graphs.push((graph, time_labels));
    });

    println!("finished reading graphs");

    let mut fulfill_property_one = 0;
    let mut fulfill_property_two = 0;
    let mut fulfill_property_three = 0;
    let mut fulfill_property_four = 0;
    let mut counter = 0;

    for (mut graph, time_labels) in temp_graphs {
        counter += 1;
        println!("Starting graph {}", counter);
        let n = graph.nodes as f64;
        let log_n = f64::log2(graph.nodes as f64);
        let deleted_edges = set_p(&mut graph, &time_labels, 0.75 * log_n / n);
        if property_four(&graph, &time_labels, &deleted_edges) {
            fulfill_property_four += 1;
        }
        println!("Finished poperty 4 of graph {}", counter);
        let deleted_edges = set_p(&mut graph, &time_labels, 0.5 * log_n / n);
        if property_three(&graph, &time_labels, &deleted_edges) {
            fulfill_property_three += 1;
        }
        println!("Finished poperty 3 of graph {}", counter);
        if property_two(&graph, &time_labels, &deleted_edges) {
            fulfill_property_two += 1;
        }
        println!("Finished poperty 2 of graph {}", counter);
        let deleted_edges = set_p(&mut graph, &time_labels, 0.25 * log_n / n);
        if property_one(&graph, &time_labels, &deleted_edges) {
            fulfill_property_one += 1;
        }
        println!("Finished poperty 1 of graph {}", counter);
    }

    println!("Fulfill property one: {} / 100", fulfill_property_one);
    println!("Fulfill property two: {} / 100", fulfill_property_two);
    println!("Fulfill property three: {} / 100", fulfill_property_three);
    println!("Fulfill property four: {} / 100", fulfill_property_four);
}

fn read_graph(path: &str) -> Graph {
    let file = File::open(path).expect("Error opening file");
    let bufread = std::io::BufReader::new(file);
    let g = Graph::try_from(bufread).expect("Error parsing graph");
    g
}

fn generate_time_labels(graph: &Graph) -> Vec<Vec<f64>> {
    let mut rng = rand::rng();
    let mut labels = Vec::new();
    for x in graph.edges.iter() {
        labels.push(vec![None; x.len()]);
    }
    for (u, x) in graph.edges.iter().enumerate() {
        for (i, v) in x.iter().enumerate() {
            let j = graph.edges[*v].iter().position(|&x| x == u).unwrap();
            match labels[*v][j] {
                Some(l) => labels[u][i] = Some(l),
                None => {
                    let l = rng.random();
                    labels[u][i] = Some(l);
                }
            }
        }
    }

    labels
        .iter()
        .map(|u| u.iter().map(|v| v.unwrap()).collect())
        .collect()
}

fn set_p(graph: &mut Graph, time_labels: &Vec<Vec<f64>>, p: f64) -> Vec<Vec<bool>> {
    let mut deleted_edges = vec![vec![false; graph.nodes]; graph.nodes];

    for (u, x) in graph.edges.iter().enumerate() {
        for (i, _) in x.iter().enumerate() {
            if time_labels[u][i] > p {
                deleted_edges[u][i] = true;
            }
        }
    }

    deleted_edges
}

fn construct_foremost_tree(
    graph: &Graph,
    time_labels: &Vec<Vec<f64>>,
    deleted_edges: &Vec<Vec<bool>>,
    source_vertex: usize,
) -> (HashSet<usize>, Vec<(usize, usize)>) {
    let mut n = HashSet::new();
    n.insert(source_vertex);
    let mut nodes = (n, Vec::new());
    for _ in 1..graph.nodes {
        let e_k = nodes
            .0
            .iter()
            .flat_map(|u| {
                graph.edges[*u]
                    .iter()
                    .enumerate()
                    .filter(|(i, v)| !nodes.0.contains(v) && !deleted_edges[*u][*i])
                    .map(|(_, v)| (*u, *v))
            }).map(|(u, v)| {
                (
                    (u, v),
                    time_labels[u][graph.edges[u].iter().position(|x| *x == v).unwrap()],
                )
            })
            .reduce(|(i, u), (j, v)| if u < v { (i, u) } else { (j, v) });

        let e_k = match e_k {
            None => break,
            Some(e_k) => e_k,
            
        };
        
        nodes.0.insert(e_k.0.1);
        nodes.1.push(e_k.0);
    }
    nodes
}

fn property_one(
    graph: &Graph,
    time_labels: &Vec<Vec<f64>>,
    deleted_edges: &Vec<Vec<bool>>,
) -> bool {
    let foremost_tree = construct_foremost_tree(graph, time_labels, deleted_edges, 0);
    foremost_tree.0.contains(&1)
}

fn property_two(
    graph: &Graph,
    time_labels: &Vec<Vec<f64>>,
    deleted_edges: &Vec<Vec<bool>>,
) -> bool {
    for i in 0..graph.nodes {
        let foremost_tree = construct_foremost_tree(graph, time_labels, deleted_edges, i);
        if foremost_tree.0.len() == graph.nodes {
            return true;
        }
    }
    false
}

fn property_three(
    graph: &Graph,
    time_labels: &Vec<Vec<f64>>,
    deleted_edges: &Vec<Vec<bool>>,
) -> bool {
    let foremost_tree = construct_foremost_tree(graph, time_labels, deleted_edges, 0);
    foremost_tree.0.len() == graph.nodes
}

fn property_four(
    graph: &Graph,
    time_labels: &Vec<Vec<f64>>,
    deleted_edges: &Vec<Vec<bool>>,
) -> bool {
    for i in 0..graph.nodes {
        let foremost_tree = construct_foremost_tree(graph, time_labels, deleted_edges, i);
        if foremost_tree.0.len() < graph.nodes {
            return false;
        }
    }

    true
}
