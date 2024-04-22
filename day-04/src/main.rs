fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-04/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let value = card_point(line);
        sum += value;
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let mut scratchcards_count: Vec<u32> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        match scratchcards_count.get_mut(index) {
            Some(card_num) => {
                *card_num += 1;
            }
            None => {
                scratchcards_count.push(1);
            }
        }
        let curr_card_num = scratchcards_count[index];
        let value = num_of_match_cards(line);
        for offset in 1..=value {
            match scratchcards_count.get_mut(index + TryInto::<usize>::try_into(offset).unwrap()) {
                Some(card_num) => {
                    *card_num += curr_card_num;
                }
                None => {
                    scratchcards_count.push(curr_card_num);
                }
            }
        }
    }

    scratchcards_count.iter().sum()
}

fn num_of_match_cards(line: &str) -> u32 {
    let mut score: u32 = 0;
    let cards_slice: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .collect();
    let win_cards: Vec<u32> = cards_slice[0]
        .trim()
        .split(' ')
        .filter(|&f| !f.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|&f| f.trim().to_string().parse::<u32>().unwrap())
        .collect();
    let cards: Vec<u32> = cards_slice[1]
        .trim()
        .split(' ')
        .filter(|&f| !f.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.trim().to_string().parse::<u32>().unwrap())
        .collect();

    for card in cards.iter() {
        if win_cards.contains(card) {
            score += 1;
        }
    }

    score
}

fn card_point(line: &str) -> u32 {
    let mut score: u32 = 0;
    let cards_slice: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .collect();
    let win_cards: Vec<u32> = cards_slice[0]
        .trim()
        .split(' ')
        .filter(|&f| !f.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|&f| f.trim().to_string().parse::<u32>().unwrap())
        .collect();
    let cards: Vec<u32> = cards_slice[1]
        .trim()
        .split(' ')
        .filter(|&f| !f.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.trim().to_string().parse::<u32>().unwrap())
        .collect();

    for card in cards.iter() {
        if win_cards.contains(card) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests_day04 {
    use super::*;

    #[test]
    fn test_card_point() {
        assert_eq!(
            card_point("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            card_point("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            card_point("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            card_point("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            card_point("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            card_point("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
    }
}
