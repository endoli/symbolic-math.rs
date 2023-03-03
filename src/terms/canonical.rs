use std::{hash::Hash, collections::HashMap};

use num::{Num, traits::Pow};

use crate::expression::{Expr, Expression};

impl<T: Num + Hash + Copy + Pow<T, Output = T> + 'static> Expr<T> {
    /// Rewrite the given expression to the canonical form.
    /// 
    /// Canonical form is defined as follows:
    /// 
    /// 1. No nested `Add` or `Mul` nodes. If one `Add` node has a child `Add` node, merge
    ///    them together. Same for nested `Mul` nodes.
    /// 2. No `Add` or `Mul` nodes with empty sub-node dictionary. If one `Add` or `Mul` node
    ///    has no sub-node, reduce it to a `Number` node with its original coefficient value.
    /// 3. `Add` and `Mul`'s sub-nodes must not be `Number`. If an `Add` or `Mul` node has a
    ///    sub-node with type `Number`, merge it with the main coefficient.
    /// 4. `Mul`'s main coefficient should not be number `0`. If the coefficient is `0`,
    ///    replace the `Mul` with the number `0`.
    /// 5. No nodes with form `0 + x` or `1 * x`. They should be replaced with the sub-node
    ///    `x`.
    pub fn canonicalize(self) -> Self {
        match self.0.as_ref() {
            Expression::Number(_) | Expression::Symbol(_) | Expression::SingleArgFunc { base: _, arg: _ } => self,
            
            Expression::Add { coeff, dict } => {
                if dict.len() == 0 {
                    // check for rule 2
                    return Expr::from(*coeff);
                }
                if dict.len() == 1 && *coeff == T::zero() {
                    if let Some((k, v)) = dict.iter().next() {
                        if *v == T::one() {
                            // check for rule 5
                            return k.clone();
                        }
                    }
                }
                // canonicify every subnodes
                let mut is_canonical = true;   // if the current `Add` node is already canonical
                let mut new_dict: HashMap<Expr<T>, T> = HashMap::new();
                let mut new_coeff = *coeff;
                for (subnode, subnode_coeff) in dict {
                    let new_subnode = subnode.clone().canonicalize();
                    if new_subnode != *subnode {
                        is_canonical = false;
                    }
                    if let Expression::Number(number) = new_subnode.0.as_ref() {
                        // check for rule 3
                        new_coeff = new_coeff + (*number) * (*subnode_coeff);
                        is_canonical = false;
                    } else if let Expression::Add { coeff: sub_coeff, dict: sub_dict } = new_subnode.0.as_ref() {
                        // check for rule 1
                        new_coeff = new_coeff + (*sub_coeff) * (*subnode_coeff);
                        for (sk, sv) in sub_dict {
                            if let Some(val) = new_dict.get_mut(sk) {
                                *val = *val + (*sv) * (*subnode_coeff);
                            } else {
                                new_dict.insert(sk.clone(), (*sv) * (*subnode_coeff));
                            }
                        }
                        is_canonical = false;
                    } else {
                        new_dict.insert(new_subnode, *subnode_coeff);
                    }
                }
                if is_canonical {
                    self
                } else {
                    Expr(Expression::Add { coeff: new_coeff, dict: new_dict }.into())
                }
            },
            
            Expression::Mul { coeff, dict } => {
                if dict.len() == 0 {
                    // check for rule 2
                    return Expr::from(*coeff);
                }
                if coeff.is_zero() {
                    // check for rule 4
                    return Expr::from(T::zero());
                }
                if dict.len() == 1 && *coeff == T::one() {
                    if let Some((k, v)) = dict.iter().next() {
                        if *v == T::one() {
                            // check for rule 5
                            return k.clone();
                        }
                    }
                }
                // canonicify every subnodes
                let mut is_canonical = true;   // if the current `Add` node is already canonical
                let mut new_dict: HashMap<Expr<T>, T> = HashMap::new();
                let mut new_coeff = *coeff;
                for (subnode, subnode_coeff) in dict {
                    let new_subnode = subnode.clone().canonicalize();
                    if new_subnode != *subnode {
                        is_canonical = false;
                    }
                    if let Expression::Number(number) = new_subnode.0.as_ref() {
                        // check for rule 3
                        new_coeff = new_coeff * (*number).pow(*subnode_coeff);
                        is_canonical = false;
                    } else if let Expression::Mul { coeff: sub_coeff, dict: sub_dict } = new_subnode.0.as_ref() {
                        // check for rule 1
                        new_coeff = new_coeff * (*sub_coeff).pow(*subnode_coeff);
                        for (sk, sv) in sub_dict {
                            if let Some(val) = new_dict.get_mut(sk) {
                                *val = *val + (*sv) * (*subnode_coeff);
                            } else {
                                new_dict.insert(sk.clone(), (*sv) * (*subnode_coeff));
                            }
                        }
                        is_canonical = false;
                    } else {
                        new_dict.insert(new_subnode, *subnode_coeff);
                    }
                }
                if is_canonical {
                    self
                } else {
                    Expr(Expression::Mul { coeff: new_coeff, dict: new_dict }.into())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, define_symbol};
    use std::rc::Rc;

    #[test]
    fn test_canonicify() {
        // test rule 1
        define_symbol!(x y);
        assert_eq!((Expr::from(1) + x.clone()) + Expr::from(2), Expr::from(3) + x.clone());
        assert_eq!((Expr::from(1) + x.clone()) + y.clone(), Expr::from(1) + (x.clone() + y.clone()));
    }
}