use std::cmp::min;

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../../aoc-2023-inputs/day-22/input.txt");

    let mut bricks = parse_bricks(input);
    let _ = apply_gravity_bricks(&mut bricks);
    let safe_bricks = find_safe_bricks(&bricks);
    println!("part_1={}", safe_bricks.len());

    let non_safe_bricks = bricks
        .iter()
        .filter(|x| !safe_bricks.contains(x))
        .collect::<Vec<_>>();
    let part_2: usize = non_safe_bricks
        .iter()
        .map(|brick_to_remove| {
            let mut bricks = bricks.clone();
            bricks.retain(|x| x.id != brick_to_remove.id);
            apply_gravity_bricks(&mut bricks)
        })
        .sum();
    println!("part_2={}", part_2);
    println!("Time elapsed: {:.2?}", now.elapsed());
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let parts = line
            .split("~")
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let start = Position {
            x: parts[0][0],
            y: parts[0][1],
            z: parts[0][2],
        };
        let end = Position {
            x: parts[1][0],
            y: parts[1][1],
            z: parts[1][2],
        };
        bricks.push(Brick::new(i, start, end));
    }
    bricks.sort_by(|a, b| a.get_lowest_z().cmp(&b.get_lowest_z()));
    bricks
}

fn apply_gravity_bricks(bricks: &mut Vec<Brick>) -> usize {
    let mut moved_bricks = 0;
    for brick_id in 0..bricks.len() {
        let mut has_moved = false;
        loop {
            if bricks[brick_id].get_lowest_z() == 1 {
                break;
            }

            bricks[brick_id].move_down();
            if bricks.iter().any(|other| {
                if other.id == bricks[brick_id].id {
                    false
                } else {
                    bricks[brick_id].is_colliding(other)
                }
            }) {
                bricks[brick_id].move_up();
                break;
            } else {
                has_moved = true;
            }
        }
        if has_moved {
            moved_bricks += 1;
        }
    }
    moved_bricks
}

fn find_safe_bricks(bricks: &Vec<Brick>) -> Vec<&Brick> {
    bricks
        .iter()
        .filter(|brick| {
            let mut b = (*brick).clone();
            b.move_up();

            let bricks_above = bricks
                .iter()
                .filter(|x| {
                    if x.id == b.id {
                        false
                    } else {
                        b.is_colliding(x)
                    }
                })
                .collect::<Vec<_>>();

            bricks_above
                .iter()
                .map(|brick| {
                    let mut b: Brick = (*brick).clone();
                    b.move_down();
                    let mut brick_support = bricks
                        .iter()
                        .map(|x| if b.is_colliding(x) { 1 } else { 0 })
                        .sum::<usize>();
                    if b.axis == Axis::Z {
                        brick_support -= 1;
                    }
                    brick_support
                })
                .all(|x| x > 1)
        })
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}
#[derive(Copy, Clone, Debug)]
struct Brick {
    id: usize,
    axis: Axis,
    start: Position,
    end: Position,
}
impl Brick {
    fn new(id: usize, start: Position, end: Position) -> Self {
        let axis = if start.x != end.x {
            Axis::X
        } else if start.y != end.y {
            Axis::Y
        } else {
            Axis::Z
        };
        Self {
            id,
            axis,
            start,
            end,
        }
    }

    fn get_lowest_z(&self) -> usize {
        min(self.start.z, self.end.z)
    }

    fn move_down(&mut self) {
        if !(self.start.z > 1 && self.end.z > 1) {
            return;
        }

        self.start.z -= 1;
        self.end.z -= 1;
    }

    fn move_up(&mut self) {
        self.start.z += 1;
        self.end.z += 1;
    }

    fn is_colliding(&self, other: &Brick) -> bool {
        (self.start.x <= other.end.x && self.end.x >= other.start.x)
            && (self.start.y <= other.end.y && self.end.y >= other.start.y)
            && (self.start.z <= other.end.z && self.end.z >= other.start.z)
    }
}
impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/* fn part_2(input: &str) -> u64 {
    todo!()
} */

#[cfg(test)]
mod tests_day22 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-22/test.txt");

        let mut bricks = parse_bricks(input);
        apply_gravity_bricks(&mut bricks);
        let safe_bricks = find_safe_bricks(&bricks);
        assert_eq!(safe_bricks.len(), 5);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-22/test.txt");

        let mut bricks = parse_bricks(input);
        apply_gravity_bricks(&mut bricks);
        let safe_bricks = find_safe_bricks(&bricks);

        let non_safe_bricks = bricks
            .iter()
            .filter(|x| !safe_bricks.contains(x))
            .collect::<Vec<_>>();
        let part_2: usize = non_safe_bricks
            .iter()
            .map(|brick_to_remove| {
                let mut bricks = bricks.clone();
                bricks.retain(|x| x.id != brick_to_remove.id);
                apply_gravity_bricks(&mut bricks)
            })
            .sum();
        assert_eq!(part_2, 7);
    }
}
