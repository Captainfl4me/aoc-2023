fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    // dbg!(part_2(input));
}

pub struct CustomRange {
    start_src: u64,
    length: u64,
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
}
pub struct CustomMapRange {
    start_src: u64,
    start_dst: u64,
    length: u64
}
impl CustomMapRange {
    pub fn new(start_src: u64, start_dst: u64, length: u64) -> CustomMapRange {
        let cmp = CustomMapRange {
            start_src: start_src,
            start_dst: start_dst,
            length
        };
        cmp
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
    pub fn get_range(&self, range: &CustomRange) -> Option<CustomRange> {
        let self_range = CustomRange::new(self.start_src, self.length);
        let range_src = self_range.intersect(range);
        if range_src.is_some() {
            let range_src_unwrap = range_src.unwrap();
            let distance = range_src_unwrap.start_src() - self.start_src;
            Some(CustomRange { start_src: self.start_dst + distance, length: range_src_unwrap.length })
        } else {
            None
        }
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
}