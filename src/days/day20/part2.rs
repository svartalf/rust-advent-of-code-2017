use std::collections::HashMap;
use std::str::FromStr;

use super::common::Particle;

pub fn parse(input: &str, iterations: usize) -> usize {
    let mut particles: Vec<Particle> = input.lines()
        .map(Particle::from_str)
        .collect::<Result<Vec<Particle>, _>>()
        .unwrap();

    for _ in 0..iterations {
        particles.iter()
            // grouping particles by coordinates
            .fold(HashMap::new(), |mut acc, particle| {
                {
                    let key = (particle.x, particle.y, particle.z);
                    let entry = acc.entry(key).or_insert(0);
                    (*entry) += 1;
                }

                acc
            })
            .iter()
            .filter(|&(_, count)| *count > 1)
            .for_each(|(&(x, y, z), _)| {
                particles.retain(|particle| {
                    particle.x != x || particle.y != y || particle.z != z
                })
            });

        particles.iter_mut().for_each(|particle| particle.step());
    }

    particles.len()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn day20_part2_test1() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        assert_eq!(1, parse(input, 3));
    }

}