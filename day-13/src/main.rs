use regex::Regex;
use std::cmp::min;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-13/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let re = Regex::new(r"((?:[#.]+\r\n)+[#.]+)(?:\r\n\r\n)?").unwrap();
    let blocks = re
        .captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [s])| s)
        .collect::<Vec<&str>>();
    let mirrors = blocks
        .iter()
        .map(|b| find_mirror(b))
        .collect::<Vec<Mirror>>();

    let mut result = 0;
    result += mirrors
        .iter()
        .filter(|m| m.m_type == MirrorType::Vertical)
        .map(|m| m.pos)
        .sum::<usize>() as u64;
    result += mirrors
        .iter()
        .filter(|m| m.m_type == MirrorType::Horizontal)
        .map(|m| m.pos * 100)
        .sum::<usize>() as u64;
    result
}

fn part_2(input: &str) -> u64 {
    let re = Regex::new(r"((?:[#.]+\r\n)+[#.]+)(?:\r\n\r\n)?").unwrap();
    let blocks = re
        .captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [s])| s)
        .collect::<Vec<&str>>();
    let mirrors = blocks
        .iter()
        .map(|b| find_diff_mirror(b))
        .collect::<Vec<Mirror>>();

    let mut result = 0;
    result += mirrors
        .iter()
        .filter(|m| m.m_type == MirrorType::Vertical)
        .map(|m| m.pos)
        .sum::<usize>() as u64;
    result += mirrors
        .iter()
        .filter(|m| m.m_type == MirrorType::Horizontal)
        .map(|m| m.pos * 100)
        .sum::<usize>() as u64;
    result
}

struct Mirror {
    pos: usize,
    m_type: MirrorType,
}
#[derive(Debug, PartialEq)]
enum MirrorType {
    Vertical,
    Horizontal,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_mirror(input: &str) -> Mirror {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut is_vertical = false;
    let mut pos = 0;
    for i in 1..map.len() {
        pos = i;
        for j in 0..min(i, map.len() - i) {
            if map[i + j] == map[i - (j + 1)] {
                is_vertical = true;
                continue;
            } else {
                is_vertical = false;
                break;
            }
        }
        if is_vertical {
            break;
        }
    }
    if is_vertical {
        Mirror {
            pos,
            m_type: MirrorType::Horizontal,
        }
    } else {
        let map = transpose(map);

        let mut is_horizontal = false;
        let mut pos = 0;
        for i in 1..map.len() {
            pos = i;
            for j in 0..min(i, map.len() - i) {
                if map[i + j] == map[i - (j + 1)] {
                    is_horizontal = true;
                    continue;
                } else {
                    is_horizontal = false;
                    break;
                }
            }
            if is_horizontal {
                break;
            }
        }

        Mirror {
            pos,
            m_type: MirrorType::Vertical,
        }
    }
}

fn cmp_diff_mirror(vec1: &Vec<char>, vec2: &Vec<char>) -> bool {
    let mut num_of_diff = 0;
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            num_of_diff += 1;
        }
    }
    num_of_diff == 1
}

fn find_diff_mirror(input: &str) -> Mirror {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut is_vertical = false;
    let mut pos = 0;
    for i in 1..map.len() {
        let mut num_of_diff = 0;
        pos = i;
        for j in 0..min(i, map.len() - i) {
            if map[i + j] == map[i - (j + 1)] {
                is_vertical = true;
            } else if num_of_diff == 0 && cmp_diff_mirror(&map[i + j], &map[i - (j + 1)]) {
                num_of_diff += 1;
                is_vertical = true;
            } else {
                is_vertical = false;
                break;
            }
        }
        if is_vertical {
            if num_of_diff == 1 {
                break;
            }
            is_vertical = false;
        }
    }
    if is_vertical {
        Mirror {
            pos,
            m_type: MirrorType::Horizontal,
        }
    } else {
        let map = transpose(map);

        let mut is_horizontal = false;
        let mut pos = 0;
        for i in 1..map.len() {
            let mut num_of_diff = 0;
            pos = i;
            for j in 0..min(i, map.len() - i) {
                if map[i + j] == map[i - (j + 1)] {
                    is_horizontal = true;
                } else if num_of_diff == 0 && cmp_diff_mirror(&map[i + j], &map[i - (j + 1)]) {
                    num_of_diff += 1;
                    is_horizontal = true;
                } else {
                    is_horizontal = false;
                    break;
                }
            }
            if is_horizontal {
                if num_of_diff == 1 {
                    break;
                }
                is_horizontal = false;
            }
        }

        Mirror {
            pos,
            m_type: MirrorType::Vertical,
        }
    }
}

#[cfg(test)]
mod test_day13 {
    use crate::*;

    #[test]
    fn test_find_mirror() {
        let input = include_str!("../../aoc-2023-inputs/day-13/test.txt");
        let result = find_mirror(input);
        assert_eq!(result.m_type, MirrorType::Vertical);
        assert_eq!(result.pos, 5);

        let input = include_str!("../../aoc-2023-inputs/day-13/test2.txt");
        let result = find_mirror(input);
        assert_eq!(result.m_type, MirrorType::Horizontal);
        assert_eq!(result.pos, 4);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-13/test3.txt");
        let result = part_1(input);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_find_diff_mirror() {
        let input = include_str!("../../aoc-2023-inputs/day-13/test.txt");
        let result = find_diff_mirror(input);
        assert_eq!(result.m_type, MirrorType::Horizontal);
        assert_eq!(result.pos, 3);

        let input = include_str!("../../aoc-2023-inputs/day-13/test2.txt");
        let result = find_diff_mirror(input);
        assert_eq!(result.m_type, MirrorType::Horizontal);
        assert_eq!(result.pos, 1);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-13/test3.txt");
        let result = part_2(input);
        assert_eq!(result, 400);
    }
}

