pub fn solve_linear_diophantine(a: isize, b: isize, c: isize) -> Option<(isize, isize)> {
    let (x0, y0, gcd) = extended_euclid(a, b);

    if c % gcd != 0 {
        return None;
    }

    let scale = c / gcd;
    let mut x = x0 * scale;
    let mut y = y0 * scale;

    let a_gcd = a / gcd;
    let b_gcd = -b / gcd;

    let k = ((-x) as f64 / b_gcd as f64).ceil() as isize;
    x += k * b_gcd;
    y += k * a_gcd;

    if x > 0 && y > 0 {
        return Some((x, y));
    }

    None
}

pub fn extended_euclid(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        return (0, 1, b);
    }

    let (x1, y1, gcd) = extended_euclid(b % a, a);

    (y1 - (b / a) * x1, x1, gcd)
}

pub trait Digits {
    fn count_digits(&self) -> usize;
}

impl Digits for u32 {
    fn count_digits(&self) -> usize {
        if *self < 10 {
            1
        } else if *self < 100 {
            2
        } else if *self < 1_000 {
            3
        } else if *self < 10_000 {
            4
        } else if *self < 100_000 {
            5
        } else if *self < 1_000_000 {
            6
        } else if *self < 10_000_000 {
            7
        } else if *self < 100_000_000 {
            8
        } else if *self < 1_000_000_000 {
            9
        } else {
            10
        }
    }
}

impl Digits for usize {
    fn count_digits(&self) -> usize {
        if *self < 10 {
            1
        } else if *self < 100 {
            2
        } else if *self < 1_000 {
            3
        } else if *self < 10_000 {
            4
        } else if *self < 100_000 {
            5
        } else if *self < 1_000_000 {
            6
        } else if *self < 10_000_000 {
            7
        } else if *self < 100_000_000 {
            8
        } else if *self < 1_000_000_000 {
            9
        } else if *self < 10_000_000_000 {
            10
        } else if *self < 100_000_000_000 {
            11
        } else if *self < 1_000_000_000_000 {
            12
        } else if *self < 10_000_000_000_000 {
            13
        } else if *self < 100_000_000_000_000 {
            14
        } else if *self < 1_000_000_000_000_000 {
            15
        } else if *self < 10_000_000_000_000_000 {
            16
        } else if *self < 100_000_000_000_000_000 {
            17
        } else if *self < 1_000_000_000_000_000_000 {
            18
        } else if *self < 10_000_000_000_000_000_000 {
            19
        } else {
            20
        }
    }
}
