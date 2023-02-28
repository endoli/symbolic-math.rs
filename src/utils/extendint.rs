// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::Integer;
use std::{cmp, ops, fmt};

/// Defines a extended integer, which can be all integers and positive/negative infinity.
#[derive(PartialEq, Eq)]
pub enum ExtendI<I> where I: Integer {
    Int(I),
    NegInf,
    PosInf,
}

impl<I: Integer + fmt::Debug> fmt::Debug for ExtendI<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PosInf => f.pad_integral(true, "", "Inf"),
            Self::NegInf => f.pad_integral(false, "", "Inf"),
            Self::Int(val) => val.fmt(f),
        }
    }
}

impl<I: Integer> PartialOrd for ExtendI<I> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: Integer> Ord for ExtendI<I> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::PosInf, Self::PosInf) | (Self::NegInf, Self::NegInf) => cmp::Ordering::Equal,
            (Self::PosInf, _) | (_, Self::NegInf) => cmp::Ordering::Greater,
            (_, Self::PosInf) | (Self::NegInf, _) => cmp::Ordering::Less,
            (Self::Int(v1), Self::Int(v2)) => v1.cmp(v2),
        }
    }
}

impl<I: Integer> From<I> for ExtendI<I> {
    fn from(value: I) -> Self {
        Self::Int(value)
    }
}

impl<I: Integer> ops::Neg for ExtendI<I> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::PosInf => Self::NegInf,
            Self::Int(val) => Self::Int(I::zero()-val),
            Self::NegInf => Self::PosInf,
        }
    }
}

impl<I: Integer> ops::Add for ExtendI<I> {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::PosInf, Self::NegInf) | (Self::NegInf, Self::PosInf) => None,
            (Self::PosInf, _) | (_, Self::PosInf) => Some(Self::PosInf),
            (Self::NegInf, _) | (_, Self::NegInf) => Some(Self::NegInf),
            (Self::Int(v1), Self::Int(v2)) => Some(Self::Int(v1 + v2)),
        }
    }
}

impl<I: Integer> ops::Sub for ExtendI<I> {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<I: Integer> ops::Mul for ExtendI<I> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::PosInf, Self::PosInf) | (Self::NegInf, Self::NegInf) => Some(Self::PosInf),
            (Self::PosInf, Self::NegInf) | (Self::NegInf, Self::PosInf) => Some(Self::NegInf),
            (Self::PosInf, Self::Int(x)) | (Self::Int(x), Self::PosInf) =>
                    if x > I::zero() { Some(Self::PosInf) } else if x < I::zero() { Some(Self::NegInf) } else { None }
            (Self::Int(v1), Self::Int(v2)) => Some(Self::Int(v1 * v2)),
            (Self::NegInf, Self::Int(x)) | (Self::Int(x), Self::NegInf) =>
                    if x > I::zero() { Some(Self::NegInf) } else if x < I::zero() { Some(Self::PosInf) } else { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend() {
        assert!(ExtendI::Int(910) < ExtendI::PosInf);
        assert!(ExtendI::PosInf > ExtendI::Int(114514));
        assert!(ExtendI::Int(1919810) > ExtendI::Int(114514));
        assert!(ExtendI::Int(1) == ExtendI::Int(1));
        assert!(ExtendI::NegInf < ExtendI::Int(-10));
        assert!(ExtendI::Int(-10) > ExtendI::NegInf);
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(ExtendI::Int(123) + ExtendI::PosInf, Some(ExtendI::PosInf));
        assert_eq!(ExtendI::Int(123) + ExtendI::Int(456), Some(ExtendI::Int(579)));
        assert_eq!(ExtendI::NegInf + ExtendI::Int(456), Some(ExtendI::NegInf));
        assert_eq!(ExtendI::<i32>::PosInf + ExtendI::NegInf, None);
    }
}