fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-02/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += is_game_possible(line);
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let values = fewest_num_possible(line);
        sum += values[0] * values[1] * values[2];
    }
    sum
}

fn is_game_possible(input_line: &str) -> u32 {
    let input_slice_id: Vec<&str> = input_line.split(":").collect();
    let game_id = parse_game_id(input_slice_id[0]);

    for round_slice in input_slice_id[1].split(";") {
        for color_slice in round_slice.split(",") {
            let key_value_split: Vec<&str> = color_slice.trim().split(" ").collect();
            let current_num: i32 = key_value_split[0].to_string().parse().unwrap();
            match key_value_split[1] {
                "blue" => {
                    if current_num > 14 {
                        return 0;
                    }
                }
                "green" => {
                    if current_num > 13 {
                        return 0;
                    }
                }
                "red" => {
                    if current_num > 12 {
                        return 0;
                    }
                }
                _ => {}
            }
        }
    }

    game_id
}

fn fewest_num_possible(input_line: &str) -> [u32; 3] {
    let input_slice_id: Vec<&str> = input_line.split(":").collect();
    let mut fewest_color_num: [u32; 3] = [0, 0, 0];

    for round_slice in input_slice_id[1].split(";") {
        for color_slice in round_slice.split(",") {
            let key_value_split: Vec<&str> = color_slice.trim().split(" ").collect();
            let current_num: u32 = key_value_split[0].to_string().parse().unwrap();
            match key_value_split[1] {
                "red" => {
                    if current_num > fewest_color_num[0] {
                        fewest_color_num[0] = current_num;
                    }
                }
                "green" => {
                    if current_num > fewest_color_num[1] {
                        fewest_color_num[1] = current_num;
                    }
                }
                "blue" => {
                    if current_num > fewest_color_num[2] {
                        fewest_color_num[2] = current_num;
                    }
                }
                _ => {}
            }
        }
    }

    fewest_color_num
}

fn parse_game_id(input_slice: &str) -> u32 {
    let str_slice: Vec<&str> = input_slice.split(" ").collect();
    str_slice[1].to_string().parse().unwrap()
}

#[cfg(test)]
mod tests_day02 {
    use super::*;

    #[test]
    fn test_game_id() {
        assert_eq!(parse_game_id("Game 1"), 1);
        assert_eq!(parse_game_id("Game 98"), 98);
    }

    #[test]
    fn test_is_game_possible() {
        assert_eq!(
            is_game_possible("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            is_game_possible("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            is_game_possible(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            0
        );
        assert_eq!(
            is_game_possible(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            0
        );
        assert_eq!(
            is_game_possible("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            5
        );
    }

    #[test]
    fn test_fewest_color_num() {
        assert_eq!(
            fewest_num_possible("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            [4, 2, 6]
        );
        assert_eq!(
            fewest_num_possible("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            [1, 3, 4]
        );
        assert_eq!(
            fewest_num_possible(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            [20, 13, 6]
        );
        assert_eq!(
            fewest_num_possible(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            [14, 3, 15]
        );
        assert_eq!(
            fewest_num_possible("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            [6, 3, 2]
        );
    }
}
