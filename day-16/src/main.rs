fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-16/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let mut map: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(TileType::new(c))).collect())
        .collect();

    let mut beams = vec![Beam::new(0, 0, BeamDirection::Right)];
    loop {
        if beams.is_empty() {
            break;
        }
        let mut new_beams = vec![];
        for beam in beams {
            if map[beam.y as usize][beam.x as usize].already_been_there(&beam) {
                continue;
            }
            let mut ap_tfr = map[beam.y as usize][beam.x as usize].apply_transform(&beam);
            ap_tfr.iter_mut().for_each(|b| b.apply_dir());
            ap_tfr
                .iter()
                .filter(|b| {
                    b.x >= 0 && b.y >= 0 && b.x < map[0].len() as i32 && b.y < map.len() as i32
                })
                .for_each(|b| new_beams.push(*b));
        }
        beams = new_beams;
    }
    map.iter().flatten().filter(|t| t.is_energized).count() as u64
}

fn part_2(input: &str) -> u64 {
    let mut max_energy = 0;

    for start_dir in [
        BeamDirection::Up,
        BeamDirection::Down,
        BeamDirection::Left,
        BeamDirection::Right,
    ] {
        for start_x in 0..input.lines().next().unwrap().len() {
            for start_y in 0..input.lines().count() {
                if (start_x != 0 && start_x != input.lines().next().unwrap().len() - 1)
                    && (start_y != 0 && start_y != input.lines().count() - 1)
                {
                    continue;
                }
                let mut map: Vec<Vec<Tile>> = input
                    .lines()
                    .map(|l| l.chars().map(|c| Tile::new(TileType::new(c))).collect())
                    .collect();

                let mut beams = vec![Beam::new(start_x as i32, start_y as i32, start_dir)];
                loop {
                    if beams.is_empty() {
                        break;
                    }
                    let mut new_beams = vec![];
                    for beam in beams {
                        if map[beam.y as usize][beam.x as usize].already_been_there(&beam) {
                            continue;
                        }
                        let mut ap_tfr =
                            map[beam.y as usize][beam.x as usize].apply_transform(&beam);
                        ap_tfr.iter_mut().for_each(|b| b.apply_dir());
                        ap_tfr
                            .iter()
                            .filter(|b| {
                                b.x >= 0
                                    && b.y >= 0
                                    && b.x < map[0].len() as i32
                                    && b.y < map.len() as i32
                            })
                            .for_each(|b| new_beams.push(*b));
                    }
                    beams = new_beams;
                }
                let energy = map.iter().flatten().filter(|t| t.is_energized).count() as u64;
                if energy > max_energy {
                    max_energy = energy;
                }
            }
        }
    }
    max_energy
}

#[derive(PartialEq, Clone, Copy)]
enum BeamDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
struct Beam {
    x: i32,
    y: i32,
    direction: BeamDirection,
}
impl Beam {
    fn new(x: i32, y: i32, dir: BeamDirection) -> Self {
        Self {
            x,
            y,
            direction: dir,
        }
    }
    fn apply_dir(&mut self) {
        match self.direction {
            BeamDirection::Up => self.y -= 1,
            BeamDirection::Down => self.y += 1,
            BeamDirection::Left => self.x -= 1,
            BeamDirection::Right => self.x += 1,
        }
    }
}

enum TileType {
    Empty,
    Horizontal,
    Vertical,
    DashRight,
    DashLeft,
}
impl TileType {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            '/' => Self::DashRight,
            '\\' => Self::DashLeft,
            _ => panic!("Invalid tile type"),
        }
    }
}
struct Tile {
    is_energized: bool,
    tile_type: TileType,
    beam_history: Vec<BeamDirection>,
}
impl Tile {
    fn new(tile_type: TileType) -> Self {
        Self {
            is_energized: false,
            tile_type,
            beam_history: vec![],
        }
    }
    fn already_been_there(&self, beam: &Beam) -> bool {
        self.beam_history.contains(&beam.direction)
    }
    fn apply_transform(&mut self, beam: &Beam) -> Vec<Beam> {
        self.beam_history.push(beam.direction);
        self.is_energized = true;
        match self.tile_type {
            TileType::Empty => vec![*beam],
            TileType::Horizontal => match beam.direction {
                BeamDirection::Up | BeamDirection::Down => vec![
                    Beam::new(beam.x, beam.y, BeamDirection::Left),
                    Beam::new(beam.x, beam.y, BeamDirection::Right),
                ],
                _ => vec![*beam],
            },
            TileType::Vertical => match beam.direction {
                BeamDirection::Left | BeamDirection::Right => vec![
                    Beam::new(beam.x, beam.y, BeamDirection::Up),
                    Beam::new(beam.x, beam.y, BeamDirection::Down),
                ],
                _ => vec![*beam],
            },
            TileType::DashRight => match beam.direction {
                BeamDirection::Right => vec![Beam::new(beam.x, beam.y, BeamDirection::Up)],
                BeamDirection::Left => vec![Beam::new(beam.x, beam.y, BeamDirection::Down)],
                BeamDirection::Down => vec![Beam::new(beam.x, beam.y, BeamDirection::Left)],
                BeamDirection::Up => vec![Beam::new(beam.x, beam.y, BeamDirection::Right)],
            },
            TileType::DashLeft => match beam.direction {
                BeamDirection::Right => vec![Beam::new(beam.x, beam.y, BeamDirection::Down)],
                BeamDirection::Left => vec![Beam::new(beam.x, beam.y, BeamDirection::Up)],
                BeamDirection::Down => vec![Beam::new(beam.x, beam.y, BeamDirection::Right)],
                BeamDirection::Up => vec![Beam::new(beam.x, beam.y, BeamDirection::Left)],
            },
        }
    }
}

#[cfg(test)]
mod tests_day16 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-16/test.txt");
        assert_eq!(part_1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-16/test.txt");
        assert_eq!(part_2(input), 51);
    }
}
