use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
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

    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false,
        }
    }
    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: usize,
}
impl Instruction {
    pub fn new(direction: Direction, steps: usize) -> Self {
        Self { direction, steps }
    }
    pub fn from_str(s: &str) -> Vec<Self> {
        let re = Regex::new(r"([URDL])\s([0-9]+)\s\(#(.*)\)").unwrap();
        re
            .captures_iter(s)
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
        re
            .captures_iter(s)
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
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    start: Node,
    end: Node,
    direction: Direction,
}
impl Line {
    pub fn new(start: &Node, end: &Node) -> Self {
        let direction = if start.x == end.x {
            if start.y < end.y {
                Direction::Down
            } else {
                Direction::Up
            }
        } else {
            if start.x < end.x {
                Direction::Right
            } else {
                Direction::Left
            }
        };
        Self { start: *start, end: *end, direction }
    }
    pub fn start_lower(&self) -> Node {
        if self.start.y < self.end.y {
            self.start
        } else {
            self.end
        }
    }
    pub fn end_higher(&self) -> Node {
        if self.start.y > self.end.y {
            self.start
        } else {
            self.end
        }
    }
    pub fn cmp_vertical(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.start_lower().y.cmp(&other.start_lower().y);
        match ord {
            std::cmp::Ordering::Equal => self.start_lower().x.cmp(&other.start_lower().x),
            _ => ord,
        }
    }
} 
#[derive(Debug, Clone)]
struct Map {
    lines: Vec<Line>,
}
impl Map {
    pub fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        let mut lines: Vec<Line> = vec![];
        let mut current_x = 0;
        let mut current_y = 0;
        for instruction in instructions.iter() {
            let start = Node::new(current_x, current_y);
            instruction.direction.step_coord(&mut current_x, &mut current_y, instruction.steps);
            let end = Node::new(current_x, current_y);
            lines.push(Line::new(&start, &end));
        }
        Map { lines: lines }
    }

    pub fn count_volume(&self) -> u64 {
        let mut vertical_lines: Vec<Line> = self.lines.iter().filter(|l| l.direction.is_vertical()).cloned().collect();
        vertical_lines.sort_by(|a, b| a.cmp_vertical(&b));
        let horizontal_lines_y: Vec<i64> = self.lines.iter().filter(|l| l.direction.is_horizontal()).map(|l| l.start_lower().y).collect();
        let higher_y = vertical_lines.iter().map(|l| l.end_higher().y).max().unwrap();
        let lowest_y = vertical_lines.iter().map(|l| l.start_lower().y).min().unwrap();
        let mut current_y = lowest_y;
        let mut volume: u64 = 0;
        let mut is_line_horizontal;
        loop {
            is_line_horizontal = horizontal_lines_y.contains(&current_y);
            let min_higher_end = vertical_lines.iter().map(|l| l.end_higher().y).filter(|n| *n > current_y).min().unwrap_or(i64::MAX);
            let min_lowest_start = vertical_lines.iter().map(|l| l.start_lower().y).filter(|y| *y > current_y).min().unwrap_or(i64::MAX);
            let mut next_line = match std::cmp::min(min_higher_end, min_lowest_start).cmp(&i64::MAX) {
                std::cmp::Ordering::Equal => higher_y+1,
                _ => std::cmp::min(min_higher_end, min_lowest_start),
            };
            if is_line_horizontal {
               next_line = current_y + 1; 
            }

            let mut lines_to_check = vertical_lines
                .iter()
                .filter(|l| l.start_lower().y <= current_y && l.end_higher().y >= current_y)
                .collect::<Vec<&Line>>();
            lines_to_check.sort_by(|a, b| a.start_lower().x.cmp(&b.start_lower().x));

            let mut previous_dir = Direction::Right;
            let mut is_inside = false;
            let mut previous_x = i64::MIN;
            for line in 0..lines_to_check.len() {
                if is_inside || previous_dir == lines_to_check[line].direction {
                    let mut width = (lines_to_check[line].start_lower().x - lines_to_check[line-1].start_lower().x).abs() as u64; 
                    if previous_x == lines_to_check[line-1].start_lower().x {
                        width -= 1;
                    }
                    previous_x = lines_to_check[line].start_lower().x;
                    let height = (current_y - next_line).abs() as u64;
                    volume += (width+1) * height;
                }
                if previous_dir != lines_to_check[line].direction {
                    is_inside = !is_inside;
                }
                previous_dir = lines_to_check[line].direction;
            }

            // remove all lines that are lower than current_y
            vertical_lines = vertical_lines.into_iter().filter(|l| l.end_higher().y >= current_y).collect();
            if current_y == higher_y {
                break;
            }
            current_y = next_line;
        }
        volume
    }
}

#[cfg(test)]
mod tests_day17 {
    use crate::*;

    #[test]
    fn test_part_lines() {
        let input = include_str!("./test.txt");
        let instructions = Instruction::from_str(input);
        let map = Map::from_instructions(&instructions);
        assert_eq!(map.lines.len(), 14);
        assert_eq!(map.lines[0].start, Node::new(0, 0));
        assert_eq!(map.lines[0].end, Node::new(6, 0));
    }

    #[test]
    fn test_volume_count() {
        let input = include_str!("./test2.txt");
        let instr = Instruction::from_str(input);
        let map = Map::from_instructions(&instr);
        assert_eq!(map.count_volume(), 49);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 62);
    }

    #[test]
    fn test_part_color_correction() {
        let input = include_str!("./test.txt");
        let instructions = Instruction::from_str_color_correction(input);
        assert_eq!(instructions.len(), 14);
        assert_eq!(instructions[0].direction, Direction::Right);
        assert_eq!(instructions[0].steps, 461937);
        assert_eq!(instructions[13].direction, Direction::Up);
        assert_eq!(instructions[13].steps, 500254);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input), 952408144115);
    }
}
