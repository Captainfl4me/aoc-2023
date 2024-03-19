use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-08/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut lines_iter = input.lines();
    let path = lines_iter.next().unwrap().chars().collect::<Vec<char>>();
    lines_iter.next().unwrap(); // discard second line

    let re = Regex::new(r"([A-Z]{3})").unwrap();
    let mut map: HashMap<&str, Direction> = HashMap::new();
    for line in lines_iter {
        let point_names: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let direction = Direction::new(point_names[1].to_string(), point_names[2].to_string());
        map.insert(point_names[0], direction);
    }

    let mut current_point = "AAA";
    let mut path_index = 0;
    let mut count_iter = 0;
    while current_point != "ZZZ" {
        let direction = map.get(current_point).unwrap();
        if path[path_index] == 'L' {
            current_point = &direction.left;
        } else {
            current_point = &direction.right;
        }
        count_iter += 1;
        path_index = (path_index + 1) % path.len();
    }
    count_iter
}

fn part_2(input: &str) -> u64 {
    let mut lines_iter = input.lines();
    let path = lines_iter.next().unwrap().chars().collect::<Vec<char>>();
    lines_iter.next().unwrap(); // discard second line

    let re = Regex::new(r"([0-9-A-Z]{3})").unwrap();
    let mut map: HashMap<&str, Direction> = HashMap::new();
    for line in lines_iter {
        let point_names: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let direction = Direction::new(point_names[1].to_string(), point_names[2].to_string());
        map.insert(point_names[0], direction);
    }

    let starting_points: Vec<&str> = map
        .keys()
        .filter(|k| (**k).ends_with("A"))
        .map(|a| *a)
        .collect();
    let mut cycle_length: Vec<u64> = Vec::new();
    for start_point in starting_points.iter() {
        let mut current_point = *start_point;
        let mut path_index = 0;
        let mut count_iter = 0;
        loop {
            let direction = map.get(current_point).unwrap();
            if path[path_index] == 'L' {
                current_point = &direction.left;
            } else {
                current_point = &direction.right;
            }
            count_iter += 1;
            path_index = (path_index + 1) % path.len();
            if current_point.ends_with("Z") {
                break;
            }
        }
        cycle_length.push(count_iter);
    }
    cycle_length.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

struct Direction {
    left: String,
    right: String,
}
impl Direction {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests_day08 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-08/test.txt");
        assert_eq!(part_1(input), 6);
    }

    #[test]
    fn test2_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-08/test2.txt");
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-08/test3.txt");
        assert_eq!(part_2(input), 6);
    }
}

