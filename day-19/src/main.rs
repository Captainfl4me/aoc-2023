use regex::Regex;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-19/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

struct PieceTracker {
    piece: PiecesRange,
    workflow: String,
    rule_to_apply: usize,
}
fn part_2(input: &str) -> u64 {
    let (workflows, _) = parse_input(input);
    let mut queue = Vec::new();
    queue.push(PieceTracker {
        piece: PiecesRange::new(),
        workflow: "in".to_string(),
        rule_to_apply: 0,
    });

    let mut possibilities = 0;
    while let Some(piece) = queue.pop() {
        let rule = workflows
            .get(piece.workflow.as_str())
            .unwrap()
            .rules
            .get(piece.rule_to_apply)
            .unwrap();
        if let Some(new_piece) = rule.apply_to_range(&piece.piece) {
            if rule.fallback.clone() == "A" {
                possibilities += new_piece.posibilities();
            } else if rule.fallback.clone() != "R" {
                queue.push(PieceTracker {
                    piece: new_piece,
                    workflow: rule.fallback.clone(),
                    rule_to_apply: 0,
                });
            }
        }
        if let Some(new_piece) = rule.apply_inverse_to_range(&piece.piece) {
            queue.push(PieceTracker {
                piece: new_piece,
                workflow: piece.workflow,
                rule_to_apply: piece.rule_to_apply + 1,
            });
        }
    }
    possibilities
}

fn part_1(input: &str) -> u64 {
    let (workflows, pieces) = parse_input(input);

    pieces
        .iter()
        .map(|piece| {
            let mut next_workflow = "in";
            while next_workflow != "A" && next_workflow != "R" {
                for rule in workflows.get(next_workflow).unwrap().rules.iter() {
                    if rule.is_valid(&piece) {
                        next_workflow = rule.fallback.as_str();
                        break;
                    }
                }
            }
            if next_workflow == "A" {
                piece.sum()
            } else {
                0
            }
        })
        .sum()
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Piece>) {
    let mut workflows = HashMap::new();
    let mut pieces = Vec::new();
    let re_wrk = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();

    let mut is_workflow = true;
    for line in input.lines() {
        if line.is_empty() {
            is_workflow = false;
            continue;
        }
        if is_workflow {
            let (_, [name, rules]) = re_wrk
                .captures_iter(line)
                .map(|c| c.extract())
                .collect::<Vec<_>>()[0];
            workflows.insert(
                name,
                Workflow {
                    rules: Rule::from_str(rules),
                },
            );
        } else {
            let re = Regex::new(r"(\d+)").unwrap();
            let xmas = re
                .captures_iter(line)
                .map(|c| c[1].parse().unwrap())
                .collect::<Vec<u64>>();
            pieces.push(Piece::new(xmas[0], xmas[1], xmas[2], xmas[3]));
        }
    }

    (workflows, pieces)
}

#[derive(Debug, PartialEq)]
enum Condition {
    Inferior,
    Greater,
    None,
}
struct Rule {
    category: char,
    condition: Condition,
    value: u64,
    fallback: String,
}
impl Rule {
    pub fn from_str(s: &str) -> Vec<Self> {
        let re_rule = Regex::new(r"(?:([xmas])([<>])(\d+):)?([a-zA-Z]+)").unwrap();
        re_rule
            .captures_iter(s)
            .map(|c| {
                let category = c.get(1).map_or("0", |f| f.as_str()).chars().next().unwrap();
                let condition = c.get(2).map_or(Condition::None, |f| match f.as_str() {
                    "<" => Condition::Inferior,
                    ">" => Condition::Greater,
                    _ => panic!("Unknown condition"),
                });
                let value = c.get(3).map_or(0, |f| f.as_str().parse::<u64>().unwrap());
                let fallback = c[4].to_string();
                Self {
                    category,
                    condition,
                    value,
                    fallback,
                }
            })
            .collect()
    }

    pub fn is_valid(&self, piece: &Piece) -> bool {
        let value = match self.category {
            'x' => piece.x,
            'm' => piece.m,
            'a' => piece.a,
            's' => piece.s,
            _ => 0,
        };
        match self.condition {
            Condition::Inferior => value < self.value,
            Condition::Greater => value > self.value,
            Condition::None => true,
        }
    }

