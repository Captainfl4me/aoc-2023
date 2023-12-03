fn main() {
    let input = include_str!("./input.txt");
    dbg!(sum_part_number(input));
}

fn sum_part_number(input: &str) -> u32 {
    let mut current_part = 0;
    let mut keep_current_part = false;
    let mut valid_parts: Vec<u32> = Vec::new();
    let mut result = 0;

    let input_lines = input.lines();
    let lines_arr: Vec<Vec<char>> = input_lines.collect::<Vec<&str>>().iter().map(|line| line.chars().collect::<Vec<char>>()).collect();

    for (index_row, line) in lines_arr.iter().enumerate() {
        let index_row_unwrap: i32 = index_row.try_into().unwrap();
        if keep_current_part {
            valid_parts.push(current_part);
            result += current_part;
            println!("n = {}", current_part);
            current_part = 0;
            keep_current_part = false;
        }
        for (index_col, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                current_part *= 10;
                current_part += char.to_digit(10).unwrap();

                let index_col_unwrap: i32 = index_col.try_into().unwrap();
                for lookup_it_col in -1i32..=1 {
                    for lookup_it_row in -1i32..=1 {
                        if lookup_it_row == 0 && lookup_it_col == 0 { continue; }
                        
                        let lookup_index_row = index_row_unwrap + lookup_it_row;
                        let lookup_index_col = index_col_unwrap + lookup_it_col;
                        if lookup_index_row < 0 || lookup_index_col < 0 || lookup_index_row >= lines_arr.len().try_into().unwrap() || lookup_index_col >= line.len().try_into().unwrap() {
                            continue;
                        }
                        let lookup_index_row: usize = lookup_index_row.try_into().unwrap();
                        let lookup_index_col: usize = lookup_index_col.try_into().unwrap();

                        let lookup_char = lines_arr[lookup_index_row][lookup_index_col];
                        if !(lookup_char.is_ascii_digit() || lookup_char == '.' ) {
                            keep_current_part = true;
                        }
                    }
                }
            } else {
                if keep_current_part {
                    valid_parts.push(current_part);
                    result += current_part;
                    println!("n = {}", current_part);
                }
                current_part = 0;
                keep_current_part = false;
            } 
        }
    }
    if keep_current_part {
        println!("n = {}", current_part);
        result += current_part;
    }
    valid_parts.sort();
    valid_parts.dedup();

    let sum: u32 = dbg!(valid_parts.iter().sum());
    dbg!(valid_parts.len());

    result
}

#[cfg(test)]
mod tests_day02 {
    use super::*;

    #[test]
    fn test_sum_part_number() {
        assert_eq!(sum_part_number("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 4361);
        assert_eq!(sum_part_number("467*114...\n..........\n.^35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 4475);
        assert_eq!(sum_part_number("12.......*..\n+.........34\n.......-12..\n..78........\n..*....60...\n78..........\n.......23...\n....90*12...\n............\n2.2......12.\n.*.........*\n1.1.......56"), 413);
        assert_eq!(sum_part_number("....................\n..-52..52-..52..52..\n..................-."), 156);
        assert_eq!(sum_part_number("12.......*..\n+.........34\n.......-12..\n..78........\n..*....60...\n78.........9\n.5.....23..$\n8...90*12...\n............\n2.2......12.\n.*.........*\n1.1..503+.56"), 925);
        assert_eq!(sum_part_number("12\n24"), 0);
    }
}