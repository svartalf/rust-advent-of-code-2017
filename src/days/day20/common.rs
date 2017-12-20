use std::cmp::{Eq, PartialEq};
use std::str::FromStr;

#[derive(Debug)]
pub struct Particle {
    pub x: isize,
    pub y: isize,
    pub z: isize,

    v_x: isize,
    v_y: isize,
    v_z: isize,

    a_x: isize,
    a_y: isize,
    a_z: isize,
}

impl Particle {
    pub fn step(&mut self) {
        self.v_x += self.a_x;
        self.v_y += self.a_y;
        self.v_z += self.a_z;
        self.x += self.v_x;
        self.y += self.v_y;
        self.z += self.v_z;
    }

    pub fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Particle {}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(", ").collect::<Vec<&str>>();
        let pos = parts[0].split(',').collect::<Vec<&str>>();
        let vel = parts[1].split(',').collect::<Vec<&str>>();
        let acc = parts[2].split(',').collect::<Vec<&str>>();
        println!("{:?}", pos);
        println!("{:?}", vel);
        println!("{:?}", acc);

        Ok(Particle {
            x: pos[0].replace("p=<", "").trim().parse().unwrap(),
            y: pos[1].parse().unwrap(),
            z: pos[2].replace(">", "").parse().unwrap(),
            v_x: vel[0].replace("v=<", "").trim().parse().unwrap(),
            v_y: vel[1].parse().unwrap(),
            v_z: vel[2].replace(">", "").parse().unwrap(),
            a_x: acc[0].replace("a=<", "").trim().parse().unwrap(),
            a_y: acc[1].parse().unwrap(),
            a_z: acc[2].replace(">", "").parse().unwrap(),
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
        Particle::from_str(input).unwrap();
    }

    #[test]
    fn day20_common_step() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>";
        let mut p = Particle::from_str(input).unwrap();

        p.step();
        assert_eq!(p, Particle {x: 4, y: 0, z: 0, v_x: 1, v_y: 0, v_z: 0, a_x: -1, a_y: 0, a_z: 0});

        p.step();
        assert_eq!(p, Particle {x: 4, y: 0, z: 0, v_x: 0, v_y: 0, v_z: 0, a_x: -1, a_y: 0, a_z: 0});

        p.step();
        assert_eq!(p, Particle {x: 3, y: 0, z: 0, v_x: -1, v_y: 0, v_z: 0, a_x: -1, a_y: 0, a_z: 0});
    }
}
//
