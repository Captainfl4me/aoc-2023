use regex::Regex;

//Shoelace formula
fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-18/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let instructions = Instruction::from_str(input);
    let map = Map::from_instructions(&instructions);
    map.count_volume()
}

fn part_2(input: &str) -> u64 {
    let instructions = Instruction::from_str_color_correction(input);
    let map = Map::from_instructions(&instructions);
    map.count_volume()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn step_coord(&self, x: &mut i64, y: &mut i64, length: usize) {
        match self {
            Direction::Up => *y -= length as i64,
            Direction::Right => *x += length as i64,
            Direction::Down => *y += length as i64,
            Direction::Left => *x -= length as i64,
        }
    }
}
#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: usize,
}
impl Instruction {
    pub fn from_str(s: &str) -> Vec<Self> {
        let re = Regex::new(r"([URDL])\s([0-9]+)\s\(#(.*)\)").unwrap();
        re.captures_iter(s)
            .map(|f| f.extract())
            .map(|(_, [dir, len, _])| Instruction {
                direction: match dir {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => panic!("Invalid direction"),
                },
                steps: len.parse().unwrap(),
            })
            .collect()
    }
    pub fn from_str_color_correction(s: &str) -> Vec<Self> {
        let re = Regex::new(r"([URDL])\s([0-9]+)\s\(#(.*)\)").unwrap();
        re.captures_iter(s)
            .map(|f| f.extract())
            .map(|(_, [_, _, color])| Instruction {
                direction: match color.chars().nth(5).unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!("Invalid direction"),
                },
                steps: usize::from_str_radix(&color[0..5], 16).unwrap(),
            })
            .collect()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    x: i64,
    y: i64,
}
impl Node {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn manahttan_distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}
#[derive(Debug, Clone)]
struct Map {
    nodes: Vec<Node>,
}
impl Map {
    pub fn from_instructions(instructions: &[Instruction]) -> Self {
        let mut nodes: Vec<Node> = vec![];
        let mut current_x = 0;
        let mut current_y = 0;
        for instruction in instructions.iter() {
            nodes.push(Node::new(current_x, current_y));
            instruction
                .direction
                .step_coord(&mut current_x, &mut current_y, instruction.steps);
        }
        Map { nodes }
    }

    pub fn count_volume(&self) -> u64 {
        let (area, perimeter) =
            self.nodes
                .iter()
                .enumerate()
                .fold((0, 0), |(sum, perimeter), (i, node1)| {
                    let node2 = &self.nodes[(i + 1) % self.nodes.len()];
                    let new_perimeter = perimeter + node1.manahttan_distance(node2);
                    let new_sum = sum + node1.x * node2.y - node1.y * node2.x;
                    (new_sum, new_perimeter)
                });
        ((area.unsigned_abs() + perimeter) / 2) + 1
    }
}

#[cfg(test)]
mod tests_day18 {
    use crate::*;

    #[test]
    fn test_volume_count() {
        let input = include_str!("../../aoc-2023-inputs/day-18/test2.txt");
        let instr = Instruction::from_str(input);
        let map = Map::from_instructions(&instr);
        assert_eq!(map.count_volume(), 49);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-18/test.txt");
        assert_eq!(part_1(input), 62);
    }

    #[test]
    fn test_part_color_correction() {
        let input = include_str!("../../aoc-2023-inputs/day-18/test.txt");
        let instructions = Instruction::from_str_color_correction(input);
        assert_eq!(instructions.len(), 14);
        assert_eq!(instructions[0].direction, Direction::Right);
        assert_eq!(instructions[0].steps, 461937);
        assert_eq!(instructions[13].direction, Direction::Up);
        assert_eq!(instructions[13].steps, 500254);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-18/test.txt");
        assert_eq!(part_2(input), 952408144115);
    }
}
