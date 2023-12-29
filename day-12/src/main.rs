use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    input.lines().map(|l| find_number_of_arr(l)).sum()
}
fn part_2(input: &str) -> u64 {
    input.lines().map(|l| find_number_of_arr2(l)).sum()
}

fn find_number_of_arr(input: &str) -> u64 {
    let input_split = input.split(" ").collect::<Vec<&str>>();
    let checksums: Vec<usize> = input_split[1].split(",").map(|n| n.parse::<usize>().unwrap()).collect();

    let springs_arrangment = input_split[0].chars().collect::<Vec<char>>();
    let mut pos: HashMap<usize, usize> = HashMap::new();
    pos.insert(0, 1);
    for (index, checksum) in checksums.iter().enumerate() {
        let mut new_pos: HashMap<usize, usize> = HashMap::new();
        for (k, v) in pos.iter() {
            let filter_checksums = &checksums[(index+1)..];
            let filter_checksums_sum: usize = filter_checksums.iter().sum();
            let filter_checksums_count: usize = filter_checksums.iter().count();
            let max_length = springs_arrangment.len() + filter_checksums_count - filter_checksums_sum;
            for n in (*k)..max_length {
                if (n + checksum - 1) < springs_arrangment.len() && !springs_arrangment[n..(n + *checksum)].contains(&'.') {
                    if (index == (checksums.len() - 1) && !springs_arrangment[(n + *checksum)..].contains(&'#')) || (index < (checksums.len() - 1) && (n + checksum) < springs_arrangment.len() && springs_arrangment[n + checksum] != '#') {
                        new_pos.insert(n + checksum + 1, if new_pos.contains_key(&(n + checksum + 1)) { new_pos[&(n + checksum + 1)] + *v } else { *v });
                    }
                }
                if springs_arrangment[n] == '#' {
                    break;
                }
            }
        }
        pos = new_pos;
    }
    pos.values().map(|v| *v as u64).sum()
}

fn find_number_of_arr2(input: &str) -> u64 {
    let input_split = input.split(" ").collect::<Vec<&str>>();
    let checksums: Vec<usize> = vec![input_split[1].split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>(); 5]
        .iter()
        .flatten()
        .cloned()
        .collect();

    let springs_arrangment = vec![input_split[0]; 5].join(&"?").chars().collect::<Vec<char>>();
    let mut pos: HashMap<usize, usize> = HashMap::new();
    pos.insert(0, 1);
    for (index, checksum) in checksums.iter().enumerate() {
        let mut new_pos: HashMap<usize, usize> = HashMap::new();
        for (k, v) in pos.iter() {
            let filter_checksums = &checksums[(index+1)..];
            let filter_checksums_sum: usize = filter_checksums.iter().sum();
            let filter_checksums_count: usize = filter_checksums.iter().count();
            let max_length = springs_arrangment.len() + filter_checksums_count - filter_checksums_sum;
            for n in (*k)..max_length {
                if (n + checksum - 1) < springs_arrangment.len() && !springs_arrangment[n..(n + *checksum)].contains(&'.') {
                    if (index == (checksums.len() - 1) && !springs_arrangment[(n + *checksum)..].contains(&'#')) || (index < (checksums.len() - 1) && (n + checksum) < springs_arrangment.len() && springs_arrangment[n + checksum] != '#') {
                        new_pos.insert(n + checksum + 1, if new_pos.contains_key(&(n + checksum + 1)) { new_pos[&(n + checksum + 1)] + *v } else { *v });
                    }
                }
                if springs_arrangment[n] == '#' {
                    break;
                }
            }
        }
        pos = new_pos;
    }
    pos.values().map(|v| *v as u64).sum()
}

#[cfg(test)]
mod test_day12 {
    use crate::*;

    #[test]
    fn test_arrangment(){
        assert_eq!(find_number_of_arr("???.### 1,1,3"), 1);  
        assert_eq!(find_number_of_arr(".??..??...?##. 1,1,3"), 4);  
        assert_eq!(find_number_of_arr("?#?#?#?#?#?#?#? 1,3,1,6"), 1);  
        assert_eq!(find_number_of_arr("????.#...#... 4,1,1"), 1);  
        assert_eq!(find_number_of_arr("????.######..#####. 1,6,5"), 4);  
        assert_eq!(find_number_of_arr("?###???????? 3,2,1"), 10);  
    }
    
    #[test]
    fn test_arrangment2(){
        assert_eq!(find_number_of_arr2("???.### 1,1,3"), 1);  
        assert_eq!(find_number_of_arr2(".??..??...?##. 1,1,3"), 16384);  
        assert_eq!(find_number_of_arr2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);  
        assert_eq!(find_number_of_arr2("????.#...#... 4,1,1"), 16);  
        assert_eq!(find_number_of_arr2("????.######..#####. 1,6,5"), 2500);  
        assert_eq!(find_number_of_arr2("?###???????? 3,2,1"), 506250);  
    }
}