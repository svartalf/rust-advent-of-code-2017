use std::convert::From;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Step {
    GroupStart, // {
    GroupEnd,  // }
    GarbageStart,  // <
    GarbageEnd, // >
    SkipNext,  // !
    Unknown, // all other chars
}

impl From<char> for Step {
    fn from(value: char) -> Step {
        match value {
            '{' => Step::GroupStart,
            '}' => Step::GroupEnd,
            '<' => Step::GarbageStart,
            '>' => Step::GarbageEnd,
            '!' => Step::SkipNext,
            _ => Step::Unknown,
        }
    }
}

#[derive(Debug)]
struct State {
    step: Step,
    groups: usize,
    garbage_count: usize,
    groups_depth: usize,
    garbage_depth: usize,
}

// -> (groups count, garbage chars count)
pub fn parse(data: &str) -> (usize, usize) {
    let mut state = State {
        step: Step::Unknown,
        groups: 0,
        garbage_count: 0,
        groups_depth: 0,
        garbage_depth: 0,
    };

    data.chars().fold(&mut state, |state, chr| {
        if state.step == Step::SkipNext {
            state.step = Step::Unknown;
            return state;
        }

        match Step::from(chr) {
            Step::GroupStart if state.garbage_depth == 0 => {
                state.groups_depth += 1;
                state.step = Step::Unknown;
            },
            Step::GroupEnd if state.groups_depth > 0 && state.garbage_depth == 0 => {
                state.groups += state.groups_depth;
                state.groups_depth -= 1;
                state.step = Step::Unknown;
            },
            Step::GarbageStart if state.garbage_depth == 0 => {
                state.garbage_depth += 1;
                state.step = Step::Unknown;
            },
            Step::GarbageEnd if state.garbage_depth > 0 => {
                state.garbage_depth -= 1;
                state.step = Step::Unknown;
            },
            Step::SkipNext => {
                state.step = Step::SkipNext;
            },
            _ if state.garbage_depth > 0 => {
                state.garbage_count += 1;
                state.step = Step::Unknown;
            },
            _ => {
                state.step = Step::Unknown;
            },
        }
        state
    });

    (state.groups, state.garbage_count)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day09_part1_test01() {
        let data = "{}";

        assert_eq!(1, parse(data).0);
    }

    #[test]
    fn day09_part1_test02() {
        let data = "{{{}}}";
        assert_eq!(6, parse(data).0);
    }

    #[test]
    fn day09_part1_test03() {
        let data = "{{},{}}";
        assert_eq!(5, parse(data).0);
    }

    #[test]
    fn day09_part1_test04() {
        let data = "{{{},{},{{}}}}";
        assert_eq!(16, parse(data).0);
    }

    #[test]
    fn day09_part1_test05() {
        let data = "{<a>,<a>,<a>,<a>}";
        assert_eq!(1, parse(data).0);
    }

    #[test]
    fn day09_part1_test06() {
        let data = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
        assert_eq!(9, parse(data).0);
    }

    #[test]
    fn day09_part1_test07() {
        let data = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
        assert_eq!(9, parse(data).0);
    }

    #[test]
    fn day09_part1_test08() {
        let data = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        assert_eq!(3, parse(data).0);
    }

    #[test]
    fn day09_part1_test09() {
        let data = "<>";
        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test10() {
        let data = "<random characters>";
        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test11() {
        let data = "<<<<>";
        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test12() {
        let data = "<{!>}>";
        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test13() {
        let data = "<!!>";
        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test14() {
        let data = "<!!!>>";

        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part1_test15() {
        let data = "<{o\"i!a,<{i<a>";

        assert_eq!(0, parse(data).0);
    }

    #[test]
    fn day09_part2_test01() {
        let data = "<>";

        assert_eq!(0, parse(data).1);
    }

    #[test]
    fn day09_part2_test02() {
        let data = "<random characters>";

        assert_eq!(17, parse(data).1);
    }

    #[test]
    fn day09_part2_test03() {
        let data = "<<<<>";

        assert_eq!(3, parse(data).1);
    }

    #[test]
    fn day09_part2_test04() {
        let data = "<{!>}>";

        assert_eq!(2, parse(data).1);
    }

    #[test]
    fn day09_part2_test05() {
        let data = "<!!>";

        assert_eq!(0, parse(data).1);
    }

    #[test]
    fn day09_part2_test06() {
        let data = "<!!!>>";

        assert_eq!(0, parse(data).1);
    }

    #[test]
    fn day09_part2_test07() {
        let data = "<{o\"i!a,<{i<a>";

        assert_eq!(10, parse(data).1);
    }
}
