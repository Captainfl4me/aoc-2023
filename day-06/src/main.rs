use regex::Regex;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-06/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let races = Race::parse_from_str(input);
    let mut margin = 1;
    for race in races.iter() {
        let mut num_of_win = 0;
        for start_time in 1..race.duration {
            // speed = start_time * 1
            let score = (race.duration - start_time) * start_time;
            if score > race.best_score {
                num_of_win += 1;
            }
        }
        margin *= num_of_win;
    }
    margin
}

fn part_2(input: &str) -> u64 {
    let race = Race::parse_from_str_part2(input);
    let mut num_of_win = 0;
    for start_time in 1..race.duration {
        // speed = start_time * 1
        let score = (race.duration - start_time) * start_time;
        if score > race.best_score {
            num_of_win += 1;
        }
    }
    num_of_win
}

struct Race {
    duration: u64,
    best_score: u64,
}
impl Race {
    pub fn new(duration: u64, best_score: u64) -> Race {
        Race {
            duration,
            best_score,
        }
    }
    pub fn parse_from_str(input: &str) -> Vec<Race> {
        let re = Regex::new(r"\s([0-9]+)").unwrap();
        let mut match_num: Vec<u64> = Vec::new();
        for (_, [num]) in re.captures_iter(input).map(|c| c.extract()) {
            match_num.push(num.parse::<u64>().unwrap());
        }
        let mut results: Vec<Race> = Vec::new();
        for i in 0..match_num.len() / 2 {
            results.push(Race::new(
                match_num[i],
                match_num[i + (match_num.len() / 2)],
            ));
        }
        results
    }

    pub fn parse_from_str_part2(input: &str) -> Race {
        let lines: Vec<u64> = input
            .lines()
            .map(|s| s.split(":").collect::<Vec<&str>>()[1])
            .map(|f| {
                f.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect();
        Race::new(lines[0], lines[1])
    }
}

#[cfg(test)]
mod tests_day06 {
    use super::*;

    #[test]
    fn test_parsing_input() {
        let input = include_str!("../../aoc-2023-inputs/day-06/test.txt");
        let races = Race::parse_from_str(input);
        assert_eq!(races.len(), 3);
        assert_eq!(races[0].duration, 7);
        assert_eq!(races[2].duration, 30);
        assert_eq!(races[2].best_score, 200);
    }

    #[test]
    fn test_parsing_input_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-06/test.txt");
        let race = Race::parse_from_str_part2(input);
        assert_eq!(race.duration, 71530);
        assert_eq!(race.best_score, 940200);

        let input = include_str!("../../aoc-2023-inputs/day-06/test.txt");
        let race = Race::parse_from_str_part2(input);
        assert_eq!(race.duration, 56717999);
        assert_eq!(race.best_score, 334113513502430);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../aoc-2023-inputs/day-06/test.txt");
        let margin = part_1(input);
        assert_eq!(margin, 288);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-06/test.txt");
        let margin = part_2(input);
        assert_eq!(margin, 71503);
    }
}

