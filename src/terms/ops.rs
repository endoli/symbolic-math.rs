use std::collections::HashMap;

use num::Num;

use super::expression::{Expr, Expression};

/// Represents the operation `coeff + c0 * x0 + c1 * x1 + c2 * x2 + ...`,
/// where `coeff`, `c0`, `c1`, `c2`, ... are number constants.
pub struct Add<T: Num> {
    /// Number coefficient of the expression.
    coeff: T,
    /// Dictionary that maps terms (x0, x1, x2, ...) to their coefficients (c0, c1, c2, ...)
    dict: HashMap<Expr<T>, T>,
}

impl<T: Num> Expression<T> for Add<T> {
    // TODO
}

/// Represents the operation `coeff * x0 ^ c0 * x1 ^ c1 * x2 ^ c2 * ...`,
/// where `coeff`, `c0`, `c1`, `c2`, ... are number constants.
pub struct Mul<T: Num> {
    /// Number coefficient of the expression.
    coeff: T,
    /// Dictionary that maps terms (x0, x1, x2, ...) to their coefficients (c0, c1, c2, ...)
    dict: HashMap<Expr<T>, T>,
}

impl<T: Num> Expression<T> for Mul<T> {
    // TODO
}