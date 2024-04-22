use num_integer::lcm;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-20/input.txt");
    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    let re = Regex::new(r"([%&])?([a-z]+)\s->(.*)").unwrap();
    let mut inputs = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap.get(1),
                cap.get(2).unwrap().as_str(),
                cap.get(3)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.trim())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut inputs_name = inputs.iter().map(|(_, name, _)| name).collect::<Vec<_>>();
    //Add inputs with no outputs
    let mut inputs_with_no_outputs: Vec<_> = Vec::new();
    for (_, _, ref_) in inputs.iter() {
        for r in ref_.iter() {
            if !inputs_name.contains(&r) {
                inputs_name.push(r);
                inputs_with_no_outputs.push((None, *r, vec![]));
            }
        }
    }
    inputs.extend(inputs_with_no_outputs);
    //Replace all references by name to references by index in the inputs array
    let inputs_with_index = inputs
        .iter()
        .map(|(a, b, ref_)| {
            (
                a,
                b,
                ref_.iter()
                    .map(|r| inputs.iter().position(|(_, name, _)| name == r).unwrap() as u64)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut modules: HashMap<&str, Rc<RefCell<Module>>> = HashMap::new();
    for (type_char, name, _) in inputs_with_index.iter() {
        modules.insert(
            name,
            if type_char.is_some() {
                match type_char.unwrap().as_str() {
                    "&" => Rc::new(RefCell::new(Module::Conjunction(Rc::new(RefCell::new(
                        Conjunction {
                            name,
                            inputs: vec![],
                            last_pulse: vec![],
                            outputs: vec![],
                        },
                    ))))),
                    "%" => Rc::new(RefCell::new(Module::FlipFlop(Rc::new(RefCell::new(
                        FlipFlop {
                            name,
                            outputs: vec![],
                            state: false,
                        },
                    ))))),
                    _ => panic!("Unknown type char"),
                }
            } else {
                Rc::new(RefCell::new(Module::Broadcast(Rc::new(RefCell::new(
                    Broadcast {
                        name,
                        outputs: vec![],
                    },
                )))))
            },
        );
    }
    //Create ref between modules
    for (_, name, outputs_index) in inputs_with_index.iter() {
        let ouputs = outputs_index
            .iter()
            .map(|index| Rc::clone(&modules[inputs_with_index[*index as usize].1]))
            .collect::<Vec<_>>();
        Module::add_outputs(&modules[*name], ouputs);
    }

    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut signals_sum = (0, 0);
    for _ in 0..1000 {
        signals.push_back(Signal {
            from: Rc::clone(&modules["broadcaster"]),
            to: Rc::clone(&modules["broadcaster"]),
            value: false,
        });
        while let Some(signal) = signals.pop_front() {
            if !signal.value {
                signals_sum.0 += 1;
            } else {
                signals_sum.1 += 1;
            }
            let new_signals = Module::handle_signal(signal);
            signals.extend(new_signals);
        }
    }
    signals_sum.0 * signals_sum.1
}

fn part_2(input: &str) -> u64 {
    let re = Regex::new(r"([%&])?([a-z]+)\s->(.*)").unwrap();
    let mut inputs = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap.get(1),
                cap.get(2).unwrap().as_str(),
                cap.get(3)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.trim())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut inputs_name = inputs.iter().map(|(_, name, _)| name).collect::<Vec<_>>();
    //Add inputs with no outputs
    let mut inputs_with_no_outputs: Vec<_> = Vec::new();
    for (_, _, ref_) in inputs.iter() {
        for r in ref_.iter() {
            if !inputs_name.contains(&r) {
                inputs_name.push(r);
                inputs_with_no_outputs.push((None, *r, vec![]));
            }
        }
    }
    inputs.extend(inputs_with_no_outputs);
    //Replace all references by name to references by index in the inputs array
    let inputs_with_index = inputs
        .iter()
        .map(|(a, b, ref_)| {
            (
                a,
                b,
                ref_.iter()
                    .map(|r| inputs.iter().position(|(_, name, _)| name == r).unwrap() as u64)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut modules: HashMap<&str, Rc<RefCell<Module>>> = HashMap::new();
    for (type_char, name, _) in inputs_with_index.iter() {
        modules.insert(
            name,
            if type_char.is_some() {
                match type_char.unwrap().as_str() {
                    "&" => Rc::new(RefCell::new(Module::Conjunction(Rc::new(RefCell::new(
                        Conjunction {
                            name,
                            inputs: vec![],
                            last_pulse: vec![],
                            outputs: vec![],
                        },
                    ))))),
                    "%" => Rc::new(RefCell::new(Module::FlipFlop(Rc::new(RefCell::new(
                        FlipFlop {
                            name,
                            outputs: vec![],
                            state: false,
                        },
                    ))))),
                    _ => panic!("Unknown type char"),
                }
            } else {
                Rc::new(RefCell::new(Module::Broadcast(Rc::new(RefCell::new(
                    Broadcast {
                        name,
                        outputs: vec![],
                    },
                )))))
            },
        );
    }
    //Create ref between modules
    for (_, name, outputs_index) in inputs_with_index.iter() {
        let ouputs = outputs_index
            .iter()
            .map(|index| Rc::clone(&modules[inputs_with_index[*index as usize].1]))
            .collect::<Vec<_>>();
        Module::add_outputs(&modules[*name], ouputs);
    }

    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut push_counter = 0;
    let mut cycle_counter: [u64; 4] = [0; 4];
    //rx is wired to only four conjunctions modules with inverters so we can find the cycle by finding the lcm of the cycle of each conjunction (jj, gf, xz and bz)
    'outer: loop {
        push_counter += 1;
        signals.push_back(Signal {
            from: Rc::clone(&modules["broadcaster"]),
            to: Rc::clone(&modules["broadcaster"]),
            value: false,
        });
        while let Some(signal) = signals.pop_front() {
            if Module::get_name(&signal.from) == "jj" && !signal.value {
                cycle_counter[0] = push_counter;
            } else if Module::get_name(&signal.from) == "gf" && !signal.value {
                cycle_counter[1] = push_counter;
            } else if Module::get_name(&signal.from) == "xz" && !signal.value {
                cycle_counter[2] = push_counter;
            } else if Module::get_name(&signal.from) == "bz" && !signal.value {
                cycle_counter[3] = push_counter;
            }
            if cycle_counter.iter().all(|f| *f > 0) {
                break 'outer;
            }
            let new_signals = Module::handle_signal(signal);
            signals.extend(new_signals);
        }
    }
    lcm(
        lcm(cycle_counter[0], cycle_counter[1]),
        lcm(cycle_counter[2], cycle_counter[3]),
    )
}

#[derive(PartialEq, Clone)]
struct FlipFlop<'a> {
    name: &'a str,
    outputs: Vec<Rc<RefCell<Module<'a>>>>,
    state: bool,
}

#[derive(PartialEq, Clone)]
struct Conjunction<'a> {
    name: &'a str,
    inputs: Vec<Rc<RefCell<Module<'a>>>>,
    last_pulse: Vec<bool>,
    outputs: Vec<Rc<RefCell<Module<'a>>>>,
}

#[derive(PartialEq, Clone)]
struct Broadcast<'a> {
    name: &'a str,
    outputs: Vec<Rc<RefCell<Module<'a>>>>,
}

