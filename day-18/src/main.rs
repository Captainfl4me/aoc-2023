use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    // dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let instructions = Instruction::from_str(input);
    let mut map = Map::from_instructions(&instructions);
    map.fill_loop();
    map.map.iter().map(|row| row.iter().map(|t| t.tile_type != TileType::Empty).filter(|b| *b).count() as u32).sum()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}
impl Direction {
    pub fn step_coord(&self, x: &mut i32, y: &mut i32) {
        match self {
            Direction::Up => *y -= 1,
            Direction::Right => *x += 1,
            Direction::Down => *y += 1,
            Direction::Left => *x -= 1,
            Direction::None => (),
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
    steps: u32,
    color: String,
}
impl Instruction {
    pub fn from_str(s: &str) -> Vec<Self> {
        let re = Regex::new(r"([URDL])\s([0-9]+)\s\(#(.*)\)").unwrap();
        re
            .captures_iter(s)
            .map(|f| f.extract())
            .map(|(_, [dir, len, color])| Instruction {
                direction: match dir {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => panic!("Invalid direction"),
                },
                steps: len.parse().unwrap(),
                color: color.to_string(),
            })
            .collect()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Inside
}
#[derive(Debug, Clone)]
struct Tile {
    direction: Direction,
    color: String,
    tile_type: TileType,
}
impl Tile {
    pub fn new(direction: Direction, color: String, tile_type: TileType) -> Self {
        Self {
            direction,
            color,
            tile_type,
        }
    }
}
#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Tile>>,
}
impl Map {
    pub fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        let mut map = vec![vec![Tile::new(Direction::None, "".to_string(), TileType::Empty); 1]; 1];
        let mut current_x = 0;
        let mut current_y = 0;
        let mut last_direction = Direction::None;
        for instruction in instructions.iter() {
            for _ in 0..instruction.steps {
                if current_y < 0 {
                    map.insert(0, vec![Tile::new(Direction::None, "".to_string(), TileType::Empty); map[0].len()]);
                    current_y = 0;
                } else if current_y >= map.len() as i32 {
                    map.push(vec![Tile::new(Direction::None, "".to_string(), TileType::Empty); map[0].len()]);
                }
                
                if current_x < 0 {
                    for row in map.iter_mut() {
                        row.insert(0, Tile::new(Direction::None, "".to_string(), TileType::Empty));
                    }
                    current_x = 0;
                } else if current_x >= map[0].len() as i32 {
                    for row in map.iter_mut() {
                        row.push(Tile::new(Direction::None, "".to_string(), TileType::Empty));
                    }
                }
                let mut dir = instruction.direction;
                if last_direction.is_vertical() && dir.is_horizontal() {
                    dir = last_direction;
                }
                
                map[current_y as usize][current_x as usize] = Tile::new(dir, instruction.color.clone(), TileType::Wall);
                instruction.direction.step_coord(&mut current_x, &mut current_y);
                last_direction = instruction.direction;
            }
        }
        if last_direction.is_vertical() {
            map[current_y as usize][current_x as usize] = Tile::new(last_direction, "".to_string(), TileType::Inside);
        }
        Map { map }
    }

    pub fn fill_loop(&mut self) {
        for h in 0..self.map.len(){
            let mut is_inside = false;
            let mut last_loop_dir = Direction::None;
            for i in 0..self.map[0].len() {
                let tile = &self.map[h][i];
                if tile.direction != Direction::None {
                    if !tile.direction.is_vertical() {
                        continue;
                    }
                    if last_loop_dir != tile.direction {
                        is_inside = !is_inside;
                    }
                    last_loop_dir = tile.direction;
                } else {
                    if is_inside {
                        self.map[h][i].tile_type = TileType::Inside;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_day17 {
    use crate::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 62);
    }
}
