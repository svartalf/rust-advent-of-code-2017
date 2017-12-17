use std::collections::HashSet;

use super::common::{Step, Queue, parse_steps};

// Probing steps cycle until find repeatable pattern
fn probe(steps: Vec<Step>, programs: &str, iterations: usize) -> usize {
    let mut probes: HashSet<String> = HashSet::new();
    let mut queue: Queue = programs.chars().collect();

    for iter in 0..iterations {
        for step in steps.iter() {
            match step {
                &Step::Spin(offset) => queue.spin(offset),
                &Step::Exchange(pos1, pos2) => queue.exchange(pos1, pos2),
                &Step::Partner(prog1, prog2) => queue.partner(prog1, prog2),
            }
        }

        let res = queue.as_string();

        if probes.contains(&res) {
            return iter;
        } else {
            probes.insert(res);
        }
    }

    0
}

pub fn parse(programs: &str, raw_steps: &str, iterations: usize) -> String {
    let steps = parse_steps(raw_steps);

    let mut queue: Queue = programs.chars().collect();
    let cycle = probe(steps.clone(), programs, iterations);
    let left = iterations % cycle;
    for _ in 0..left {
        for step in steps.iter() {
            match step {
                &Step::Spin(offset) => queue.spin(offset),
                &Step::Exchange(pos1, pos2) => queue.exchange(pos1, pos2),
                &Step::Partner(prog1, prog2) => queue.partner(prog1, prog2),
            }
        }
    }

    queue.as_string()
}
