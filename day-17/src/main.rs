use std::cmp::Ordering;
use std::collections::BinaryHeap;

//Djikstra's algorithm
fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-17/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let map = Map::from_str(input);
    map.compute_min_heat_loss(1, 3)
}
fn part_2(input: &str) -> u64 {
    let map = Map::from_str(input);
    map.compute_min_heat_loss(4, 10)
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_to(&self, dir: Direction) -> Option<Self> {
        if self.x == 0 && dir == Direction::West || self.y == 0 && dir == Direction::North {
            return None;
        }
        match dir {
            Direction::North => Some(Self::new(self.x, self.y - 1)),
            Direction::East => Some(Self::new(self.x + 1, self.y)),
            Direction::South => Some(Self::new(self.x, self.y + 1)),
            Direction::West => Some(Self::new(self.x - 1, self.y)),
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn usize(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}
#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    pos: Position,
    direction: Direction,
    heat_loss: u32,
}
impl State {
    pub fn new(pos: Position, direction: Direction, heat_loss: u32) -> Self {
        Self {
            pos,
            direction,
            heat_loss,
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.heat_loss.cmp(&other.heat_loss) {
            Ordering::Greater => std::cmp::Ordering::Less,
            Ordering::Less => std::cmp::Ordering::Greater,
            Ordering::Equal => (self.pos, self.direction).cmp(&(other.pos, other.direction)),
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct Map {
    grid: Vec<Vec<u8>>,
}
impl Map {
    pub fn from_str(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_string().parse::<u8>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn step_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        let new_pos = pos.move_to(dir);
        if let Some(new_pos) = new_pos {
            if new_pos.x >= self.grid[0].len() || new_pos.y >= self.grid.len() {
                return None;
            }
        }
        new_pos
    }

    pub fn state_from_dir(
        &self,
        pos: Position,
        dir: Direction,
        init_heat: u32,
        min_length: usize,
        max_length: usize,
    ) -> Vec<State> {
        let mut states = Vec::new();
        let mut heat = init_heat;
        let mut new_pos = self.step_pos(pos, dir);
        for dist in 1..=max_length {
            if new_pos.is_none() {
                break;
            }
            heat += self.grid[new_pos.unwrap().y][new_pos.unwrap().x] as u32;
            if dist >= min_length {
                states.push(State::new(new_pos.unwrap(), dir, heat));
            }
            new_pos = self.step_pos(new_pos.unwrap(), dir);
        }
        states
    }

    pub fn compute_min_heat_loss(&self, min_length: usize, max_length: usize) -> u64 {
        let mut dist: Vec<u32> = (0..(self.grid.len() * self.grid[0].len() * 4))
            .map(|_| u32::MAX)
            .collect();
        let mut heap = BinaryHeap::new();

        //Add initiale states
        for dir in [Direction::East, Direction::South] {
            let states = self.state_from_dir(Position::new(0, 0), dir, 0, min_length, max_length);
            for state in states {
                dist[((state.pos.y * self.grid.len()) + state.pos.x) * 4
                    + state.direction.usize()] = state.heat_loss;
                heap.push(state);
            }
        }

        while let Some(state) = heap.pop() {
            if state.heat_loss
                > dist[((state.pos.y * self.grid.len()) + state.pos.x) * 4
                    + state.direction.usize()]
            {
                continue;
            }

            let next_states =
                [state.direction.turn_left(), state.direction.turn_right()].map(|dir| {
                    self.state_from_dir(state.pos, dir, state.heat_loss, min_length, max_length)
                });

            for next_state in next_states.iter().flatten() {
                if next_state.heat_loss
                    < dist[((next_state.pos.y * self.grid.len()) + next_state.pos.x) * 4
                        + next_state.direction.usize()]
                {
                    dist[((next_state.pos.y * self.grid.len()) + next_state.pos.x) * 4
                        + next_state.direction.usize()] = next_state.heat_loss;
                    heap.push(*next_state);
                }
            }
        }
        let goal = ((self.grid.len() * self.grid[0].len()) - 1) * 4;
        *dist[goal..(goal + 4)].iter().min().unwrap() as u64
    }
}

#[cfg(test)]
mod tests_day_17 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-17/test.txt");
        assert_eq!(part_1(input), 102);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-17/test.txt");
        assert_eq!(part_2(input), 94);
    }
}

