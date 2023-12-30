use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input, 1000000000));
}

fn part_1(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    map = tilt_map_north(map);
    
    map.iter().enumerate().map(|(i, row)| {
        row.iter().filter(|c| **c=='O').count() as u64 * (map.len() - i) as u64
    }).sum::<u64>()
}

fn part_2(input: &str, cycle: usize) -> u64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cache_map: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();

    let mut cycle_map: Vec<Vec<Vec<char>>> = Vec::new();
    let mut start_cycle_index = 0;
    for i in 0..cycle {
        if cache_map.contains_key(&map) {
            if cycle_map.len() <= 0 {
                start_cycle_index = i;
            } else {
                if map == *cycle_map.first().unwrap() {
                    break;
                }
            }
            cycle_map.push(map.clone());
            map = cache_map.get(&map).unwrap().clone();
        } else {
            let new_map = tilt_map_east(tilt_map_south(tilt_map_west(tilt_map_north(map.clone()))));
            cache_map.insert(map.clone(), new_map.clone());
            map = new_map;
        }
    }
    let cycle_index = (cycle - start_cycle_index) % cycle_map.len();

    cycle_map[cycle_index].iter().enumerate().map(|(i, row)| {
        row.iter().filter(|c| **c=='O').count() as u64 * (map.len() - i) as u64
    }).sum::<u64>()
}

fn tilt_map_north(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    let mut lowest_location_north = vec![0; map[0].len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '#' => lowest_location_north[j] = i+1,
                'O' => {
                    map[i][j] = '.';
                    map[lowest_location_north[j]][j] = 'O';
                    lowest_location_north[j] += 1;
                },
                _ => {}
            }
        }
    }
    map
}
fn tilt_map_west(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let map = transpose(map);
    transpose(tilt_map_north(map))
}
fn tilt_map_south(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    let mut lowest_location_north = vec![map.len()-1; map[0].len()];
    for i in (0..map.len()).rev() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '#' => lowest_location_north[j] = if i > 0 { i-1 } else { 0 },
                'O' => {
                    map[i][j] = '.';
                    map[lowest_location_north[j]][j] = 'O';
                    if lowest_location_north[j] > 0 {
                        lowest_location_north[j] -= 1;
                    }
                },
                _ => {}
            }
        }
    }
    map
}
fn tilt_map_east(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let map = transpose(map);
    transpose(tilt_map_south(map))
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

#[cfg(test)]
mod test_day14 {
    use crate::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        let result = part_1(input);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        let result = part_2(input, 1000000000);
        assert_eq!(result, 64);
    }
}
