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

pub struct Compositions {
    m: usize,
    current: Vec<usize>,
    done: bool,
}

impl Compositions {
    pub fn new(n: usize, m: usize) -> Self {
        let mut current = vec![0; m];
        if m > 0 {
            current[m - 1] = n;
        }

        Compositions {
            m,
            current,
            done: m == 0, // if m == 0, no compositions exist
        }
    }
}

impl Iterator for Compositions {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Yield the current composition
        let result = self.current.clone();

        // Generate next composition
        // Find the rightmost index i where current[i] > 0
        let mut i = self.m - 1;
        while i > 0 && self.current[i] == 0 {
            i -= 1;
        }

        if i == 0 {
            // We've reached the final composition
            self.done = true;
        } else {
            // Move 1 from position i to position i-1
            self.current[i] -= 1;
            self.current[i - 1] += 1;

            // Push remaining sum all the way to the right
            let mut remainder: usize = 0;
            for j in i..self.m {
                remainder += self.current[j];
                self.current[j] = 0;
            }
            self.current[self.m - 1] = remainder;
        }

        Some(result)
    }
}
