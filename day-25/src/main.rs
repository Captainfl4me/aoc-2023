use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-25/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    // dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, HashMap<&'a str, usize>>,
}
impl<'a> Graph<'a> {
    fn from_edges(edges: Vec<(&'a str, &'a str)>) -> Self {
        let mut graph = Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        };

        for (a, b) in edges.iter() {
            graph.add_edge(a, b, 1);
        }

        graph
    }

    pub fn add_edge(&mut self, a: &'a str, b: &'a str, w: usize) {
        self.nodes.insert(a);
        self.nodes.insert(b);

        self.edges.entry(a).or_default().insert(b, w);
        self.edges.entry(b).or_default().insert(a, w);
    }

    pub fn update_edge(&mut self, a: &'a str, b: &'a str, w: usize) {
        *self.edges.entry(a).or_default().entry(b).or_insert(0) += w;
        *self.edges.entry(b).or_default().entry(a).or_insert(0) += w;
    }

    pub fn remove_edge(&mut self, a: &'a str) {
        self.nodes.remove(a);
        self.edges.remove(a);
        for (_, node_neighboors) in self.edges.iter_mut() {
            node_neighboors.remove(a);
        }
    }

    pub fn get_neighboors(&self, a: &'a str) -> Vec<(&'a str, usize)> {
        self.edges
            .get(a)
            .unwrap()
            .iter()
            .map(|(node, w)| (*node, *w))
            .collect()
    }

    pub fn vertices_contraction(&mut self, v0: &'a str, v1: &'a str) {
        for (neighboor, w) in self.get_neighboors(v1) {
            self.update_edge(v0, neighboor, w);
        }
        self.remove_edge(v1);
    }

    fn minimum_cut_phase(&self) -> (&'a str, &'a str, usize) {
        let mut priority_queue = PriorityQueue::new();
        self.nodes.iter().for_each(|node| {
            priority_queue.push(node, 0);
        });

        let mut s = "";
        let mut t = "";
        let mut cut_weight = 0;
        while let Some((node, w)) = priority_queue.pop() {
            s = t;
            t = node;
            cut_weight = w;

            for (neighboor, w) in self.get_neighboors(node) {
                priority_queue.change_priority_by(&neighboor, |p| *p += w);
            }
        }

        (s, t, cut_weight)
    }

    /// Stoer–Wagner's algorithm
    /// Returns one partittion and the minimum cut of the graph
    fn minimum_cut(&self) -> (HashSet<&'a str>, usize) {
        let mut temp_graph = self.clone();
        let mut best_phase = 0;
        let mut phases = Vec::new();
        let mut min_cut = usize::MAX;

        for phase in 0..temp_graph.nodes.len() - 1 {
            let (s, t, cut_weight) = temp_graph.minimum_cut_phase();
            temp_graph.vertices_contraction(s, t);
            phases.push((s, t));

            if cut_weight < min_cut {
                min_cut = cut_weight;
                best_phase = phase;
            }
        }

        let mut graph = HashMap::new();
        for (s, t) in phases.iter().take(best_phase) {
            graph.entry(*s).or_insert(Vec::new()).push(t);
            graph.entry(*t).or_insert(Vec::new()).push(s);
        }

        let mut partition = HashSet::new();
        let mut search_queue = VecDeque::new();
        search_queue.push_back(phases[best_phase].1);
        while let Some(node) = search_queue.pop_front() {
            if partition.contains(node) {
                continue;
            }

            partition.insert(node);
            if let Some(neighboors) = graph.get(node) {
                for neighboor in neighboors {
                    search_queue.push_back(*neighboor);
                }
            }
        }

        (partition, min_cut)
    }
}

fn part_1(input: &str) -> u64 {
    let mut edges = Vec::new();
    for line in input.lines() {
        let line_split = line.split(':').collect::<Vec<_>>();
        let start = line_split[0].trim();
        let ends = line_split[1].trim().split(' ').collect::<Vec<_>>();
        for end in ends {
            edges.push((start, end));
        }
    }
    let graph = Graph::from_edges(edges.clone());
    let (partition, _) = graph.minimum_cut();
    (partition.len() * (graph.nodes.len() - partition.len())) as u64
}

/* fn part_2(input: &str) -> u64 {
    todo!()
} */

#[cfg(test)]
mod tests_day25 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-25/test.txt");
        assert_eq!(part_1(input), 54);
    }

    /* #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input), 0);
    } */
}
