#[allow(dead_code)]

pub trait Ternary {
    fn ternary<T>(&self, opt_1: T, opt_2: T) -> T;
}

impl Ternary for bool {
    fn ternary<T>(&self, opt_1: T, opt_2: T) -> T {
       if *self {
           return opt_1;
       }
       return opt_2;
    }
}

pub fn ternary<T>(condition: bool, opt_1: T, opt_2: T) -> T {
    if condition {
        return opt_1;
    }
    return opt_2;
}