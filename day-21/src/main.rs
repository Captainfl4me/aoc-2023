use polyfit_rs::polyfit_rs::polyfit;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part_1(input, 64));
    dbg!(part_2(input, 26501365, 0));
}

fn part_1(input: &str, step_count: u64) -> u64 {
    let line_width = input.lines().count() as i64;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = Position {
        x: (line_width-1)/2, 
        y: (line_width-1)/2
    };
    assert_eq!(map[start_pos.y as usize][start_pos.x as usize], 'S');

    let mut states: Vec<State> = vec![State {
        position: start_pos,
        step: 0,
    }];
    for _step in 0..step_count {
        let mut next_states: Vec<State> = vec![];
        for state in states.iter() {
            let mut neighbour_states = vec![];
            let left_tile = Position { x: (state.position.x - 1).rem_euclid(line_width), y: state.position.y.rem_euclid(line_width) };
            let right_tile = Position { x: (state.position.x + 1).rem_euclid(line_width), y: state.position.y.rem_euclid(line_width) };
            let up_tile = Position { x: state.position.x.rem_euclid(line_width), y: (state.position.y - 1).rem_euclid(line_width) };
            let down_tile = Position { x: state.position.x.rem_euclid(line_width), y: (state.position.y + 1).rem_euclid(line_width) };

            // Go left
            if map[left_tile.y as usize][left_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x - 1, y: state.position.y },
                    step: state.step + 1,
                });
            }
            // Go right
            if map[right_tile.y as usize][right_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x + 1, y: state.position.y },
                    step: state.step + 1,
                });
            }
            // Go up
            if map[up_tile.y as usize][up_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x, y: state.position.y - 1 },
                    step: state.step + 1,
                });
            }
            // Go down
            if map[down_tile.y as usize][down_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x, y: state.position.y + 1 },
                    step: state.step + 1,
                });
            }

            for next_state in neighbour_states.iter() {
                if !next_states.iter().any(|s| s.position == next_state.position) {
                    next_states.push(*next_state);
                }
            }
        }
        states = next_states;
    }
    assert_eq!(states.iter().all(|s| s.step == step_count), true);
    states.len() as u64
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Position,
    step: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

fn part_2(input: &str, step_count: u64, offset: u64) -> u64 {
    let map_size = input.lines().count() as u64;

    if step_count < (2+offset)*map_size {
        return part_1(input, step_count);
    }

    let remainder = step_count % map_size;
    let xs: Vec<f64> = vec![offset as f64, (offset + 1) as f64, (offset + 2) as f64];
    let x = ((step_count - (map_size-1)/2) / map_size) as i64;
    let step_to_take: Vec<u64> = xs.iter().map(|x| map_size*(*x as u64)+remainder).collect();
    let mut ys: Vec<f64> = vec![];
    
    let line_width = input.lines().count() as i64;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = Position {
        x: (line_width-1)/2, 
        y: (line_width-1)/2
    };
    assert_eq!(map[start_pos.y as usize][start_pos.x as usize], 'S');

    let mut states: Vec<State> = vec![State {
        position: start_pos,
        step: 0,
    }];
    for step in 1..=*step_to_take.iter().max().unwrap() {
        let mut next_states: Vec<State> = vec![];
        for state in states.iter() {
            let mut neighbour_states = vec![];
            let left_tile = Position { x: (state.position.x - 1).rem_euclid(line_width), y: state.position.y.rem_euclid(line_width) };
            let right_tile = Position { x: (state.position.x + 1).rem_euclid(line_width), y: state.position.y.rem_euclid(line_width) };
            let up_tile = Position { x: state.position.x.rem_euclid(line_width), y: (state.position.y - 1).rem_euclid(line_width) };
            let down_tile = Position { x: state.position.x.rem_euclid(line_width), y: (state.position.y + 1).rem_euclid(line_width) };

            // Go left
            if map[left_tile.y as usize][left_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x - 1, y: state.position.y },
                    step: state.step + 1,
                });
            }
            // Go right
            if map[right_tile.y as usize][right_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x + 1, y: state.position.y },
                    step: state.step + 1,
                });
            }
            // Go up
            if map[up_tile.y as usize][up_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x, y: state.position.y - 1 },
                    step: state.step + 1,
                });
            }
            // Go down
            if map[down_tile.y as usize][down_tile.x as usize] != '#' {
                neighbour_states.push(State {
                    position: Position { x: state.position.x, y: state.position.y + 1 },
                    step: state.step + 1,
                });
            }

            for next_state in neighbour_states.iter() {
                if !next_states.iter().any(|s| s.position == next_state.position) {
                    next_states.push(*next_state);
                }
            }
        }
        states = next_states;

        if step_to_take.contains(&(step)) {
            ys.push(states.len() as f64);
        }
    }
    assert_eq!(ys.len(), 3);

    let coeff = polyfit(&xs, &ys, 2).unwrap();
    let coeff = coeff.iter().map(|x| x.round() as i64).collect::<Vec<i64>>();
    (x.pow(2) * coeff[2] + x * coeff[1] + coeff[0]) as u64
}

#[cfg(test)]
mod tests_day21 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        assert_eq!(part_1(input, 6), 16);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        assert_eq!(part_2(input, 6, 5), 16);
        assert_eq!(part_2(input, 10, 5), 50);
        assert_eq!(part_2(input, 50, 5), 1594);
        assert_eq!(part_2(input, 500, 5), 167004);
        assert_eq!(part_2(input, 1000, 5), 668697);
        assert_eq!(part_2(input, 5000, 5), 16733044);
    }
}
