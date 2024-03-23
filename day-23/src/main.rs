use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-23/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    println!("Time: {:?}", now.elapsed());
    let now = std::time::Instant::now();
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

#[derive(PartialEq, Eq)]
enum TileType {
    Wall,
    Slope,
    Empty,
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
struct Tile {
    tile_type: TileType,
    direction: Option<Direction>,
}
impl Tile {
    pub fn from_char(c: char) -> Self {
        Self {
            tile_type: match c {
                '#' => TileType::Wall,
                '.' => TileType::Empty,
                _ => TileType::Slope,
            },
            direction: match c {
                '^' => Some(Direction::North),
                '>' => Some(Direction::East),
                'v' => Some(Direction::South),
                '<' => Some(Direction::West),
                _ => None,
            },
        }
    }
    pub fn from_char_without_slope(c: char) -> Self {
        Self {
            tile_type: match c {
                '#' => TileType::Wall,
                _ => TileType::Empty,
            },
            direction: None,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    position: (u16, u16),
    already_visited: Vec<(u16, u16)>,
    distance: u16,
}
impl State {
    pub fn new(position: (u16, u16), distance: u16) -> Self {
        Self {
            position,
            distance,
            already_visited: vec![],
        }
    }

    pub fn new_from_state(position: (u16, u16), last_state: &State) -> Self {
        let mut already_visited = last_state.already_visited.clone();
        already_visited.push(last_state.position);
        Self {
            position,
            distance: last_state.distance
                + (*vec![
                    position.0.abs_diff(last_state.position.0),
                    position.1.abs_diff(last_state.position.1),
                ]
                .iter()
                .max()
                .unwrap_or(&0)),
            already_visited,
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.distance.cmp(&other.distance) {
            Ordering::Greater => std::cmp::Ordering::Greater,
            Ordering::Less => std::cmp::Ordering::Less,
            Ordering::Equal => self.position.cmp(&other.position),
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct Map {
    tiles: Vec<Vec<Tile>>,
}
impl Map {
    pub fn from_str(s: &str) -> Self {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();
        Self { tiles }
    }
    pub fn from_str_without_slope(s: &str) -> Self {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(Tile::from_char_without_slope).collect())
            .collect();
        Self { tiles }
    }

    pub fn next_states_available(&self, state: &State) -> Vec<State> {
        let mut states = Vec::new();
        let next_positions = vec![
            (
                Direction::North,
                (state.position.0.wrapping_sub(1), state.position.1),
            ),
            (
                Direction::South,
                (state.position.0.wrapping_add(1), state.position.1),
            ),
            (
                Direction::West,
                (state.position.0, state.position.1.wrapping_sub(1)),
            ),
            (
                Direction::East,
                (state.position.0, state.position.1.wrapping_add(1)),
            ),
        ];

        for (dir, (x, y)) in next_positions {
            if x < self.tiles.len() as u16 && y < self.tiles[0].len() as u16 {
                if state.already_visited.contains(&(x, y)) {
                    continue;
                }

                match self.tiles[x as usize][y as usize].tile_type {
                    TileType::Empty => {
                        states.push(State::new_from_state((x, y), &state));
                    }
                    TileType::Slope => {
                        if let Some(direction) = self.tiles[x as usize][y as usize].direction {
                            if direction == dir {
                                match direction {
                                    Direction::North => {
                                        states.push(State::new_from_state((x - 1, y), &state))
                                    }
                                    Direction::South => {
                                        states.push(State::new_from_state((x + 1, y), &state))
                                    }
                                    Direction::West => {
                                        states.push(State::new_from_state((x, y - 1), &state))
                                    }
                                    Direction::East => {
                                        states.push(State::new_from_state((x, y + 1), &state))
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        states
    }

    pub fn find_longest_hike(&self) -> u16 {
        let mut dist = vec![u16::MIN; self.tiles[0].len() * self.tiles.len()];
        let mut heap = BinaryHeap::new();

        // Add initiale states
        dist[0] = 0;
        heap.push(State::new((0, 1), 0));

        while let Some(state) = heap.pop() {
            if state.distance
                < dist[(state.position.0 as usize) * self.tiles.len() + (state.position.1 as usize)]
            {
                continue;
            }

            let next_states = self.next_states_available(&state);

            for next_state in next_states.iter() {
                if next_state.distance
                    > dist[(next_state.position.0 as usize) * self.tiles.len()
                        + next_state.position.1 as usize]
                {
                    dist[(next_state.position.0 as usize) * self.tiles.len()
                        + next_state.position.1 as usize] = next_state.distance;
                    heap.push(next_state.clone());
                }
            }
        }

        *dist[(self.tiles.len() - 1) * self.tiles[0].len()..]
            .iter()
            .max()
            .unwrap_or(&0)
    }
}

#[derive(Eq, PartialEq, Clone)]
struct GraphState {
    state: State,
    origin: (u16, u16),
}
impl GraphState {
    pub fn new(origin: (u16, u16), state: State) -> Self {
        Self { state, origin }
    }
}
#[derive(Debug, Clone)]
struct Path {
    start: (u16, u16),
    end: (u16, u16),
    weight: u16,
}
#[derive(Debug, Clone)]
struct Node {
    id: u8,
    nb_path: u8,
    paths: [u8; 4],
}
impl Node {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            nb_path: 0,
            paths: [0; 4],
        }
    }

    pub fn add_path(&mut self, path_id: u8) {
        if self.nb_path == 4 {
            panic!("Too many path for this node");
        }
        self.paths[self.nb_path as usize] = path_id;
        self.nb_path += 1;
    }
}
struct GraphMap {
    paths: Vec<Path>,
    node_lookup_table: HashMap<(u16, u16), Node>,
    end_node_coord: (u16, u16),
}
impl GraphMap {
    pub fn new(input: &str) -> Self {
        let map = Map::from_str_without_slope(input);
        let mut paths = Vec::new();
        let mut node_lookup_table: HashMap<(u16, u16), Node> = HashMap::new();
        let mut heap = Vec::new();
        heap.push(GraphState::new((0, 1), State::new((0, 1), 0)));

        while let Some(graph_state) = heap.pop() {
            let state = graph_state.state;
            let mut previous_state = state.clone();
            loop {
                let next_states = map.next_states_available(&previous_state);

                match next_states.len().cmp(&1) {
                    Ordering::Equal => {
                        previous_state = next_states.first().unwrap().clone();
                    }
                    Ordering::Greater => {
                        if paths.iter().any(|path: &Path| {
                            (path.start == graph_state.origin
                                && path.end == previous_state.position)
                                || (path.start == previous_state.position
                                    && path.end == graph_state.origin)
                        }) {
                            break;
                        }

                        // Create path from state to previous_state
                        let path = Path {
                            start: graph_state.origin,
                            end: previous_state.position,
                            weight: previous_state.distance - state.distance + 1,
                        };
                        paths.push(path);

                        if !node_lookup_table.contains_key(&graph_state.origin) {
                            node_lookup_table.insert(
                                graph_state.origin,
                                Node::new((node_lookup_table.len() - 1) as u8),
                            );
                        }
                        node_lookup_table
                            .get_mut(&graph_state.origin)
                            .unwrap()
                            .add_path((paths.len() - 1) as u8);

                        if !node_lookup_table.contains_key(&previous_state.position) {
                            node_lookup_table.insert(
                                previous_state.position,
                                Node::new((node_lookup_table.len() - 1) as u8),
                            );
                        }
                        node_lookup_table
                            .get_mut(&previous_state.position)
                            .unwrap()
                            .add_path((paths.len() - 1) as u8);

                        for next_state in next_states.iter() {
                            heap.push(GraphState::new(previous_state.position, next_state.clone()));
                        }
                        break;
                    }
                    Ordering::Less => {
                        if paths.iter().any(|path: &Path| {
                            (path.start == graph_state.origin
                                && path.end == previous_state.position)
                                || (path.start == previous_state.position
                                    && path.end == graph_state.origin)
                        }) {
                            break;
                        }

                        if previous_state.position.0 as usize == map.tiles.len() - 1 {
                            let path = Path {
                                start: graph_state.origin,
                                end: previous_state.position,
                                weight: previous_state.distance - state.distance + 1,
                            };
                            paths.push(path);

                            if node_lookup_table.contains_key(&graph_state.origin) {
                                node_lookup_table.insert(
                                    graph_state.origin,
                                    Node::new((node_lookup_table.len() - 1) as u8),
                                );
                            }
                            node_lookup_table
                                .get_mut(&graph_state.origin)
                                .unwrap()
                                .add_path((paths.len() - 1) as u8);

                            node_lookup_table.insert(
                                previous_state.position,
                                Node::new((node_lookup_table.len() - 1) as u8),
                            );
                            node_lookup_table
                                .get_mut(&previous_state.position)
                                .unwrap()
                                .add_path((paths.len() - 1) as u8);
                        }
                        break;
                    }
                }
            }
        }

        Self {
            paths,
            node_lookup_table,
            end_node_coord: (
                (map.tiles.len() - 1) as u16,
                (map.tiles[0].len() - 2) as u16,
            ),
        }
    }

    pub fn find_longest_hike(&self) -> u16 {
        self.find_longest_path_from_coord(0, (0, 1), 0) - 1
    }

    fn find_longest_path_from_coord(
        &self,
        already_visited: u64,
        coord: (u16, u16),
        weight: u16,
    ) -> u16 {
        let mut states = Vec::new();
        states.push((already_visited, coord, weight));
        let end_node = self.node_lookup_table.get(&self.end_node_coord).unwrap();

        let mut longest_path = u16::MIN;
        while let Some((mut already_visited, coord, weight)) = states.pop() {
            let node = self.node_lookup_table.get(&coord).unwrap();
            if end_node.id == node.id {
                if weight > longest_path {
                    longest_path = weight;
                }
                continue;
            }

            already_visited |= 1 << node.id;
            for index in node.paths[0..node.nb_path as usize].iter() {
                let path = self.paths.get(*index as usize).unwrap();
                let next_coord = if path.start == coord {
                    path.end
                } else {
                    path.start
                };

                let next_node = self.node_lookup_table.get(&next_coord).unwrap();
                if already_visited & (1 << next_node.id) == 0 {
                    states.push((already_visited, next_coord, weight + path.weight));
                }
            }
        }
        longest_path + self.paths.get(end_node.paths[0] as usize).unwrap().weight
    }
}

fn part_1(input: &str) -> u16 {
    let map = Map::from_str(input);
    map.find_longest_hike()
}

fn part_2(input: &str) -> u16 {
    let graph_map = GraphMap::new(input);
    graph_map.find_longest_hike()
    // graph_map.node_lookup_table.len() as u16
}

#[cfg(test)]
mod tests_day23 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-23/test.txt");
        assert_eq!(part_1(input), 94);
    }

    #[test]
    fn test_graph_map() {
        let input = include_str!("../../aoc-2023-inputs/day-23/test2.txt");
        let graph_map = GraphMap::new(input);
        dbg!(graph_map.paths.to_vec());
        assert_eq!(graph_map.paths.len(), 5);
        assert_eq!(graph_map.paths[0].weight, 4);
        assert_eq!(graph_map.paths.iter().map(|p| p.weight).sum::<u16>(), 20);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-23/test2.txt");
        assert_eq!(part_2(input), 13);
        let input = include_str!("../../aoc-2023-inputs/day-23/test.txt");
        assert_eq!(part_2(input), 154);
    }
}
