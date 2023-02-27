use std::hash::Hash;

use num::Num;

/// Defines a function that can accept a single argument.
/// 
/// This struct only contains information about the function. It
/// does not include the specific argument this function calls on.
pub struct SingleArgFunc<T: Num + 'static> {
    pub name: &'static str,
    pub call: &'static (dyn Fn(T) -> T + Sync),
    // TODO
}

impl<T: Num + 'static> PartialEq for SingleArgFunc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T: Num + 'static> Eq for SingleArgFunc<T> {}

impl<T: Num + 'static + Hash> Hash for SingleArgFunc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub mod collections {
    use super::*;

    pub static EXP_F32: SingleArgFunc<f32> = SingleArgFunc {
        name: "exp",
        call: &|x| f32::exp(x),
    };
    pub static EXP_F64: SingleArgFunc<f64> = SingleArgFunc {
        name: "exp",
        call: &|x| f64::exp(x),
    };
    pub static LN_F32: SingleArgFunc<f32> = SingleArgFunc {
        name: "ln",
        call: &|x| f32::ln(x),
    };
    pub static LN_F64: SingleArgFunc<f64> = SingleArgFunc {
        name: "ln",
        call: &|x| f64::ln(x),
    };
}