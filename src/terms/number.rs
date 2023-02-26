use std::hash::Hash;

use num::Num;

use super::expression::Expression;

/// Implement the Expression trait for all numbers
impl<T: Num + Hash> Expression<T> for T {
    // TODO
}