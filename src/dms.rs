use std::fmt;

use crate::traits::FloatAngle;

#[derive(Debug, Clone, Copy)]
pub struct DMS<T: FloatAngle> {
    pub sign: bool,
    pub degree: usize,
    pub minute: u8,
    pub second: T,
}

impl<T: FloatAngle> DMS<T> {
    pub fn new(sign :bool, degree: usize, minute: u8, second: T) -> Self {
        Self { sign, degree, minute, second }
    }
    pub fn to_decimal(&self) -> T {
        let num = T::from(self.degree).unwrap()
            + T::from(self.minute).unwrap() / T::from(60).unwrap()
            + self.second / T::from(3600).unwrap();
        if self.sign { num } else { -num }
    }
    pub fn from_decimal(mut decimal: T) -> Self {
        let sign = decimal >= T::zero();
        if !sign { decimal = -decimal; }
        let degree: usize = decimal.to_usize().unwrap();
        decimal = decimal - T::from(degree).unwrap();
        decimal = decimal * T::from(60).unwrap();
        let minute = decimal.to_u8().unwrap();
        decimal = decimal - T::from(minute).unwrap();
        let second = decimal * T::from(60).unwrap();

        Self { sign, degree, minute, second }
    }
}

impl<T: FloatAngle> fmt::Display for DMS<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign {
            write!(f, "{0}°{1: <02}\'{2: <02.5}\"", self.degree, self.minute, self.second)
        } else {
            write!(f, "-{0}°{1: <02}\'{2: <02.5}\"", self.degree, self.minute, self.second)
        }
    }
}