fn main() {
    let input = include_str!("./input.txt");

    let mut sum = 0;
    for line in input.lines() {
        let value = calib_value(line);
        sum += value;
    }

    dbg!(sum);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(calib_value("1abc2"), 12);
        assert_eq!(calib_value("pqr3stu8vwx"), 38);
        assert_eq!(calib_value("a1b2c3d4e5f"), 15);
        assert_eq!(calib_value("treb7uchet"), 77);
    }
}