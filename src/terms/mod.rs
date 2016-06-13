// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use num::bigint::BigInt;
use num::rational::BigRational;
use std::fmt;

pub trait Expression: fmt::Debug {
    fn apply(&self) -> Term;
}

#[derive(Debug)]
pub enum Term {
    Integer(BigInt),
    Expression(Box<Expression>),
    Rational(BigRational),
    Symbol(String),
}

/// Pass `Expression` methods along to the underlying value.
impl Expression for Term {
    fn apply(&self) -> Term {
        match *self {
            Term::Integer(ref val) => Term::from(val.clone()),
            Term::Expression(ref expr) => expr.apply(),
            Term::Rational(ref rational) => Term::from(rational.clone()),
            Term::Symbol(ref symbol) => Term::from(symbol.clone()),
        }
    }
}

/// Construct a `Term::Integer`.
impl From<i64> for Term {
    fn from(value: i64) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<i8> for Term {
    fn from(value: i8) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<i16> for Term {
    fn from(value: i16) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<isize> for Term {
    fn from(value: isize) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<u64> for Term {
    fn from(value: u64) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<u8> for Term {
    fn from(value: u8) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<u16> for Term {
    fn from(value: u16) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<u32> for Term {
    fn from(value: u32) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<usize> for Term {
    fn from(value: usize) -> Self {
        Term::Integer(BigInt::from(value))
    }
}

/// Construct a `Term::Integer`.
impl From<BigInt> for Term {
    fn from(value: BigInt) -> Self {
        Term::Integer(value)
    }
}

/// Construct a `Term::Expression`.
impl From<Box<Expression>> for Term {
    fn from(value: Box<Expression>) -> Self {
        Term::Expression(value)
    }
}

/// Construct a `Term::Rational`.
impl From<BigRational> for Term {
    fn from(value: BigRational) -> Self {
        Term::Rational(value)
    }
}

/// Construct a `Term::Symbol`.
impl From<String> for Term {
    fn from(value: String) -> Self {
        Term::Symbol(value)
    }
}
