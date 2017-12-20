use std::str::FromStr;

use super::common::Particle;

pub fn parse(input: &str, iterations: usize) -> usize {
    let mut particles = input.lines()
        .map(Particle::from_str)
        .collect::<Result<Vec<Particle>, _>>()
        .unwrap();

    let (close_one, _) = particles.iter_mut()
        .map(|particle| {
            for _ in 0..iterations {
                particle.step();
            }
            particle
        })
        .enumerate()
        .min_by_key(|&(_, ref particle)| particle.distance())
        .unwrap();

    close_one
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day20_part1_test1() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
";
        assert_eq!(0, parse(input, 3));
    }

}