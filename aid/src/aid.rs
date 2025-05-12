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

pub trait BoolTo {
    fn i8(self) -> i8;
    fn i16(self) -> i16;
    fn i32(self) -> i32;
    fn i64(self) -> i64;
    fn i128(self) -> i128;
    fn isize(self) -> isize;

    fn u8(self) -> u8;
    fn u16(self) -> u16;
    fn u32(self) -> u32;
    fn u64(self) -> u64;
    fn u128(self) -> u128;
    fn usize(self) -> usize;

    fn f32(self) -> f32;
    fn f64(self) -> f64;
}

impl BoolTo for bool {
    fn i8(self) -> i8 {
        self.ternary(1, 0)
    }

    fn i16(self) -> i16 {
        self.ternary(1, 0)
    }

    fn i32(self) -> i32 {
        self.ternary(1, 0)
    }

    fn i64(self) -> i64 {
        self.ternary(1, 0)
    }

    fn i128(self) -> i128 {
        self.ternary(1, 0)
    }

    fn isize(self) -> isize {
        self.ternary(1, 0)
    }

    fn u8(self) -> u8 {
        self.ternary(1, 0)
    }

    fn u16(self) -> u16 {
        self.ternary(1, 0)
    }

    fn u32(self) -> u32 {
        self.ternary(1, 0)
    }

    fn u64(self) -> u64 {
        self.ternary(1, 0)
    }

    fn u128(self) -> u128 {
        self.ternary(1, 0)
    }

    fn usize(self) -> usize {
        self.ternary(1, 0)
    }

    fn f32(self) -> f32 {
        self.ternary(1.0, 0.0)
    }

    fn f64(self) -> f64 {
        self.ternary(1.0, 0.0)
    }
}