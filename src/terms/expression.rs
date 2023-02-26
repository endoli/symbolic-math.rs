use std::rc::Rc;

use num::Num;

/// Trait for every symbolic expression.
pub trait Expression<T: Num> {
}

/// Wrapper class for reference counting pointer to an expression.
pub type Expr<T> = Rc<dyn Expression<T>>;

// TODO (Add `Hash` and `Eq` trait to `Expr<T>`)