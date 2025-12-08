use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::traits::{Modulo, ModuloAssign};

#[derive(Debug, Clone, Copy)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn cross(&self, rhs: &Point3d) -> Point3d {
        Point3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Point3d) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn non_sqrt_magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Add for Point3d {
    type Output = Point3d;

    fn add(self, rhs: Self) -> Self::Output {
        Point3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Point3d> for Point3d {
    fn add_assign(&mut self, rhs: Point3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

forward_ref_binop!(impl Add, add for Point3d, Point3d);
forward_ref_op_assign!(impl AddAssign, add_assign for Point3d, Point3d);

impl Sub for Point3d {
    type Output = Point3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Point3d> for Point3d {
    fn sub_assign(&mut self, rhs: Point3d) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_binop!(impl Sub, sub for Point3d, Point3d);
forward_ref_op_assign!(impl SubAssign, sub_assign for Point3d, Point3d);

impl Mul<f64> for Point3d {
    type Output = Point3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Point3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Point3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

forward_ref_binop!(impl Mul, mul for Point3d, f64);
forward_ref_op_assign!(impl MulAssign, mul_assign for Point3d, f64);

impl Div<f64> for Point3d {
    type Output = Point3d;

    fn div(self, rhs: f64) -> Self::Output {
        Point3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Point3d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

forward_ref_binop!(impl Div, div for Point3d, f64);
forward_ref_op_assign!(impl DivAssign, div_assign for Point3d, f64);

impl Modulo<f64> for Point3d {
    type Output = Point3d;

    fn modulo(self, rhs: f64) -> Self::Output {
        Point3d {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl ModuloAssign<f64> for Point3d {
    fn modulo_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}

forward_ref_binop!(impl Modulo, modulo for Point3d, f64);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Point3d, f64);

impl Mul<Point3d> for Point3d {
    type Output = Point3d;

    fn mul(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Point3d> for Point3d {
    fn mul_assign(&mut self, rhs: Point3d) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

forward_ref_binop!(impl Mul, mul for Point3d, Point3d);
forward_ref_op_assign!(impl MulAssign, mul_assign for Point3d, Point3d);

impl Div<Point3d> for Point3d {
    type Output = Point3d;

    fn div(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<Point3d> for Point3d {
    fn div_assign(&mut self, rhs: Point3d) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

forward_ref_binop!(impl Div, div for Point3d, Point3d);
forward_ref_op_assign!(impl DivAssign, div_assign for Point3d, Point3d);

impl Modulo<Point3d> for Point3d {
    type Output = Point3d;

    fn modulo(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}

impl ModuloAssign<Point3d> for Point3d {
    fn modulo_assign(&mut self, rhs: Point3d) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

forward_ref_binop!(impl Modulo, modulo for Point3d, Point3d);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Point3d, Point3d);
