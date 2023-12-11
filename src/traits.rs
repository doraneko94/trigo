use std::fmt::{Debug, Display};

use num_traits::{Float, FloatConst};

pub trait FloatAngle: Float + FloatConst + Debug + Display {}

impl FloatAngle for f32 {}
impl FloatAngle for f64 {}