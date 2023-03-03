use std::{rc::Rc, collections::HashMap, hash::Hash, fmt::Debug};

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
    /// This struct contains both the function (`base`) and the arguments it calls on (`arg`).
    SingleArgFunc {
        base: &'static SingleArgFunc<T>,
        arg: Expr<T>,
    },
}

/// Wrapper class of a reference counting pointer to an expression.
/// 
/// When cloning an object of this type, the Expression it points to would not be cloned.
#[derive(Hash)]
pub struct Expr<T: Num + 'static>(pub(crate) Rc<Expression<T>>);

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
                (b1 == b2) && (a1.0 == a2.0),

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
                    k.0.hash(state);
                    v.hash(state);
                }
            },
            Self::SingleArgFunc { base, arg } => {
                base.hash(state);
                arg.0.hash(state);
            }
        }
    }
}

impl<T: Num + Debug + 'static> Debug for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(a) => a.fmt(f),
            Self::Symbol(x) => f.write_str(x),
            Self::Add { coeff, dict } => {
                f.write_str("(")?;
                coeff.fmt(f)?;
                for (term, c) in dict {
                    f.write_str(" + ")?;
                    if c == &T::zero() {
                        continue;
                    }
                    if c != &T::one() {
                        c.fmt(f)?;
                        f.write_str(" * ")?;
                    }
                    term.0.fmt(f)?;
                }
                f.write_str(")")
            },
            Self::Mul { coeff, dict } => {
                f.write_str("(")?;
                coeff.fmt(f)?;
                for (term, c) in dict {
                    f.write_str(" * ")?;
                    if c == &T::zero() {
                        continue;
                    }
                    term.0.fmt(f)?;
                    if c != &T::one() {
                        f.write_str(" ^ ")?;
                        c.fmt(f)?;
                    }
                }
                f.write_str(")")
            },
            Self::SingleArgFunc { base, arg } => {
                base.name.fmt(f)?;
                f.write_str("(")?;
                arg.0.fmt(f)?;
                f.write_str(")")
            }
        }
    }
}

impl<T: Num + Hash + 'static> PartialEq for Expr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Num + Hash + 'static> Eq for Expr<T> {}

impl<T: Num + 'static> Clone for Expr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Num + 'static> From<T> for Expr<T> {
    /// Create a new Expression::Number<T> object in the heap with given value and 
    /// returns the Expr<T> that points to the new object created.
    fn from(value: T) -> Self {
        Self(Expression::from(value).into())
    }
}

impl<T: Num + Debug + 'static> Debug for Expr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
/// Shorthand for defining an `Expr` that points to a symbol.
macro_rules! symbol {
    ($x:ident) => {
        Expr(Rc::new(Expression::Symbol(stringify!($x))))
    };
    ($x:ident, $t:ty) => {
        Expr(Rc::new(Expression::Symbol::<$t>(stringify!($x))))
    }
}

#[macro_export]
macro_rules! define_symbol {
    {} => {};
    {$x:ident  $($symbols:ident)*} => {
        let $x = symbol!($x);
        define_symbol!($($symbols)*)
    };
}

#[cfg(test)]
mod tests {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{:?}", Expression::from(1)), "1");
        assert_eq!(format!("{:?}", symbol!(x_0, i32)), "x_0");
        assert_eq!(format!("{:?}", Expr::from(2) + symbol!(x)), "(2 + x)");
    }

    #[test]
    fn test_hash() {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        Expr::from(2i32).hash(&mut hasher1);
        Expr(Expression::Symbol::<i32>("2").into()).hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}