struct Signal<'a> {
    from: Rc<RefCell<Module<'a>>>,
    to: Rc<RefCell<Module<'a>>>,
    value: bool,
}

#[derive(PartialEq, Clone)]
enum Module<'a> {
    Broadcast(Rc<RefCell<Broadcast<'a>>>),
    Conjunction(Rc<RefCell<Conjunction<'a>>>),
    FlipFlop(Rc<RefCell<FlipFlop<'a>>>),
}

impl<'a> Module<'a> {
    fn handle_signal(signal: Signal<'a>) -> Vec<Signal<'a>> {
        let self_ = &signal.to;
        match &*self_.borrow() {
            Module::Broadcast(broadcast) => {
                let broadcast = broadcast.borrow();
                broadcast
                    .outputs
                    .iter()
                    .map(|output| Signal {
                        from: Rc::clone(self_),
                        to: Rc::clone(output),
                        value: signal.value,
                    })
                    .collect()
            }
            Module::Conjunction(conjunction) => {
                {
                    let index_of_input_signal = conjunction
                        .borrow()
                        .inputs
                        .iter()
                        .position(|input| Module::names_eq(input, &signal.from))
                        .unwrap();
                    let mut conjunction = conjunction.borrow_mut();
                    if conjunction.inputs.len() != conjunction.last_pulse.len() {
                        conjunction.last_pulse = vec![false; conjunction.inputs.len()];
                    }
                    conjunction.last_pulse[index_of_input_signal] = signal.value;
                }

                conjunction
                    .borrow()
                    .outputs
                    .iter()
                    .map(|output| Signal {
                        from: Rc::clone(self_),
                        to: Rc::clone(output),
                        value: !conjunction.borrow().last_pulse.iter().all(|pulse| *pulse),
                    })
                    .collect()
            }
            Module::FlipFlop(flip_flop) => {
                if !signal.value {
                    {
                        let mut flip_flop = flip_flop.borrow_mut();
                        flip_flop.state = !flip_flop.state;
                    }

                    flip_flop
                        .borrow()
                        .outputs
                        .iter()
                        .map(|output| Signal {
                            from: Rc::clone(self_),
                            to: Rc::clone(output),
                            value: flip_flop.borrow().state,
                        })
                        .collect()
                } else {
                    vec![]
                }
            }
        }
    }
    pub fn names_eq(self_: &Rc<RefCell<Module<'a>>>, other: &Rc<RefCell<Module<'a>>>) -> bool {
        match (&*self_.borrow(), &*other.borrow()) {
            (Module::Broadcast(broadcast), Module::Broadcast(other_broadcast)) => {
                broadcast.borrow().name == other_broadcast.borrow().name
            }
            (Module::Conjunction(conjunction), Module::Conjunction(other_conjunction)) => {
                conjunction.borrow().name == other_conjunction.borrow().name
            }
            (Module::FlipFlop(flip_flop), Module::FlipFlop(other_flip_flop)) => {
                flip_flop.borrow().name == other_flip_flop.borrow().name
            }
            _ => false,
        }
    }
    pub fn get_name(self_: &Rc<RefCell<Module<'a>>>) -> &'a str {
        match &*self_.borrow() {
            Module::Broadcast(broadcast) => broadcast.borrow().name,
            Module::Conjunction(conjunction) => conjunction.borrow().name,
            Module::FlipFlop(flip_flop) => flip_flop.borrow().name,
        }
    }
    pub fn add_input(self_: &Rc<RefCell<Module<'a>>>, input: &Rc<RefCell<Module<'a>>>) {
        if let Module::Conjunction(conjunction) = &*self_.borrow() {
            let mut conjunction = conjunction.borrow_mut();
            conjunction.inputs.push(Rc::clone(input));
            conjunction.last_pulse.push(false);
        }
    }
    pub fn add_outputs(self_: &Rc<RefCell<Module<'a>>>, outputs: Vec<Rc<RefCell<Module<'a>>>>) {
        match &*self_.borrow() {
            Module::Broadcast(broadcast) => {
                let mut broadcast = broadcast.borrow_mut();
                for output in outputs.iter() {
                    Module::add_input(output, self_);
                }
                broadcast.outputs = outputs;
            }
            Module::Conjunction(conjunction) => {
                let mut conjunction = conjunction.borrow_mut();
                for output in outputs.iter() {
                    Module::add_input(output, self_);
                }
                conjunction.outputs = outputs;
            }
            Module::FlipFlop(flip_flop) => {
                let mut flip_flop = flip_flop.borrow_mut();
                for output in outputs.iter() {
                    Module::add_input(output, self_);
                }
                flip_flop.outputs = outputs;
            }
        }
    }
}

#[cfg(test)]
mod tests_day20 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2023-inputs/day-20/test.txt");
        assert_eq!(part_1(input), 32000000);

        let input = include_str!("../../aoc-2023-inputs/day-20/test2.txt");
        assert_eq!(part_1(input), 11687500);
    }
}
