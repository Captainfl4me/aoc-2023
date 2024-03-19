fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-09/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> i64 {
    let histories: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let mut projected_values: Vec<i64> = Vec::new();
    for history in histories.iter() {
        let mut project_table = vec![history.clone()];
        while project_table.last().unwrap().iter().any(|f| *f != 0) {
            let diff: Vec<i64> = project_table
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            project_table.push(diff);
        }
        projected_values.push(project_table.into_iter().map(|a| *a.last().unwrap()).sum())
    }
    projected_values.iter().sum()
}

fn part_2(input: &str) -> i64 {
    let histories: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let mut projected_values: Vec<i64> = Vec::new();
    for history in histories.iter() {
        let mut project_table = vec![history.clone()];
        while project_table.last().unwrap().iter().any(|f| *f != 0) {
            let diff: Vec<i64> = project_table
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            project_table.push(diff);
        }
        projected_values.push(
            project_table
                .into_iter()
                .enumerate()
                .map(|(index, a)| {
                    let val = *a.first().unwrap();
                    if index % 2 == 0 {
                        val
                    } else {
                        -val
                    }
                })
                .sum(),
        )
    }
    projected_values.iter().sum()
}

#[cfg(test)]
mod test_day09 {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-09/test.txt");
        assert_eq!(part_1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-09/input.txt");
        assert_eq!(part_2(input), 2);
    }
}

