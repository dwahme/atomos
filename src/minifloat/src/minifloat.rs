use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{min, max};


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Minifloat {
    pub value: i8
}

impl Minifloat {
    pub fn new(f: f32) -> Minifloat {
        if f <= (-16 as f32) || f >= (16 as f32) {
            panic!("out of bounds value received for new");
        }

        Minifloat {
            value: (f * 8 as f32) as i8
        }
    }

    pub fn new_from_i8(i: i8) -> Minifloat {
        if (i < -128 || i > 127) {
            panic!("out of bounds value received for new");
        }

        Minifloat {
            value: i
        }
    }

    pub fn to_float(&self) -> f32 {
        self.value as f32 * 0.125
    }

    pub fn to_u8(&self) -> u8 {
        self.value as u8
    }
}

impl Add for Minifloat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { value: self.value.saturating_add(other.value) }
    }
}

impl Sub for Minifloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {value: self.value.saturating_sub(other.value) }
    }
}

impl Mul for Minifloat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {value: self.value.saturating_mul(other.value / 8) }
    }
}

impl Div for Minifloat {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        dbg!(self.value);
        dbg!(other.value);
        Self {value:  ((self.value as i64 * 8) / (other.value as i64)) as i8}
    }
}


#[cfg(test)]
mod tests {
    use crate::minifloat::Minifloat;
    
    #[test]
    fn minifloat_works() {
        let value = 3.25;
        let mf = Minifloat::new(value);
        assert_eq!(mf.to_float(), value);
    }

    #[test]
    fn adding() {
        let mf1 = Minifloat::new(3.5);
        let mf2 = Minifloat::new(1.125);
        let result = mf1 + mf2;
        assert_eq!(result, Minifloat::new(4.625))
    }

    #[test]
    fn subtracting() {
        let mf1 = Minifloat::new(3.5);
        let mf2 = Minifloat::new(1.125);
        let result = mf1 - mf2;
        assert_eq!(result, Minifloat::new(2.375))
    }

    #[test]
    fn dividing() {
        let mf1 = Minifloat::new(3.5);
        let mf2 = Minifloat::new(4 as f32);
        let result = mf1 / mf2;
        assert_eq!(result, Minifloat::new(0.875))
    }

    #[test]
    fn multiplying() {
        let mf1 = Minifloat::new(3.5);
        let mf2 = Minifloat::new(3 as f32);
        let result = mf1 * mf2;
        assert_eq!(result, Minifloat::new(10.5))
    }


    #[test]
    fn adding_overflow() {
        let mf1 = Minifloat::new(15 as f32);
        let mf2 = Minifloat::new(2 as f32);
        let result = mf1 + mf2;
        assert_eq!(result, Minifloat::new(15.875 as f32))
    }
}
