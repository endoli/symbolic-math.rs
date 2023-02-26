// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::rational::BigRational;
use std::ops;
use terms::{Expression, Term};

/// Add 2 terms together.
#[derive(Debug)]
pub struct Add {
    #[allow(missing_docs)]
    pub lhs: Term,
    #[allow(missing_docs)]
    pub rhs: Term,
}

impl Add {
    /// Construct a new instance of `Add`.
    fn new<L, R>(lhs: L, rhs: R) -> Term
    where
        Term: From<L>,
        Term: From<R>,
    {
        Term::Expression(Box::new(Add {
            lhs: Term::from(lhs),
            rhs: Term::from(rhs),
        }))
    }
}

impl Expression for Add {
    fn apply(&self) -> Term {
        let lhs = self.lhs.apply();
        let rhs = self.rhs.apply();
        match (lhs, rhs) {
            (Term::Integer(lhs), Term::Integer(rhs)) => Term::Integer(lhs + rhs),
            (Term::Rational(lhs), Term::Integer(rhs)) => {
                Term::Rational(lhs + BigRational::from_integer(rhs))
            }
            (Term::Rational(lhs), Term::Rational(rhs)) => Term::Rational(lhs + rhs),
            (Term::Integer(lhs), Term::Rational(rhs)) => {
                Term::Rational(BigRational::from_integer(lhs) + rhs)
            }
            (lhs, rhs) => lhs + rhs,
        }
    }
}

impl ops::Add for Term {
    type Output = Term;

    fn add(self, rhs: Term) -> Term {
        Add::new(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use num::bigint::BigInt;
    use terms::{Expression, Term};

    #[test]
    fn it_works() {
        let sum = Term::from(3) + Term::from(5);
        match sum.apply() {
            Term::Integer(val) => assert_eq!(val, BigInt::from(8)),
            _ => panic!(),
        }
        let sum = sum + Term::from(5);
        match sum.apply() {
            Term::Integer(val) => assert_eq!(val, BigInt::from(13)),
            _ => panic!(),
        }
    }
}
