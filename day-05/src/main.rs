fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CustomRange {
    start_src: u64,
    length: u64,
}
#[derive(Debug)]
pub struct CustomRangeMatch {
    range: CustomRange,
    has_match: bool,
}
impl CustomRange {
    pub fn new(start_src: u64, length: u64) -> CustomRange {
        CustomRange { start_src: start_src, length: length }
    }
    pub fn start_src(&self) -> u64 {
        self.start_src
    }
    pub fn end_src(&self) -> u64 {
        self.start_src + self.length - 1
    }

    pub fn does_intersect(&self, range: &CustomRange) -> bool {
        range.start_src >= self.start_src && range.start_src <= self.end_src()
            || range.end_src() >= self.start_src && range.end_src() <= self.end_src()
            || range.start_src <= self.start_src && range.end_src() >= self.end_src()
    }
    pub fn intersect(&self, range: &CustomRange) -> Option<CustomRange> {
        if self.does_intersect(range) {
            let start = std::cmp::max(self.start_src, range.start_src);
            let end = std::cmp::min(self.end_src(), range.end_src());
            Some(CustomRange { start_src:start, length: end - start + 1 })
        } else {
            None
        }
    }
    pub fn intersect_inverse(&self, range: &CustomRange) -> Vec<CustomRange> {
        let mut not_intersect_vector: Vec<CustomRange> = Vec::new();
        
        if self.start_src < range.start_src {
            let end = std::cmp::min(range.start_src - 1, self.end_src());
            not_intersect_vector.push(CustomRange { start_src: self.start_src, length: end - self.start_src + 1 });
        }
        if self.end_src() > range.end_src() {
            let start = std::cmp::max(self.start_src, range.end_src() + 1);
            not_intersect_vector.push(CustomRange { start_src: start, length: self.end_src() - start + 1 });
        }
        not_intersect_vector
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CustomMapRange {
    start_src: u64,
    start_dst: u64,
    length: u64
}
impl CustomMapRange {
    pub fn new(start_src: u64, start_dst: u64, length: u64) -> CustomMapRange {
        CustomMapRange {
            start_src: start_src,
            start_dst: start_dst,
            length
        }
    }
    pub fn from_str(input: &str) -> CustomMapRange{
        let num_split: Vec<u64> = input.split(" ").map(|f| f.to_string().parse::<u64>().unwrap()).collect();
        CustomMapRange::new(num_split[1], num_split[0], num_split[2])
    }

    pub fn get(&self, num_src: u64) -> Option<u64> {
        if num_src >= self.start_src && num_src < self.start_src+self.length {
            let distance = num_src - self.start_src;
            Some(self.start_dst + distance)
        } else {
            None
        }
    }
    pub fn get_range(&self, range: &CustomRange) -> Vec<CustomRangeMatch> {
        let mut match_vec: Vec<CustomRangeMatch> = Vec::new();
        let self_range = CustomRange::new(self.start_src, self.length);
        let range_src = self_range.intersect(range);
        if range_src.is_some() {
            let range_src_unwrap = range_src.unwrap();
            let distance = range_src_unwrap.start_src() - self.start_src;
            match_vec.push(CustomRangeMatch { range: CustomRange { start_src: self.start_dst + distance, length: range_src_unwrap.length }, has_match: true});
            let unchange_ranges = range.intersect_inverse(&self_range);
            for unchande_range in unchange_ranges.iter() {
                match_vec.push(CustomRangeMatch { range: *unchande_range, has_match: false });
            }
        } else {
            match_vec.push(CustomRangeMatch { range: *range, has_match: false });
        }
        match_vec
    }
}

pub struct CustomMultipleMapRange {
    ranges: Vec<CustomMapRange>
}
impl CustomMultipleMapRange {
    pub fn new() -> CustomMultipleMapRange {
        CustomMultipleMapRange { ranges:Vec::new() }
    }

    pub fn clear(&mut self) {
        self.ranges.clear()
    }

    pub fn add_range(&mut self, new_range: CustomMapRange) {
        self.ranges.push(new_range);
    }

    pub fn get(&self, num_src: u64) -> u64 {
        for range in self.ranges.iter() {
            let check_map = range.get(num_src);
            if check_map.is_some() { return check_map.unwrap(); }
        }
        num_src
    }

    pub fn get_range(&self, range_src_key: &CustomRange) -> Vec<CustomRange> {
        let mut ranges_src = vec![ *range_src_key ];
        let mut res: Vec<CustomRange> = Vec::new();

        let mut new_ranges_src: Vec<CustomRange> = Vec::new();
        let mut match_vec: Vec<CustomRange> = Vec::new();
        loop {
            for range_src in new_ranges_src.iter() {
                let mut has_match = true;
                for range in self.ranges.iter() {
                    let get_range = range.get_range(range_src);
                    if get_range.len() == 1 && get_range[0].has_match == false {
                        has_match = false;
                    } else {
                        for match_range in get_range {
                            if match_range.has_match {
                                res.push(match_range.range);
                            } else {
                                match_vec.push(match_range.range);
                            }
                        }
                        has_match = true;
                        break;
                    }
                }
                if has_match == false {
                    res.push(*range_src);
                } else {
                    ranges_src.append(&mut match_vec);
                }
                match_vec.clear(); // maybe useless!
            }
            new_ranges_src = ranges_src.clone();
            ranges_src.clear();

            if new_ranges_src.len() <= 0 {
                break;
            }
        }

        res
    }
}

fn part_1(input: &str) -> u64{
    let mut input_lines = input.lines();
    let seeds: Vec<u64> = input_lines.next().unwrap().split(":").last().unwrap().trim().split(" ").map(|f| f.parse::<u64>().unwrap()).collect();

    let mut last_src = seeds;

    let mut is_building_map = false;
    let mut current_map = CustomMultipleMapRange::new(); 
    for line in input_lines {
        if line.is_empty() {
            if is_building_map {
                last_src = last_src.iter().map(|f| current_map.get(*f)).collect();
                is_building_map = false;
                current_map.clear();
            }
            continue;
        }else if !is_building_map && line.contains("map") {
            is_building_map = true;
        } else {
            current_map.add_range(CustomMapRange::from_str(line));
        }
    }
    if is_building_map {
        last_src = last_src.iter().map(|f| current_map.get(*f)).collect();
    }

    *last_src.iter().min().unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let mut input_lines = input.lines();
    let seeds: Vec<u64> = input_lines.next().unwrap().split(":").last().unwrap().trim().split(" ").map(|f| f.parse::<u64>().unwrap()).collect();

    let mut seeds_ranges: Vec<CustomRange> = Vec::new();
    let mut i = 0;
    loop {
        if i >= seeds.len() - 1 { break; }
        seeds_ranges.push(CustomRange::new(seeds[i], seeds[i+1]));
        i += 2;
    }

    let mut last_src = seeds_ranges;

    let mut is_building_map = false;
    let mut current_map = CustomMultipleMapRange::new(); 
    for line in input_lines {
        if line.is_empty() {
            if is_building_map {
                last_src = last_src.iter().map(|f| current_map.get_range(f)).flatten().collect();
                is_building_map = false;
                current_map.clear();
            }
            continue;
        }else if !is_building_map && line.contains("map") {
            is_building_map = true;
        } else {
            current_map.add_range(CustomMapRange::from_str(line));
        }
    }
    if is_building_map {
        last_src = last_src.iter().map(|f| current_map.get_range(f)).flatten().collect();
    }

    let start_range: Vec<u64> = last_src.iter().map(|f| f.start_src).collect();
    start_range.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests_day05 {
    use super::*;
    
    #[test]
    fn test_custom_map_range(){
        let map = CustomMapRange::from_str("50 98 2");
        let map2 = CustomMapRange::from_str("52 50 48");
        assert_eq!(map.get(98).unwrap(), 50);
        assert_eq!(map2.get(79).unwrap(), 81);
        assert_eq!(map.get(14).is_none(), true);
        let res = map.get_range(&CustomRange::new(96, 6));
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].range.length, 2);
        assert_eq!(res[1].range.length, 2);
        assert_eq!(res[2].range.length, 2);
    }
    
    #[test]
    fn test_multiple_map_range(){
        let mut map = CustomMultipleMapRange { ranges: Vec::new() };
        map.add_range(CustomMapRange::from_str("50 98 2"));
        map.add_range(CustomMapRange::from_str("52 50 48"));
        assert_eq!(map.get(55), 57);
        assert_eq!(map.get(79), 81);
        assert_eq!(map.get(14), 14);
        assert_eq!(map.get(13), 13);
    }

    #[test]
    fn test_part1(){
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 35);
    }
    
    #[test]
    fn test_range_get(){
        let mut map = CustomMultipleMapRange { ranges: Vec::new() };
        map.add_range(CustomMapRange::from_str("50 90 5"));
        map.add_range(CustomMapRange::from_str("55 95 5"));
        let res = map.get_range(&CustomRange { start_src: 90, length: 10 });
        assert_eq!(res.len(), 2);
        let res = map.get_range(&CustomRange { start_src: 88, length: 14 });
        assert_eq!(res.len(), 4);
        assert_eq!(res[0].start_src, 50);
        let res = map.get_range(&CustomRange { start_src: 92, length: 10 });
        assert_eq!(res[0].start_src, 52);
    }

    #[test]
    fn test_part2(){
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input), 46);
    }
}