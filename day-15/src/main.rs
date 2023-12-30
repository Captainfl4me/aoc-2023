fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    input.split(',').map(|w| hash(w)).sum()
}

fn part_2(input: &str) -> u64 {
    let mut box_: Vec<Box> = Vec::with_capacity(256);
    for _ in 0..256 { box_.push(Box::new()); }

    for s in input.split(',') {
        if s.contains('-') {
            let label = s.replace('-', "");
            let index = hash(label.as_str()) as usize;
            box_[index].remove_lens(label.as_str());
        } else {
            let sp_arr: Vec<&str> = s.split('=').collect();
            let lens = Lens::new(sp_arr[0], sp_arr[1].parse::<u8>().unwrap());
            let hash = hash(lens.label.as_str());
            let index = hash as usize;
            box_[index].add_lens(lens);
        }
    }

    box_.iter().enumerate().map(|(i, b)| {
        b.lenses.iter().enumerate().map(|(j, l)| {
            (i + 1) * (j + 1) * l.focal_length as usize
        }).sum::<usize>()
    }).sum::<usize>() as u64
}

fn hash(input: &str) -> u64 {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as u64;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}
impl Lens {
    fn new(label: &str, focal_length: u8) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }
}
#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}
impl Box {
    fn new() -> Self {
        Self { lenses: vec![] }
    }
    fn add_lens(&mut self, lens: Lens) {
        let lens_with_label = self.lenses.iter_mut().filter(|l| l.label == lens.label).next();
        if lens_with_label.is_some() {
            lens_with_label.unwrap().focal_length = lens.focal_length;
        } else {
            self.lenses.push(lens);
        }
    }
    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != label);
    }
}

#[cfg(test)]
mod tests_day15 {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("pc-"), 48);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input), 145);
    }
}
