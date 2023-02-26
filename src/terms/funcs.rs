use num::Num;

use super::expression::{Expr, Expression};

/// Represents a function with single input argument.
pub struct SingleArgFunc<T: Num> {
    name: &'static str,
    arg: Expr<T>,
}

impl<T: Num> Expression<T> for SingleArgFunc<T> {
    // TODO
}