// Based on the https://math.stackexchange.com/a/163101
fn coords(n: f64) -> (f64, f64) {
    let k = ((n.sqrt() - 1.0) / 2.0).ceil();
    let mut t: f64 = 2.0 * k + 1.0;
    let mut m = t.powi(2);
    t -= 1.0;

    if n >= m - t {
        return (k - (m - n), -k);
    } else {
        m -= t;
    }

    if n >= m - t {
        return (-k, -k + (m - n));
    } else {
        m -= t;
    }

    if n >= m - t {
        return (-k + (m -n), k);
    } else {
        return (k, k - (m - n - t));
    }
}

pub fn distance(input: usize) -> usize {
    let (x, y) = coords(input as f64);

    let res = x.abs() + y.abs();
    res.ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::distance;

    #[test]
    fn day03_part1_test1() {
        assert_eq!(0, distance(1));
    }

    #[test]
    fn day03_part1_test2() {
        assert_eq!(3, distance(12));
    }

    #[test]
    fn day03_part1_test3() {
        assert_eq!(2, distance(23));
    }

    #[test]
    fn day03_part1_test4() {
        assert_eq!(31, distance(1024));
    }
}
