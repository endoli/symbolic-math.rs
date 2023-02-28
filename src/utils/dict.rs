use std::collections::HashMap;

use num::Num;

/// Remove all entries whose corresponding value is zero.
pub fn remove_zeros<U, T: Num>(dict: &mut HashMap<U, T>) {
    dict.retain(|_, v| *v != T::zero());
}