use std::str;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
pub enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

pub struct Queue(VecDeque<char>);

impl Queue {

    pub fn spin(&mut self, offset: usize) {
        for _ in 0..offset {
            let chr = self.0.pop_back().unwrap();
            self.0.push_front(chr);
        }
    }

    pub fn exchange(&mut self, idx1: usize, idx2: usize) {
        self.0.swap(idx1, idx2);
    }

    pub fn partner(&mut self, prog1: char, prog2: char) {
        let (prog1_idx, _) = self.0.iter().enumerate().find(|&(_, p)| p == &prog1).unwrap();
        let (prog2_idx, _) = self.0.iter().enumerate().find(|&(_, p)| p == &prog2).unwrap();
        self.0.swap(prog1_idx, prog2_idx);
    }

    pub fn as_string(&self) -> String {
        self.0.iter().collect()
    }
}

impl FromIterator<char> for Queue {
    fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
        let inner = VecDeque::from_iter(iter);
        Queue(inner)
    }
}

pub fn parse_steps(input: &str) -> Vec<Step> {
    input.split(',').map(|step| {
        let mut chars = step.chars();
        match chars.next() {
            Some('s') => {
                let offset = chars
                    .collect::<String>()
                    .parse::<usize>().unwrap();

                Step::Spin(offset)
            },
            Some('x') => {
                let pos1 = chars
                    .take_while(|chr| *chr != '/')
                    .collect::<String>()
                    .parse::<usize>().unwrap();
                let pos2 = step.chars()
                    .skip_while(|chr| *chr != '/')
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>().unwrap();
                Step::Exchange(pos1, pos2)
            },
            Some('p') => {
                let prog1 = chars
                    .take_while(|chr| *chr != '/')
                    .next().unwrap();
                let prog2 = step.chars()
                    .skip_while(|chr| *chr != '/')
                    .skip(1)
                    .next().unwrap();
                Step::Partner(prog1, prog2)
            },
            _ => unimplemented!(),
        }
    }).collect()
}
