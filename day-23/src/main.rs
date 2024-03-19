use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-23/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    println!("Time: {:?}", now.elapsed());
    // dbg!(part_2(input));
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
                '.' | 'S' => TileType::Empty,
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
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    position: (usize, usize),
    already_visited: Vec<(usize, usize)>,
    distance: u64,
}
impl State {
    pub fn new(position: (usize, usize), distance: u64) -> Self {
        Self {
            position,
            distance,
            already_visited: vec![],
        }
    }

    pub fn new_from_state(position: (usize, usize), last_state: &State) -> Self {
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
                .unwrap_or(&0)) as u64,
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
            if x < self.tiles.len() && y < self.tiles[0].len() {
                if state.already_visited.contains(&(x, y)) {
                    continue;
                }

                match self.tiles[x][y].tile_type {
                    TileType::Empty => {
                        states.push(State::new_from_state((x, y), &state));
                    }
                    TileType::Slope => {
                        if let Some(direction) = self.tiles[x][y].direction {
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

    pub fn find_longest_hike(&self) -> u64 {
        let mut dist = vec![u64::MIN; self.tiles[0].len() * self.tiles.len()];
        let mut heap = BinaryHeap::new();

        // Add initiale states
        dist[0] = 0;
        heap.push(State::new((0, 0), 0));

        while let Some(state) = heap.pop() {
            if state.distance < dist[(state.position.0 * self.tiles.len()) + state.position.1] {
                continue;
            }

            let next_states = self.next_states_available(&state);

            for next_state in next_states.iter() {
                if next_state.distance
                    > dist[next_state.position.0 * self.tiles.len() + next_state.position.1]
                {
                    dist[next_state.position.0 * self.tiles.len() + next_state.position.1] =
                        next_state.distance;
                    heap.push(next_state.clone());
                }
            }
        }

        *dist.iter().max().unwrap_or(&0) - 1
    }
}

fn part_1(input: &str) -> u64 {
    let map = Map::from_str(input);
    map.find_longest_hike()
}

// fn part_2(input: &str) -> u64 {
//     todo!()
// }

#[cfg(test)]
mod tests_day23 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-23/test.txt");
        assert_eq!(part_1(input), 94);
    }

    /* #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input), 0);
    } */
}
