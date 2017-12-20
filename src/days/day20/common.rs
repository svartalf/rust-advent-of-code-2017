use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Particle {
    x: isize,
    y: isize,
    z: isize,

    v_x: isize,
    v_y: isize,
    v_z: isize,

    a_x: isize,
    a_y: isize,
    a_z: isize,
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"p=<(-{0,1}\d+),(-{0,1}\d+),(-{0,1}\d+)>, v=<(-{0,1}\d+),(-{0,1}\d+),(-{0,1}\d+)>, a=<(-{0,1}\d+),(-{0,1}\d+),(-{0,1}\d+)>").unwrap();
        let captures = reg.captures(s).unwrap();

        Ok(Particle {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
            z: captures.get(3).unwrap().as_str().parse().unwrap(),
            v_x: captures.get(4).unwrap().as_str().parse().unwrap(),
            v_y: captures.get(5).unwrap().as_str().parse().unwrap(),
            v_z: captures.get(6).unwrap().as_str().parse().unwrap(),
            a_x: captures.get(7).unwrap().as_str().parse().unwrap(),
            a_y: captures.get(8).unwrap().as_str().parse().unwrap(),
            a_z: captures.get(9).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Particle;

    #[test]
    fn day20_common_parse() {
        let input = "p=<-1724,-1700,5620>, v=<44,-10,-107>, a=<2,6,-9>";
        let p = Particle::from_str(input).unwrap();
        println!("{:#?}", p);
    }
}
//
