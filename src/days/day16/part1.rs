use super::common::{Step, Queue, parse_steps};

pub fn parse(programs: &str, raw_steps: &str) -> String {
    let steps = parse_steps(raw_steps);

    let mut queue: Queue = programs.chars().collect();

    steps.iter().for_each(|step| {
        match step {
            &Step::Spin(offset) => queue.spin(offset),
            &Step::Exchange(pos1, pos2) => queue.exchange(pos1, pos2),
            &Step::Partner(prog1, prog2) => queue.partner(prog1, prog2),
        }
    });

    queue.as_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::parse;

    #[test]
    fn day16_part1_test1() {
        let programs = "abcde";
        let steps = "s1,x3/4,pe/b";

        assert_eq!("baedc", &parse(programs, steps, 1));
    }
}
