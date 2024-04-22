fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-10/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut map = Map::new(input);
    map.set_start_open_directions();

    let path_length = map.define_loop();
    (path_length - 1) / 2
}

fn part_2(input: &str) -> u32 {
    let mut map = Map::new(input);
    map.set_start_open_directions();
    map.define_loop();
    map.count_tiles_inside()
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}
impl Direction {
    pub fn is_vertical(&self) -> bool {
        *self == Self::North || *self == Self::South
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Pipe {
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}
impl Pipe {
    pub fn new(c: char) -> Self {
        match c {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid pipe"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Tile {
    pipe: Pipe,
    x: u32,
    y: u32,
    loop_dir: Option<Direction>,
    open_directions: [Direction; 2],
}
impl Tile {
    pub fn new(pipe: Pipe, x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            loop_dir: None,
            open_directions: match &pipe {
                Pipe::Horizontal => [Direction::East, Direction::West],
                Pipe::Vertical => [Direction::North, Direction::South],
                Pipe::NorthEast => [Direction::North, Direction::East],
                Pipe::NorthWest => [Direction::North, Direction::West],
                Pipe::SouthEast => [Direction::South, Direction::East],
                Pipe::SouthWest => [Direction::South, Direction::West],
                Pipe::Ground => [Direction::None, Direction::None],
                Pipe::Start => [Direction::None, Direction::None],
            },
            pipe,
        }
    }

    pub fn is_coord_adjacent(&self, coord: (&u32, &u32)) -> bool {
        let (x, y) = coord;
        (*x == self.x && self.y > 0 && *y == self.y - 1)
            || (*x == self.x + 1 && *y == self.y)
            || (*x == self.x && *y == self.y + 1)
            || (self.x > 0 && *x == self.x - 1 && *y == self.y)
    }

    pub fn coord_from_direction(&self, direction: &Direction) -> (u32, u32) {
        match direction {
            Direction::North => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
            Direction::None => (self.x, self.y),
        }
    }
    pub fn relative_direction_from_coord(&self, coord: (&u32, &u32)) -> Direction {
        match coord {
            (x, y) if self.y != 0 && (*x == self.x && *y == self.y - 1) => Direction::North,
            (x, y) if *x == self.x + 1 && *y == self.y => Direction::East,
            (x, y) if *x == self.x && *y == self.y + 1 => Direction::South,
            (x, y) if self.x != 0 && (*x == self.x - 1 && *y == self.y) => Direction::West,
            _ => Direction::None,
        }
    }

    pub fn is_open_to_coord(&self, coord: (&u32, &u32)) -> bool {
        let (x, y) = coord;
        let direction = self.relative_direction_from_coord((x, y));
        self.open_directions.contains(&direction)
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}
impl Map {
    pub fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            height += 1;
            width = 0;
            for (x, c) in line.chars().enumerate() {
                width += 1;
                tiles.push(Tile::new(Pipe::new(c), x as u32, y as u32));
            }
        }
        Self {
            tiles,
            width: width as u32,
            height: height as u32,
        }
    }

    fn set_start_open_directions(&mut self) {
        let tiles_copy = self.tiles.clone();
        let start = self
            .tiles
            .iter_mut()
            .find(|t| t.pipe == Pipe::Start)
            .unwrap();
        let adjacent_tiles: Vec<&Tile> = tiles_copy
            .iter()
            .filter(|t| start.is_coord_adjacent((&t.x, &t.y)))
            .collect();

        for adjacent_tile in adjacent_tiles.iter() {
            if adjacent_tile.is_open_to_coord((&start.x, &start.y)) {
                if start.open_directions[0] == Direction::None {
                    start.open_directions[0] =
                        start.relative_direction_from_coord((&adjacent_tile.x, &adjacent_tile.y));
                    if start.open_directions[0] == Direction::North
                        || start.open_directions[0] == Direction::South
                    {
                        start.loop_dir = Some(start.open_directions[0]);
                    }
                } else {
                    start.open_directions[1] =
                        start.relative_direction_from_coord((&adjacent_tile.x, &adjacent_tile.y))
                }
            }
        }
    }

    pub fn define_loop(&mut self) -> u32 {
        let self_copy = self.clone();
        let start = self_copy
            .tiles
            .iter()
            .find(|t| t.pipe == Pipe::Start)
            .unwrap();

        let mut current_tile = start;
        let mut next_dir = &start.open_directions[0];
        let mut last_next_dir = next_dir;
        let mut path_length = 1;
        loop {
            if next_dir.is_vertical() {
                self.get_tile_mut(current_tile.x, current_tile.y)
                    .unwrap()
                    .loop_dir = Some(*next_dir);
            } else {
                self.get_tile_mut(current_tile.x, current_tile.y)
                    .unwrap()
                    .loop_dir = Some(*last_next_dir);
            }
            last_next_dir = next_dir;
            let next_coord = current_tile.coord_from_direction(next_dir);
            let next_tile = self_copy.get_tile(next_coord.0, next_coord.1).unwrap();

            path_length += 1;
            next_dir = next_tile
                .open_directions
                .iter()
                .find(|dir| {
                    **dir
                        != next_tile
                            .relative_direction_from_coord((&current_tile.x, &current_tile.y))
                })
                .unwrap();
            current_tile = next_tile;
            if current_tile.pipe == Pipe::Start {
                if next_dir.is_vertical() {
                    self.get_tile_mut(current_tile.x, current_tile.y)
                        .unwrap()
                        .loop_dir = Some(*next_dir);
                } else {
                    self.get_tile_mut(current_tile.x, current_tile.y)
                        .unwrap()
                        .loop_dir = Some(*last_next_dir);
                }
                break;
            }
        }
        path_length
    }

    pub fn count_tiles_inside(&self) -> u32 {
        let mut tile_count = 0;
        for h in 0..self.height {
            let mut is_inside = false;
            let mut last_loop_dir = Direction::None;
            for i in 0..self.width {
                let tile = self.get_tile(i, h).unwrap();
                if tile.loop_dir.is_some() {
                    if !tile.loop_dir.unwrap().is_vertical() {
                        continue;
                    }
                    if last_loop_dir != tile.loop_dir.unwrap() {
                        is_inside = !is_inside;
                    }
                    last_loop_dir = tile.loop_dir.unwrap();
                } else if is_inside {
                    tile_count += 1;
                }
            }
        }
        tile_count
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles
            .get(x as usize + y as usize * self.width as usize)
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        self.tiles
            .get_mut(x as usize + y as usize * self.width as usize)
    }
}

#[cfg(test)]
mod test_day10 {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-10/test.txt");
        assert_eq!(part_1(input), 4);

        let input = include_str!("../../aoc-2023-inputs/day-10/test2.txt");
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-10/test3.txt");
        assert_eq!(part_2(input), 4);

        let input = include_str!("../../aoc-2023-inputs/day-10/test4.txt");
        assert_eq!(part_2(input), 8);

        let input = include_str!("../../aoc-2023-inputs/day-10/test5.txt");
        assert_eq!(part_2(input), 10);
    }
}
