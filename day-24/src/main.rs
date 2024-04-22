use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::*;
use regex::Regex;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-24/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input, 200000000000000, 400000000000000));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

struct Path {
    point: (i64, i64, i64),
    v0: (i64, i64, i64),
}
impl Path {
    fn new(point: (i64, i64, i64), v0: (i64, i64, i64)) -> Self {
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
        if self.v0.0 > 0 && (self.point.0 as f64 > point.0)
            || self.v0.0 < 0 && (self.point.0 as f64) < point.0
        {
            return false;
        }
        if self.v0.1 > 0 && (self.point.1 as f64) > point.1
            || self.v0.1 < 0 && (self.point.1 as f64) < point.1
        {
            return false;
        }

        true
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
                    && paths[i].is_point_on_2dpath((x, y))
                    && paths[j].is_point_on_2dpath((x, y))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_system(p1: &Path, p2: &Path, p3: &Path) -> Vec<f64> {
    let a: Array2<i64> = arr2(&[
        [
            0,
            p2.v0.2 - p1.v0.2,
            p1.v0.1 - p2.v0.1,
            0,
            p1.point.2 - p2.point.2,
            p2.point.1 - p1.point.1,
        ],
        [
            p1.v0.2 - p2.v0.2,
            0,
            p2.v0.0 - p1.v0.0,
            p2.point.2 - p1.point.2,
            0,
            p1.point.0 - p2.point.0,
        ],
        [
            p2.v0.1 - p1.v0.1,
            p1.v0.0 - p2.v0.0,
            0,
            p1.point.1 - p2.point.1,
            p2.point.0 - p1.point.0,
            0,
        ],
        [
            0,
            p3.v0.2 - p1.v0.2,
            p1.v0.1 - p3.v0.1,
            0,
            p1.point.2 - p3.point.2,
            p3.point.1 - p1.point.1,
        ],
        [
            p1.v0.2 - p3.v0.2,
            0,
            p3.v0.0 - p1.v0.0,
            p3.point.2 - p1.point.2,
            0,
            p1.point.0 - p3.point.0,
        ],
        [
            p3.v0.1 - p1.v0.1,
            p1.v0.0 - p3.v0.0,
            0,
            p1.point.1 - p3.point.1,
            p3.point.0 - p1.point.0,
            0,
        ],
    ]);
    let a: Array2<f64> = a.mapv(|x| x as f64);

    let b: Array1<i64> = -arr1(&[
        p1.point.1 * p1.v0.2 - p2.point.1 * p2.v0.2 - p1.point.2 * p1.v0.1 + p2.point.2 * p2.v0.1,
        p1.point.2 * p1.v0.0 - p2.point.2 * p2.v0.0 - p1.point.0 * p1.v0.2 + p2.point.0 * p2.v0.2,
        p1.point.0 * p1.v0.1 - p2.point.0 * p2.v0.1 - p1.point.1 * p1.v0.0 + p2.point.1 * p2.v0.0,
        p1.point.1 * p1.v0.2 - p3.point.1 * p3.v0.2 - p1.point.2 * p1.v0.1 + p3.point.2 * p3.v0.1,
        p1.point.2 * p1.v0.0 - p3.point.2 * p3.v0.0 - p1.point.0 * p1.v0.2 + p3.point.0 * p3.v0.2,
        p1.point.0 * p1.v0.1 - p3.point.0 * p3.v0.1 - p1.point.1 * p1.v0.0 + p3.point.1 * p3.v0.0,
    ]);
    let b: Array1<f64> = b.mapv(|x| x as f64);

    let x = a.solve(&b).unwrap();
    x.into_raw_vec()
}

fn part_2(input: &str) -> u64 {
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

    // Solve the system of equations for the first 10 combinations of paths (to prevent float
    // imprecision errors)
    let results = paths
        .iter()
        .tuple_combinations()
        .map(|(p1, p2, p3)| {
            let r = solve_system(p1, p2, p3);
            vec![r[0], r[1], r[2]]
        })
        .take(10)
        .collect::<Vec<_>>();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for r in results.iter() {
        x += r[0];
        y += r[1];
        z += r[2];
    }
    let xr = (x / results.len() as f64).round() as u64;
    let yr = (y / results.len() as f64).round() as u64;
    let zr = (z / results.len() as f64).round() as u64;

    xr + yr + zr
}

#[cfg(test)]
mod tests_day24 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-24/test.txt");
        println!("{}", input);
        assert_eq!(part_1(input, 7, 27), 2);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-24/test.txt");
        assert_eq!(part_2(input), 47);
    }
}
