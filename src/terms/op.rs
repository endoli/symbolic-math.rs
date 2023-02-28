use std::{collections::HashMap, hash::Hash, rc::Rc};

use num::Num;

use std::ops::Add;

use crate::expression::{Expression, Expr};

impl<T: Num + Add + Copy + Hash + 'static> Add for Expr<T> {
    type Output = Expr<T>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.0.as_ref(), rhs.0.as_ref()) {
            // adding two number constants
            (Expression::Number(a), Expression::Number(b)) => 
                Expr(Expression::from(*a + *b).into()),

            // default branch
            (lhs_ref, rhs_ref) => {
                let mut hash_map = HashMap::new();
                if lhs_ref == rhs_ref {
                    hash_map.insert(self.clone(), T::one() + T::one());
                } else {
                    hash_map.insert(self.clone(), T::one());
                    hash_map.insert(rhs.clone(), T::one());
                }
                Expr(Rc::new(Expression::Add { coeff: T::zero(), dict: hash_map }))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let num1 = Rc::new(Expression::from(1));
        let num2 = Rc::new(Expression::from(2));
        let num3 = Rc::new(Expression::from(3));
        assert_eq!(Expr(num1.clone()) + Expr(num2.clone()), Expr(num2.clone()) + Expr(num1.clone()));
        assert_eq!(Expr(num1.clone()) + Expr(num2.clone()), Expr(num3.clone()));
    }
}