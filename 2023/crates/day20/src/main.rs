use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Debug, Default)]
struct Network {
    modules: HashMap<String, Module>,
    inputs: HashMap<String, Vec<String>>,
    outputs: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default)]
struct NetworkState {
    queue: VecDeque<(Pulse, String, String)>,
    flip_flop_memory: HashMap<String, bool>,
    conjunction_memory: HashMap<String, u64>,
    last_high_signal_time: HashMap<String, u64>,
    high_signal_periods: HashMap<String, u64>,
    button_count: u64,
    low_pulse_count: u64,
    high_pulse_count: u64,
}

#[derive(Debug, Clone, Copy)]
enum Module {
    Untyped,
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

impl Network {
    fn parse(input: &str) -> Network {
        fn parse_line(line: &str) -> (Module, String, Vec<String>) {
            let (name, outputs) = line.split_once(" -> ").unwrap();
            let outputs: Vec<String> = outputs.split(", ").map(|s| s.to_owned()).collect();
            if let Some(name) = name.strip_prefix("%") {
                (Module::FlipFlop, name.to_owned(), outputs)
            } else if let Some(name) = name.strip_prefix("&") {
                (Module::Conjunction, name.to_owned(), outputs)
            } else if name == "broadcaster" {
                (Module::Broadcaster, name.to_owned(), outputs)
            } else {
                (Module::Untyped, name.to_owned(), outputs)
            }
        }

        let mut network = Network::default();
        for (module, name, outputs) in input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Vec<_>>()
        {
            network.modules.insert(name.clone(), module);
            for output in &outputs {
                network
                    .inputs
                    .entry(output.clone())
                    .or_default()
                    .push(name.clone());
            }
            network.outputs.insert(name, outputs);
        }
        network
    }

    fn process_pulse(
        &self,
        state: &mut NetworkState,
        from_name: &str,
        to_name: &str,
        pulse: Pulse,
    ) {
        match pulse {
            Pulse::Low => state.low_pulse_count += 1,
            Pulse::High => {
                use std::collections::hash_map::Entry;
                state.high_pulse_count += 1;
                match state.last_high_signal_time.entry(from_name.to_owned()) {
                    Entry::Occupied(e) => {
                        state
                            .high_signal_periods
                            .insert(from_name.to_owned(), state.button_count - *e.get());
                    }
                    Entry::Vacant(e) => {
                        e.insert(state.button_count);
                    }
                }
            }
        }
        match self.modules.get(to_name).unwrap_or(&Module::Untyped) {
            Module::Untyped => {}
            Module::Broadcaster => {
                for output in &self.outputs[to_name] {
                    self.queue_pulse(state, to_name, output, pulse);
                }
            }
            Module::FlipFlop => match pulse {
                Pulse::Low => {
                    let ff_state = state
                        .flip_flop_memory
                        .entry(to_name.to_owned())
                        .or_default();
                    *ff_state = !*ff_state;
                    let ff_pulse = if *ff_state { Pulse::High } else { Pulse::Low };
                    for output in &self.outputs[to_name] {
                        self.queue_pulse(state, to_name, output, ff_pulse);
                    }
                }
                Pulse::High => {}
            },
            Module::Conjunction => {
                let conj_inputs = &self.inputs[to_name];
                let conj_input_count = conj_inputs.len();
                let input_index = conj_inputs
                    .iter()
                    .position(|input| input == from_name)
                    .unwrap();
                let conj_state = state
                    .conjunction_memory
                    .entry(to_name.to_owned())
                    .or_default();
                let bit = 1u64 << input_index;
                match pulse {
                    Pulse::Low => *conj_state &= !bit,
                    Pulse::High => *conj_state |= bit,
                }
                let all_bits = (1 << conj_input_count) - 1;
                let conj_pulse = if *conj_state == all_bits {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for output in &self.outputs[to_name] {
                    self.queue_pulse(state, to_name, output, conj_pulse);
                }
            }
        }
    }

    fn push_button(&self, state: &mut NetworkState) {
        state.button_count += 1;
        self.process_pulse(state, "button", "broadcaster", Pulse::Low);
    }

    fn queue_pulse(&self, state: &mut NetworkState, from_name: &str, to_name: &str, pulse: Pulse) {
        state
            .queue
            .push_back((pulse, from_name.to_owned(), to_name.to_owned()));
    }

    fn process(&self, state: &mut NetworkState) {
        while let Some((pulse, from_name, to_name)) = state.queue.pop_front() {
            self.process_pulse(state, &from_name, &to_name, pulse);
        }
    }
}

fn solve_p1(input: &str) -> u64 {
    let network = Network::parse(input);
    let mut state = NetworkState::default();
    for _ in 0..1000 {
        network.push_button(&mut state);
        network.process(&mut state);
    }
    state.low_pulse_count * state.high_pulse_count
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn solve_p2(input: &str) -> u64 {
    let network = Network::parse(input);
    let mut state = NetworkState::default();
    loop {
        let rx_conj = &network.inputs["rx"][0];
        let rx_conj_deps = &network.inputs[rx_conj];
        if let Some(periods) = rx_conj_deps
            .iter()
            .map(|dep| state.high_signal_periods.get(dep).copied())
            .collect::<Option<Vec<u64>>>()
        {
            break periods
                .into_iter()
                .fold(1u64, |acc, period| lcm(acc, period));
        }
        network.push_button(&mut state);
        network.process(&mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE_INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(EXAMPLE_INPUT_1), 32000000);
        assert_eq!(solve_p1(EXAMPLE_INPUT_2), 11687500);
    }
}
