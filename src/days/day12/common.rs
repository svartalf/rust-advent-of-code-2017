use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    input.lines()
        .map(|line| {
            let mut parts = line.splitn(3, ' ');
            let source = parts.next().unwrap().parse::<usize>().unwrap();
            let targets = parts.skip(1).next().unwrap().split(", ").map(|part| {
                part.parse::<usize>().unwrap()
            }).collect::<Vec<_>>();
            (source, targets)
        }).for_each(|(source, targets)| {
            map.insert(source, targets);
        });

    map
}
