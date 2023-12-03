pub mod search_tree;
use search_tree::StringSearchTree;

fn main() {
    let input = include_str!("./input.txt");
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
    first_digit.to_digit(10).unwrap()*10 + last_digit.to_digit(10).unwrap()
}  

fn calib_value_imprv(hash: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    let mut str_buf: String = String::new();

    let mut search_tree = StringSearchTree::new();
    search_tree.insert_string_on_root("one", 1);
    search_tree.insert_string_on_root("two", 2);
    search_tree.insert_string_on_root("three", 3);
    search_tree.insert_string_on_root("four", 4);
    search_tree.insert_string_on_root("five", 5);
    search_tree.insert_string_on_root("six", 6);
    search_tree.insert_string_on_root("seven", 7);
    search_tree.insert_string_on_root("eight", 8);
    search_tree.insert_string_on_root("nine", 9);

    for char in hash.chars() {
        let mut num: Option<u32> = Option::None;
        if char.is_ascii_digit() {
           num = Some(char.to_digit(10).unwrap());
        } else if char.is_ascii_alphabetic() {
            str_buf.push(char);

            let match_result = search_tree.match_string_from_root(&str_buf);
            if match_result.length <= 0 {
                str_buf.clear();
            } else if match_result.length < str_buf.len().try_into().unwrap() {
                str_buf.clear();
                str_buf.push(char);
            } else if match_result.length == str_buf.len().try_into().unwrap() {
                if match_result.value != 0 {
                    num = Some(match_result.value);
                    str_buf.clear();
                }
            }
        }

        if num.is_some() {
            if first_digit == 0 {
                first_digit = num.unwrap();
            }
            last_digit = num.unwrap();
        }
    }
    first_digit*10 + last_digit
}

#[cfg(test)]
mod tests {
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