    pub fn apply_to_range(&self, piece: &PiecesRange) -> Option<PiecesRange> {
        let mut new_piece = piece.clone();
        match self.category {
            'x' => match self.condition {
                Condition::Greater => match new_piece.x.end().cmp(&self.value) {
                    Ordering::Greater => {
                        new_piece.x =
                            max(*new_piece.x.start(), self.value + 1)..=*new_piece.x.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.x.start().cmp(&self.value) {
                    Ordering::Less => {
                        new_piece.x =
                            *new_piece.x.start()..=min(*new_piece.x.end(), self.value - 1);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => Some(piece.clone()),
            },
            'm' => match self.condition {
                Condition::Greater => match new_piece.m.end().cmp(&self.value) {
                    Ordering::Greater => {
                        new_piece.m =
                            max(*new_piece.m.start(), self.value + 1)..=*new_piece.m.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.m.start().cmp(&self.value) {
                    Ordering::Less => {
                        new_piece.m =
                            *new_piece.m.start()..=min(*new_piece.m.end(), self.value - 1);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => Some(piece.clone()),
            },
            'a' => match self.condition {
                Condition::Greater => match new_piece.a.end().cmp(&self.value) {
                    Ordering::Greater => {
                        new_piece.a =
                            max(*new_piece.a.start(), self.value + 1)..=*new_piece.a.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.a.start().cmp(&self.value) {
                    Ordering::Less => {
                        new_piece.a =
                            *new_piece.a.start()..=min(*new_piece.a.end(), self.value - 1);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => Some(piece.clone()),
            },
            's' => match self.condition {
                Condition::Greater => match new_piece.s.end().cmp(&self.value) {
                    Ordering::Greater => {
                        new_piece.s =
                            max(*new_piece.s.start(), self.value + 1)..=*new_piece.s.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.s.start().cmp(&self.value) {
                    Ordering::Less => {
                        new_piece.s =
                            *new_piece.s.start()..=min(*new_piece.s.end(), self.value - 1);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => Some(piece.clone()),
            },
            _ => Some(piece.clone()),
        }
    }

    pub fn apply_inverse_to_range(&self, piece: &PiecesRange) -> Option<PiecesRange> {
        let mut new_piece = piece.clone();
        match self.category {
            'x' => match self.condition {
                Condition::Greater => match new_piece.x.start().cmp(&self.value) {
                    Ordering::Less | Ordering::Equal => {
                        new_piece.x = *new_piece.x.start()..=min(*new_piece.x.end(), self.value);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.x.end().cmp(&self.value) {
                    Ordering::Greater | Ordering::Equal => {
                        new_piece.x = max(*new_piece.x.start(), self.value)..=*new_piece.x.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => None,
            },
            'm' => match self.condition {
                Condition::Greater => match new_piece.m.start().cmp(&self.value) {
                    Ordering::Less | Ordering::Equal => {
                        new_piece.m = *new_piece.m.start()..=min(*new_piece.m.end(), self.value);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.m.end().cmp(&self.value) {
                    Ordering::Greater | Ordering::Equal => {
                        new_piece.m = max(*new_piece.m.start(), self.value)..=*new_piece.m.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => None,
            },
            'a' => match self.condition {
                Condition::Greater => match new_piece.a.start().cmp(&self.value) {
                    Ordering::Less | Ordering::Equal => {
                        new_piece.a = *new_piece.a.start()..=min(*new_piece.a.end(), self.value);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.a.end().cmp(&self.value) {
                    Ordering::Greater | Ordering::Equal => {
                        new_piece.a = max(*new_piece.a.start(), self.value)..=*new_piece.a.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => None,
            },
            's' => match self.condition {
                Condition::Greater => match new_piece.s.start().cmp(&self.value) {
                    Ordering::Less | Ordering::Equal => {
                        new_piece.s = *new_piece.s.start()..=min(*new_piece.s.end(), self.value);
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::Inferior => match new_piece.s.end().cmp(&self.value) {
                    Ordering::Greater | Ordering::Equal => {
                        new_piece.s = max(*new_piece.s.start(), self.value)..=*new_piece.s.end();
                        Some(new_piece)
                    }
                    _ => None,
                },
                Condition::None => None,
            },
            _ => None,
        }
    }
}
struct Workflow {
    rules: Vec<Rule>,
}
struct Piece {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Piece {
    pub fn new(x: u64, m: u64, a: u64, s: u64) -> Self {
        Self { x, m, a, s }
    }
    pub fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}
#[derive(Clone)]
struct PiecesRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}
impl PiecesRange {
    pub fn new() -> Self {
        let default = 1..=4000;
        Self {
            x: default.clone(),
            m: default.clone(),
            a: default.clone(),
            s: default.clone(),
        }
    }
    pub fn posibilities(&self) -> u64 {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }
}

#[cfg(test)]
mod tests_day19 {
    use crate::*;

    #[test]
    fn test_rule_parse() {
        let rules = Rule::from_str("a<2006:qkq,m>2090:A,rfg");
        assert_eq!(rules.len(), 3);
        let rule = rules.first().unwrap();
        assert_eq!(rule.fallback, "qkq".to_string());
        assert_eq!(rule.category, 'a');
        assert_eq!(rule.value, 2006);
        assert_eq!(rule.condition, Condition::Inferior);
    }
    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-19/test.txt");
        assert_eq!(part_1(input), 19114);
    }
    #[test]
    fn test_rule_range() {
        let rule = Rule {
            category: 'x',
            condition: Condition::Greater,
            value: 2000,
            fallback: "qkq".to_string(),
        };
        let piece = PiecesRange::new();

        let new_piece = rule.apply_to_range(&piece).unwrap();
        assert_eq!(new_piece.x, 2001..=4000);
        let inv_new_piece = rule.apply_inverse_to_range(&piece).unwrap();
        assert_eq!(inv_new_piece.x, 1..=2000);
        assert_eq!(rule.apply_inverse_to_range(&new_piece).is_none(), true);
    }
    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2023-inputs/day-19/test.txt");
        assert_eq!(part_2(input), 167409079868000);
    }
}
