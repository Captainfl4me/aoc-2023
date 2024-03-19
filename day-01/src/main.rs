pub mod search_tree;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-01/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let value = calib_value(line);
        sum += value;
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let value = calib_value_imprv(line);
        sum += value;
    }
    sum
}

fn calib_value(hash: &str) -> u32 {
    let mut first_digit = '0';
    let mut last_digit = '0';
    for char in hash.chars() {
        if char.is_ascii_digit() {
            if first_digit == '0' {
                first_digit = char;
            }
            last_digit = char;
        }
    }
    first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap()
}

fn calib_value_imprv(hash: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut first_digit_index: i32 = 10000;

    let mut last_digit: u32 = 0;
    let mut last_digit_index: i32 = -1;

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (num_index, number) in numbers.iter().enumerate() {
        for (i, _) in hash.match_indices(number) {
            let i_unwrap = i.try_into().unwrap();
            let num_index_unwrap: u32 = num_index.try_into().unwrap();
            if i_unwrap < first_digit_index {
                first_digit = num_index_unwrap + 1;
                first_digit_index = i_unwrap;
            }
            if i_unwrap > last_digit_index {
                last_digit = num_index_unwrap + 1;
                last_digit_index = i_unwrap;
            }
        }
    }

    for (i, char) in hash.chars().enumerate() {
        if char.is_ascii_digit() {
            let i_unwrap = i.try_into().unwrap();
            if i_unwrap < first_digit_index {
                first_digit = char.to_digit(10).unwrap();
                first_digit_index = i_unwrap;
            }
            if i_unwrap > last_digit_index {
                last_digit = char.to_digit(10).unwrap();
                last_digit_index = i_unwrap;
            }
        }
    }

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests_day01 {
    use super::*;

    #[test]
    fn test_day_01_part1() {
        assert_eq!(calib_value("1abc2"), 12);
        assert_eq!(calib_value("pqr3stu8vwx"), 38);
        assert_eq!(calib_value("a1b2c3d4e5f"), 15);
        assert_eq!(calib_value("treb7uchet"), 77);
    }
    #[test]
    fn test_day_02_part2() {
        assert_eq!(calib_value_imprv("two1nine"), 29);
        assert_eq!(calib_value_imprv("eightwothree"), 83);
        assert_eq!(calib_value_imprv("abcone2threexyz"), 13);
        assert_eq!(calib_value_imprv("xtwone3four"), 24);
        assert_eq!(calib_value_imprv("4nineeightseven2"), 42);
        assert_eq!(calib_value_imprv("zoneight234"), 14);
        assert_eq!(calib_value_imprv("7pqrstsixteen"), 76);
        assert_eq!(calib_value_imprv("twone"), 21);
    }

    #[test]
    fn test_search_tree() {
        use search_tree::StringSearchTree;

        let mut tree = StringSearchTree::new();
        tree.insert_string_on_root("one", 1);
        tree.insert_string_on_root("two", 2);
        tree.insert_string_on_root("three", 3);

        assert_eq!(tree.match_string_from_root("n").length, 0);
        assert_eq!(tree.match_string_from_root("on").length, 2);
        assert_eq!(tree.match_string_from_root("one").length, 3);
        assert_eq!(tree.match_string_from_root("one").value, 1);
        assert_eq!(tree.match_string_from_root("two").value, 2);
    }
}
