extern crate rand;

use rand::distributions::Uniform;
use rand::Rng;

use std::collections::BTreeSet;
use std::iter::FromIterator;

type Entry = (usize, usize);
type Table = BTreeSet<Entry>;
type Graph = BTreeSet<Entry>;
type State = (Graph, Vec<Table>);

fn main() {
    let n = 30;
    let mut state = init(n);
    // let graph = init_graph(n);
    // let tables = init_all_tables(n);
    // let mut state = (graph, tables);
    println!("Initial tables");
    print_tables(&state);
    for i in 1..=n * 2 {
        iterate(&mut state);
        println!("======= After iteration {} =======", i);
        print_tables(&state);
    }
}

fn iterate(state: &mut State) {
    *state = construct_tables(state);
}

fn print_tables((_graph, tables): &State) {
    for (i, table) in tables.iter().enumerate() {
        println!("Table {}", i);
        for entry in table {
            println!("{:?}", entry);
        }
        println!("=======");
        println!();
    }
}

fn construct_tables(state: &State) -> State {
    let (graph, tables) = state;
    let new_tables = (0..tables.len())
        .map(|index| construct_table_single(index, state))
        .collect::<Vec<_>>();
    (graph.clone(), new_tables)
}

fn construct_table_single(index: usize, (graph, tables): &State) -> Table {
    let neighbors = find_neighbors(index, graph);
    let readset = read_neighbors(&neighbors, tables);
    let readset_removed = remove_self(index, &readset);
    let readset_incr = increment_all(&readset_removed);
    let readset_w_self = add_self(index, &readset_incr);
    let readset_min = min_dist(&readset_w_self);
    let readset_con = con_prefix(&readset_min);
    readset_con
}

fn init(n: usize) -> State {
    (init_graph(n), init_all_tables(n))
}

fn init_all_tables(n: usize) -> Vec<Table> {
    (0..n).map(|_| init_table(n)).collect()
}

fn init_table(n: usize) -> Table {
    let dist = dist(n);
    let mut rng = rand::thread_rng();
    let count = rng.sample(dist);
    (0..count)
        .map(|_| (rng.sample(&dist), rng.sample(&dist)))
        .collect()
}

fn init_graph(n: usize) -> Graph {
    (0..n)
        .map(|i| init_node(i, n))
        .fold(BTreeSet::new(), |acc, comp| &acc | &comp)
}

fn init_node(index: usize, n: usize) -> Graph {
    let dist = dist(n);
    let mut other_indices = (0..n).filter(|&i| i != index).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut other_indices);
    let count = rng.sample(dist);
    other_indices
        .iter()
        .take(count)
        .fold(Graph::new(), |acc, &to| {
            let mut s = Graph::new();
            s.insert((index, to));
            s.insert((to, index));
            &acc | &s
        })
}

fn init_specific_graph() -> Graph {
    let connections = vec![
        (0, 1),
        (0, 4),
        (1, 0),
        (1, 2),
        (1, 4),
        (2, 1),
        (2, 3),
        (2, 4),
        (3, 2),
        (3, 5),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 5),
        (5, 3),
        (5, 4),
    ];
    Graph::from_iter(connections)
}

fn find_neighbors(index: usize, graph: &Graph) -> Vec<usize> {
    graph
        .iter()
        .filter(|&(from, _to)| from == &index)
        .map(|&(_from, to)| to)
        .collect::<Vec<_>>()
}

fn read_neighbors(neighbors: &[usize], tables: &Vec<Table>) -> Table {
    neighbors
        .iter()
        .map(|&n| &tables[n])
        .fold(Table::new(), |acc, processors| &acc | &processors)
}

fn remove_self(index: usize, table: &Table) -> Table {
    table
        .iter()
        .cloned()
        .filter(|&(id, _dis)| id != index)
        .collect()
}

fn increment_all(table: &Table) -> Table {
    table
        .iter()
        .cloned()
        .map(|(id, dis)| (id, dis + 1))
        .collect()
}

fn add_self(index: usize, table: &Table) -> Table {
    let mut s = table.clone();
    s.insert((index, 0));
    s
}

fn min_dist_single(index: usize, table: &Table) -> Entry {
    *table
        .iter()
        .filter(|(id, _dis)| *id == index)
        .min_by_key(|(_id, dis)| dis)
        .unwrap()
}

fn min_dist(table: &Table) -> Table {
    table
        .iter()
        .map(|&(id, _dis)| min_dist_single(id, table))
        .collect()
}

fn con_prefix(table: &Table) -> Table {
    let distances = table
        .iter()
        .map(|(_id, dis)| dis.clone())
        .collect::<BTreeSet<_>>();
    let max_distance = distances.iter().fold(0, |acc, &dis| acc.max(dis));
    let possible_distances = (0..=max_distance).collect::<BTreeSet<_>>();
    let missing_distances = &possible_distances - &distances;
    println!("missing distances: {:?}", missing_distances);
    let minimal_missing_distance = missing_distances
        .iter()
        .fold(max_distance + 1, |acc, &dis| acc.min(dis));
    table
        .iter()
        .cloned()
        .filter(|(_id, dis)| dis < &minimal_missing_distance)
        .collect()
}

fn dist(n: usize) -> Uniform<usize> {
    Uniform::from(0..n * 2)
}
