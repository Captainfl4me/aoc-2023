use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    // dbg!(part_2(input));
}

fn part_1(input: &str) -> u64{
    let mut input_lines = input.lines();
    let seeds: Vec<u64> = input_lines.next().unwrap().split(":").last().unwrap().trim().split(" ").map(|f| f.parse::<u64>().unwrap()).collect();

    let mut last_src = seeds;

    let mut is_building_map = false;
    let mut current_map : HashMap<u64, u64> = HashMap::new(); 
    for line in input_lines {
        if line.is_empty() {
            if is_building_map {
                last_src = last_src.iter().map(|f| match current_map.get(f) {
                    Some(val) => { *val },
                    None => { *f }
                }).collect();
                is_building_map = false;
                current_map.clear();
            }
            continue;
        }else if !is_building_map && line.contains("map") {
            is_building_map = true;
        } else {
            current_map.extend(create_sub_source_destination_map(line).into_iter());
        }
    }
    if is_building_map {
        last_src = last_src.iter().map(|f| match current_map.get(f) {
            Some(val) => { *val },
            None => { *f }
        }).collect();
    }

    *last_src.iter().min().unwrap()
}

fn create_sub_source_destination_map(input: &str) -> HashMap<u64, u64> {
    let num_split: Vec<u64> = input.split(" ").map(|f| f.to_string().parse::<u64>().unwrap()).collect();
    let mut res_map: HashMap<u64, u64> = HashMap::new();

    let dest_start = num_split[0];
    let src_start = num_split[1];
    let map_length = num_split[2];

    for i in 0..map_length {
        res_map.insert(src_start+i, dest_start+i);
    } 

    res_map
}


#[cfg(test)]
mod tests_day05 {
    use super::*;
    
    #[test]
    fn test_sub_create_source_destination_map(){
        let mut map = create_sub_source_destination_map("50 98 2");
        let map2 = create_sub_source_destination_map("52 50 48");
        map.extend(map2.into_iter());
        assert_eq!(*map.get(&(55 as u64)).unwrap(), 57);
        assert_eq!(*map.get(&(79 as u64)).unwrap(), 81);
        assert_eq!(map.get(&(14 as u64)).is_none(), true);
    }

    #[test]
    fn test_part1(){
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 35);
    }
}