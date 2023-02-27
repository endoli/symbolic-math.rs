use std::{rc::Rc, collections::HashMap, hash::Hash};

use num::Num;

use super::func::SingleArgFunc;

/// Enum for every symbolic expression.
pub enum Expression<T: Num + 'static> {
    /// Represents a constant number.
    Number(T),

    /// Represents a named symbol.
    Symbol(&'static str),

    /// Represents the operation `coeff + c0 * x0 + c1 * x1 + c2 * x2 + ...`,
    /// where `coeff`, `c0`, `c1`, `c2`, ... are number constants.
    Add {
        coeff: T,
        dict: HashMap<Expr<T>, T>,
    },

    /// Represents the operation `coeff * x0 ^ c0 * x1 ^ c1 * x2 ^ c2 * ...`,
    /// where `coeff`, `c0`, `c1`, `c2`, ... are number constants.
    Mul {
        coeff: T,
        dict: HashMap<Expr<T>, T>,
    },

    /// Represents a function call to a single argument function.
    /// 
    /// This struct contains both the function and the arguments it calls on.
    SingleArgFunc {
        base: &'static SingleArgFunc<T>,
        arg: Expr<T>,
    },
}

/// Wrapper class for reference counting pointer to an expression.
pub type Expr<T> = Rc<Expression<T>>;

impl<T: Num + 'static> From<T> for Expression<T> {
    /// Convert a number to Self::Number object
    fn from(value: T) -> Self {
        Self::Number(value)
    }
}

impl<T: Num + Hash + 'static> PartialEq for Expression<T> {
    /// Returns true if the expression tree of this expression is same as another expression.
    /// 
    /// This function only considers the tree structure. It would not perform any
    /// rewrite or simplification to expression trees.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,

            (Self::Symbol(x), Self::Symbol(y)) => x == y,

            (Self::Add { coeff: c1, dict: d1 }, Self::Add { coeff: c2, dict: d2 }) =>
                (c1 == c2) && (d1.len() == d2.len()) && d1.iter().map(|(k, v)| d2.get(k) == Some(v)).all(|b| b),

            (Self::Mul { coeff: c1, dict: d1 }, Self::Mul { coeff: c2, dict: d2 }) =>
                (c1 == c2) && (d1.len() == d2.len()) && d1.iter().map(|(k, v)| d2.get(k) == Some(v)).all(|b| b),

            (Self::SingleArgFunc { base: b1, arg: a1 }, Self::SingleArgFunc { base: b2, arg: a2 }) =>
                (b1 == b2) && (a1 == a2),

            (_, _) => false,
        }
    }
}

impl<T: Num + Hash + 'static> Eq for Expression<T> {}

impl<T: Num + Hash + 'static> Hash for Expression<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Number(a) => a.hash(state),
            Self::Symbol(x) => x.hash(state),
            Self::Add { coeff, dict } | Self::Mul { coeff, dict } => {
                coeff.hash(state);
                for (k, v) in dict {
                    k.hash(state);
                    v.hash(state);
                }
            },
            Self::SingleArgFunc { base, arg } => {
                base.hash(state);
                arg.hash(state);
            }
        }
    }
}

// TODO (Add arithmetic operation traits `Add`, `Sub`, `Mul`, `Div`, etc. to `Expression<T>`)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}