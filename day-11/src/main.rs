fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input, 1000000));
}

fn part_1(input: &str) -> u64 {
    let mut map = Map::new(input);
    map.replace_expansion();

    let galaxies = map.find_all_galaxy();
    let mut distance_sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            distance_sum += distance(galaxy, &galaxies[j]);
        }
    }
    distance_sum
}

fn part_2(input: &str, expansion: u32) -> u64 {
    let map = Map::new(input);

    let galaxies = map.find_all_galaxy_with_big_expansion(expansion);
    let mut distance_sum: u64 = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            distance_sum += distance(galaxy, &galaxies[j]);
        }
    }
    distance_sum
}

fn distance(galaxy1: &(usize, usize), galaxy2: &(usize, usize)) -> u64 {
    let (x1, y1) = galaxy1;
    let (x2, y2) = galaxy2;
    let x = (*x1 as i64 - *x2 as i64).abs();
    let y = (*y1 as i64 - *y2 as i64).abs();
    (x + y) as u64
}

#[derive(PartialEq, Clone)]
enum Galaxy {
    Empty,
    Galaxy,
}
impl Galaxy {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid galaxy")
        }
    }
}
struct Map {
    map: Vec<Vec<Galaxy>>,
    width: usize,
    height: usize,
}
impl Map {
    pub fn new(input: &str) -> Self {
        let map: Vec<Vec<Galaxy>> = input.lines()
            .map(|line| line.chars().map(|c| Galaxy::new(c)).collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }

    pub fn detect_empty_line(&self) -> Vec<usize> {
        let mut empty_lines = Vec::new();
        for (y, line) in self.map.iter().enumerate() {
            if line.iter().all(|galaxy| *galaxy == Galaxy::Empty) {
                empty_lines.push(y);
            }
        }
        empty_lines
    }
    pub fn detect_empty_column(&self) -> Vec<usize> {
        let mut empty_columns = Vec::new();
        for x in 0..self.width {
            if self.map.iter().all(|line| line[x] == Galaxy::Empty) {
                empty_columns.push(x);
            }
        }
        empty_columns
    }

    pub fn replace_expansion(&mut self) {
        let empty_lines = self.detect_empty_line();
        let empty_lines = empty_lines
            .iter()
            .enumerate()
            .map(|(i, y)| (i, y + i));

        for (_, index) in empty_lines {
            self.map.insert(index, vec![Galaxy::Empty; self.width]);
        }
        self.height = self.map.len();

        let empty_columns = self.detect_empty_column();
        let empty_columns = empty_columns
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x + i));

        for (_, index) in empty_columns {
            for line in self.map.iter_mut() {
                line.insert(index, Galaxy::Empty);
            }
        }
    }

    pub fn find_all_galaxy(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for (y, line) in self.map.iter().enumerate() {
            for (x, galaxy) in line.iter().enumerate() {
                if *galaxy == Galaxy::Galaxy {
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }
    
    pub fn find_all_galaxy_with_big_expansion(&self, expansion: u32) -> Vec<(usize, usize)> {
        let expansion = expansion - 1;
        let empty_lines = self.detect_empty_line();
        let empty_columns = self.detect_empty_column();
        let mut galaxies = Vec::new();

        let mut number_of_line_cross = 0;
        for (y, line) in self.map.iter().enumerate() {
            if empty_lines.contains(&y) {
                number_of_line_cross += 1;
                continue;
            }
            let mut number_of_col_cross = 0;
            for (x, galaxy) in line.iter().enumerate() {
                if empty_columns.contains(&x) {
                    number_of_col_cross += 1;
                    continue;
                }
                if *galaxy == Galaxy::Galaxy {
                    let x = x + number_of_col_cross as usize * expansion as usize;
                    let y = y + number_of_line_cross as usize * expansion as usize;
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }
}

#[cfg(test)]
mod test_day11 {
    use crate::*;

    #[test]
    fn test_emptylines(){
        let input = include_str!("./test.txt");
        let map = Map::new(input);
        assert_eq!(map.detect_empty_line(), vec![3, 7]);
        assert_eq!(map.detect_empty_column(), vec![2, 5, 8]);
    }

    #[test]
    fn test_part1(){
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input), 374);
    }
    
    #[test]
    fn test_part2(){
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input, 10), 1030);
        assert_eq!(part_2(input, 100), 8410);
    }
}