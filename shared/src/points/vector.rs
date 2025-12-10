use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sqrt,
{
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    pub fn cross(&self, rhs: &Vector<T>) -> Vector<T> {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vector<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude_relative(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vector<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

forward_ref_binop! {
    [T]
    impl Add for Vector<T>, Vector<T>
    where T: Copy + Add<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl AddAssign for Vector<T>, Vector<T>
    where T: Copy + AddAssign
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign<Vector<T>> for Vector<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_binop! {
    [T]
    impl Sub for Vector<T>, Vector<T>
    where T: Copy +Sub<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl SubAssign for Vector<T>, Vector<T>
    where T: Copy + SubAssign
}

impl<T> Mul<T> for Vector<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

forward_ref_binop! {
    [T]
    impl Mul for Vector<T>, T
    where T: Copy + Mul<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl MulAssign for Vector<T>, T
    where T: Copy + MulAssign
}

impl<T> Div<T> for Vector<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

forward_ref_binop! {
    [T]
    impl Div for Vector<T>, T
    where T: Copy + Div<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl DivAssign for Vector<T>, T
    where T: Copy + DivAssign
}

impl<T> Rem<T> for Vector<T>
where
    T: Copy + Rem<Output = T>,
{
    type Output = Vector<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl<T> RemAssign<T> for Vector<T>
where
    T: Copy + RemAssign,
{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}

forward_ref_binop! {
    [T]
    impl Rem, rem for Vector<T>, T
    where T: Copy + Rem<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl RemAssign, rem_assign for Vector<T>, T
    where T: Copy + RemAssign
}

impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> MulAssign<Vector<T>> for Vector<T>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: Vector<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

forward_ref_binop! {
    [T]
    impl Mul for Vector<T>, Vector<T>
    where T: Copy + Mul<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl MulAssign for Vector<T>, Vector<T>
    where T: Copy + MulAssign
}

impl<T> Div<Vector<T>> for Vector<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vector<T>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> DivAssign<Vector<T>> for Vector<T>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: Vector<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

forward_ref_binop! {
    [T]
    impl Div for Vector<T>, Vector<T>
    where T: Copy + Div<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl DivAssign for Vector<T>, Vector<T>
    where T: Copy + DivAssign
}

impl<T> Rem<Vector<T>> for Vector<T>
where
    T: Copy + Rem<Output = T>,
{
    type Output = Vector<T>;

    fn rem(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}

impl<T> RemAssign<Vector<T>> for Vector<T>
where
    T: Copy + RemAssign,
{
    fn rem_assign(&mut self, rhs: Vector<T>) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

forward_ref_binop! {
    [T]
    impl Rem, rem for Vector<T>, Vector<T>
    where T: Copy +Rem<Output = T>
}

forward_ref_op_assign! {
    [T]
    impl RemAssign, rem_assign for Vector<T>, Vector<T>
    where T: Copy + RemAssign
}
