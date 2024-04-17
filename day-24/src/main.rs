use regex::Regex;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-24/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input, 200000000000000, 400000000000000));
    // dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

struct Path {
    point: (u64, u64, u64),
    v0: (i64, i64, i64),
}
impl Path {
    fn new(point: (u64, u64, u64), v0: (i64, i64, i64)) -> Self {
        Self { point, v0 }
    }

    /// Returns the coefficient of the path in 2D a,b such as the path is given by y=ax+b
    pub fn equation_2d(&self) -> (f64, f64) {
        let p1 = (self.point.0 as f64, self.point.1 as f64);
        let p2 = (
            self.point.0 as f64 + self.v0.0 as f64,
            self.point.1 as f64 + self.v0.1 as f64,
        );

        let a = (p1.1 - p2.1) / (p1.0 - p2.0);
        let b = p1.1 - a * p1.0;
        (a, b)
    }

    pub fn intersect_2d(&self, p: &Path) -> Option<(f64, f64)> {
        let (a1, b1) = self.equation_2d();
        let (a2, b2) = p.equation_2d();

        if a1 - a2 == 0.0 {
            return None;
        }

        let x = (b2 - b1) / (a1 - a2);
        let y = a1 * x + b1;
        Some((x, y))
    }

    pub fn is_point_on_2dpath(&self, point: (f64, f64)) -> bool {
        if self.v0.0 > 0 && (self.point.0 as f64 > point.0) {
            return false;
        } else if self.v0.0 < 0 && (self.point.0 as f64) < point.0 {
            return false;
        }

        if self.v0.1 > 0 && (self.point.1 as f64) > point.1 {
            return false;
        } else if self.v0.1 < 0 && (self.point.1 as f64) < point.1 {
            return false;
        }

        return true;
    }
}

fn part_1(input: &str, min_box: u64, max_box: u64) -> u64 {
    let re = Regex::new(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)").unwrap();
    let mut paths = Vec::new();
    for (_, [px, py, pz, vx, vy, vz]) in re.captures_iter(input).map(|c| c.extract()) {
        let path = Path::new(
            (
                px.parse().unwrap(),
                py.parse().unwrap(),
                pz.parse().unwrap(),
            ),
            (
                vx.parse().unwrap(),
                vy.parse().unwrap(),
                vz.parse().unwrap(),
            ),
        );
        paths.push(path);
    }

    let mut count = 0;
    for i in 0..paths.len() {
        for j in i + 1..paths.len() {
            if let Some((x, y)) = paths[i].intersect_2d(&paths[j]) {
                if x >= min_box as f64
                    && x <= max_box as f64
                    && y >= min_box as f64
                    && y <= max_box as f64
                {
                    if paths[i].is_point_on_2dpath((x, y)) && paths[j].is_point_on_2dpath((x, y)) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

/* fn part_2(input: &str) -> u64 {
    todo!()
} */

#[cfg(test)]
mod tests_day24 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-24/test.txt");
        println!("{}", input);
        assert_eq!(part_1(input, 7, 27), 2);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = include_str!("./test.txt");
    //     assert_eq!(part_2(input), 0);
    // }
}
