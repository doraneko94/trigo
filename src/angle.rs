use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::dms::DMS;
use crate::traits::FloatAngle;

#[derive(Clone, Copy)]
pub struct Angle<T: FloatAngle> {
    rad: T,
}

macro_rules! method {
    ($f: ident, $t: ty) => {
        pub fn $f(self) -> $t {
            self.rad.$f()
        }
    };
}

macro_rules! method_cmp {
    ($f: ident) => {
        pub fn $f(self, other: Self) -> Self {
            Self::from_rad(self.rad.$f(other.rad))
        }
    };
}

macro_rules! function_ret_self {
    ($f: ident) => {
        pub fn $f() -> Self {
            Self::from_rad(T::$f())
        }
    };
}

macro_rules! function_arc {
    ($f: ident) => {
        pub fn $f(value: T) -> Self {
            Self::from_rad(value.$f())
        }
    };
}

impl <T: FloatAngle> Angle<T> {
    pub fn from_deg(deg: T) -> Self {
        Self::from_rad(deg.to_radians())
    }
    pub fn from_rad(rad: T) -> Self {
        Self { rad }
    }
    pub fn from_dms(dms: DMS<T>) -> Self {
        Self::from_deg(dms.to_decimal())
    }

    pub fn deg(self) -> T {
        self.rad.to_degrees()
    }
    pub fn rad(self) -> T {
        self.rad
    }
    pub fn to_dms(self) -> DMS<T> {
        DMS::from_decimal(self.deg())
    }

    pub fn bind(&mut self) {
        let rad = _bind_core(self.rad);
        if rad > T::PI() { self.rad = rad - T::PI() }
        else if rad < -T::PI() { self.rad = rad + T::PI() }
        else { self.rad = rad }
    }
    pub fn bind_positive(&mut self) {
        let rad = _bind_core(self.rad);
        if rad < T::zero() { self.rad = T::from(2).unwrap() * T::PI() - rad }
        else { self.rad = rad }
    }

    function_ret_self!(nan);
    function_ret_self!(infinity);
    function_ret_self!(neg_infinity);
    function_ret_self!(min_value);
    function_ret_self!(min_positive_value);
    function_ret_self!(max_value);

    method!(is_nan, bool);
    method!(is_infinite, bool);
    method!(is_finite, bool);
    method!(is_normal, bool);
    method!(is_sign_positive, bool);
    method!(is_sign_negative, bool);

    method!(recip, T);
    method!(sqrt, T);
    method!(exp, T);
    method!(exp2, T);
    method!(ln, T);
    method!(log2, T);
    method!(log10, T);
    method!(cbrt, T);
    method!(sin, T);
    method!(cos, T);
    method!(tan, T);
    method!(sin_cos, (T, T));
    method!(exp_m1, T);
    method!(ln_1p, T);
    method!(sinh, T);
    method!(cosh, T);
    method!(tanh, T);

    function_arc!(asin); // [-pi/2, pi/2]
    function_arc!(acos); // [0, pi]
    function_arc!(atan); // [-pi/2, pi/2]
    function_arc!(asinh);
    function_arc!(acosh);
    function_arc!(atanh);
    pub fn atan2(y: T, x: T) -> Self {
        Self::from_rad(y.atan2(x))
    } // atan(y/x), [-pi/2, pi/2]

    method_cmp!(max);
    method_cmp!(min);
    method_cmp!(abs_sub);

    pub fn abs(self) -> Self {
        Self::from_rad(self.rad.abs())
    }
    pub fn powi(self, n: i32) -> T {
        self.rad.powi(n)
    }
    pub fn powf(self, n: T) -> T {
        self.rad.powf(n)
    }
    pub fn log(self, base: T) -> T {
        self.rad.log(base)
    }
}

impl<T: FloatAngle> Add<Angle<T>> for Angle<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad + rhs.rad)
    }
}
impl<T: FloatAngle> AddAssign<Angle<T>> for Angle<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.rad = self.rad + rhs.rad;
    }
}

impl<T: FloatAngle> Sub<Angle<T>> for Angle<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad - rhs.rad)
    }
}
impl<T: FloatAngle> SubAssign<Angle<T>> for Angle<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.rad = self.rad - rhs.rad;
    }
}

impl<T: FloatAngle> Mul<T> for Angle<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::from_rad(self.rad * rhs)
    }
}
impl<T: FloatAngle> MulAssign<T> for Angle<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.rad = self.rad * rhs
    }
}

impl<T: FloatAngle> Div<T> for Angle<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::from_rad(self.rad / rhs)
    }
}
impl<T: FloatAngle> DivAssign<T> for Angle<T> {
    fn div_assign(&mut self, rhs: T) {
        self.rad = self.rad / rhs
    }
}

macro_rules! impl_mul {
    ($t: ty) => {
        impl Mul<Angle<$t>> for $t {
            type Output = Angle<$t>;

            fn mul(self, rhs: Angle<$t>) -> Self::Output {
                Self::Output::from_rad(self * rhs.rad)
            }
        }
    };
}

impl_mul!(f32);
impl_mul!(f64);

fn _bind_core<T: FloatAngle>(value: T) -> T {
    value % (T::from(2).unwrap() * T::PI())
}