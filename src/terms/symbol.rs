use std::hash::Hash;

use num::Num;

use super::expression::Expression;

pub struct Symbol {
    name: &'static str,
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<T: Num> Expression<T> for Symbol {
    // TODO
}