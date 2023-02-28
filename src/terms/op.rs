use std::{collections::HashMap, hash::Hash, rc::Rc, ops::{Sub, Neg}};

use num::Num;

use std::ops::Add;

use crate::{expression::{Expression, Expr}, utils::dict::remove_zeros};

impl<T: Num + Copy + Hash + 'static> Add for Expr<T> {
    type Output = Expr<T>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.0.as_ref(), rhs.0.as_ref()) {
            // adding two number constants
            (Expression::Number(a), Expression::Number(b)) => 
                Expr(Expression::from(*a + *b).into()),
            
            (Expression::Number(a), Expression::Add { coeff, dict }) |
            (Expression::Add { coeff, dict }, Expression::Number(a)) =>
                Expr(Expression::Add { coeff: *a + *coeff, dict: dict.clone() }.into()),
            
            (Expression::Add { coeff: c1, dict: d1 }, Expression::Add { coeff: c2, dict: d2 }) => {
                let mut dict = d1.clone();
                for (key, value) in d2 {
                    if let Some(dict_elem) = dict.get_mut(key) {
                        *dict_elem = *dict_elem + *value;
                    } else {
                        dict.insert(key.clone(), *value);
                    }
                }
                remove_zeros(&mut dict);
                Expr(Expression::Add { coeff: *c1 + *c2, dict }.into())
            },

            (Expression::Add { coeff, dict }, _) => {
                let mut new_dict = dict.clone();
                if let Some(dict_elem) = new_dict.get_mut(&rhs) {
                    *dict_elem = *dict_elem + T::one();
                    if *dict_elem == T::zero() {
                        new_dict.remove(&rhs);
                    }
                } else {
                    new_dict.insert(rhs.clone(), T::one());
                }
                Expr(Expression::Add { coeff: *coeff, dict: new_dict }.into())
            },
            (_, Expression::Add { coeff, dict }) => {
                let mut new_dict = dict.clone();
                if let Some(dict_elem) = new_dict.get_mut(&self) {
                    *dict_elem = *dict_elem + T::one();
                    if *dict_elem == T::zero() {
                        new_dict.remove(&self);
                    }
                } else {
                    new_dict.insert(rhs.clone(), T::one());
                }
                Expr(Expression::Add { coeff: *coeff, dict: new_dict }.into())
            }

            (Expression::Number(a), _) => {
                let mut dict = HashMap::new();
                dict.insert(rhs.clone(), T::one());
                Expr(Expression::Add { coeff: *a, dict }.into())
            },
            (_, Expression::Number(a)) => {
                let mut dict = HashMap::new();
                dict.insert(self.clone(), T::one());
                Expr(Expression::Add { coeff: *a, dict }.into())
            },

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

impl<T: Num + Copy + Hash + 'static> Neg for Expr<T> {
    type Output = Expr<T>;
    fn neg(self) -> Self::Output {
        match self.0.as_ref() {
            Expression::Number(a) => Expr::from(T::zero() - *a),
            Expression::Add { coeff, dict } => {
                let new_dict = dict.iter().map(|(k, v)| (k.clone(), T::zero() - *v)).collect();
                Expr(Expression::Add { coeff: T::zero() - *coeff, dict: new_dict }.into())
            },
            Expression::Mul { coeff, dict } => {
                Expr(Expression::Mul { coeff: T::zero() - *coeff, dict: dict.clone() }.into())
            },
            _ => {
                let mut dict = HashMap::new();
                dict.insert(self, T::zero() - T::one());
                Expr(Expression::Add { coeff: T::zero(), dict }.into())
            }
        }
    }
}

impl<T: Num + Copy + Hash + 'static> Sub for Expr<T> {
    type Output = Expr<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
        //? Performance?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let num1 = Expr(Rc::new(Expression::from(1)));
        let num2 = Expr(Rc::new(Expression::from(2)));
        let num3 = Expr(Rc::new(Expression::from(3)));
        let num1_clone = Expr(Rc::new(Expression::from(1)));
        let num2_clone = Expr(Rc::new(Expression::Number(2)));
        let sym_x = Expr(Rc::new(Expression::<i32>::Symbol("x")));
        assert_eq!(num1.clone() + num2.clone(), num2_clone.clone() + num1_clone.clone());
        assert_eq!(num1.clone() + num2.clone(), num3.clone());
        assert_eq!(sym_x.clone() + num1.clone(), num1.clone() + sym_x.clone());
        assert_eq!((sym_x.clone() + num1.clone()) + num2.clone(), (sym_x.clone() + num3.clone()));
        assert_eq!((sym_x.clone() + num1.clone()) + sym_x.clone(), (sym_x.clone() + sym_x.clone()) + num1.clone());
        // assert_eq!((sym_x.clone() + num1.clone()) + (sym_x.clone() + num2.clone()), (sym_x.clone() * 2 + num3.clone()));
    }

    #[test]
    fn test_sub() {
        let num0 = Expr(Rc::new(Expression::from(0)));
        let num1 = Expr(Rc::new(Expression::from(1)));
        let num2 = Expr(Rc::new(Expression::from(2)));
        let sym_x = Expr(Rc::new(Expression::<i32>::Symbol("x")));
        assert_eq!(-num1.clone(), Expr::from(-1));
        assert_eq!(num2.clone() - num1.clone(), num1.clone());
        assert_eq!(sym_x.clone() - sym_x.clone(), num0.clone());
    }
}