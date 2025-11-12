use super::Complex;

use num_traits::{Bounded, AsPrimitive, FromPrimitive, NumCast, ToPrimitive, ConstOne, ConstZero, One, Zero, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedSub, Float, FloatConst, Inv, Pow};


impl<N: Float> Inv for Complex<N> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        return Self::recip(&self);
    }
}

impl<N: Float> Pow<N> for Complex<N> {
    type Output = Self;

    fn pow(self, other: N) -> Self::Output {
        return Self::powf(&self, other);
    }
}

impl<N: Float> Pow<Self> for Complex<N> {
    type Output = Self;

    fn pow(self, other: Self) -> Self::Output {
        return Self::pow(&self, &other);
    }
}

impl<N: Float+Bounded> Bounded for Complex<N> {
    fn min_value() -> Self {
        return Self::min_value();
    }

    fn max_value() -> Self {
        return Self::max_value();
    }
}

impl<N: Float> Zero for Complex<N> {
    fn zero() -> Self {
        return Self::zero();
    }

    fn is_zero(&self) -> bool {
        return Self::is_zero(self);
    }

    fn set_zero(&mut self) {
        Self::set_zero(self);
    }
}

impl<N: Float+ConstZero> ConstZero for Complex<N> {
    const ZERO: Self = Self {
        real: N::ZERO,
        imaginary: N::ZERO
    };
}

impl<N: Float> One for Complex<N> {
    fn one() -> Self {
        return Self::one();
    }

    fn is_one(&self) -> bool {
        return Self::is_one(self);
    }

    fn set_one(&mut self) {
        Self::set_one(self);
    }
}

impl<N: Float+ConstZero+ConstOne> ConstOne for Complex<N> {
    const ONE: Self = Self {
        real: N::ONE,
        imaginary: N::ZERO
    };
}

impl<N: Float+ConstZero+FloatConst> FloatConst for Complex<N> {
    fn E() -> Self {
        return Self {
            real: N::E(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_1_PI() -> Self {
        return Self {
            real: N::FRAC_1_PI(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_1_SQRT_2() -> Self {
        return Self {
            real: N::FRAC_1_SQRT_2(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_2_PI() -> Self {
        return Self {
            real: N::FRAC_2_PI(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_2_SQRT_PI() -> Self {
        return Self {
            real: N::FRAC_2_SQRT_PI(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_PI_2() -> Self {
        return Self {
            real: N::FRAC_PI_2(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_PI_3() -> Self {
        return Self {
            real: N::FRAC_PI_3(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_PI_4() -> Self {
        return Self {
            real: N::FRAC_PI_4(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_PI_6() -> Self {
        return Self {
            real: N::FRAC_PI_6(),
            imaginary: N::ZERO
        };
    }

    fn FRAC_PI_8() -> Self {
        return Self {
            real: N::FRAC_PI_8(),
            imaginary: N::ZERO
        };
    }

    fn LN_10() -> Self {
        return Self {
            real: N::LN_10(),
            imaginary: N::ZERO
        };
    }

    fn LN_2() -> Self {
        return Self {
            real: N::LN_2(),
            imaginary: N::ZERO
        };
    }

    fn LOG10_E() -> Self {
        return Self {
            real: N::LOG10_E(),
            imaginary: N::ZERO
        };
    }

    fn LOG2_E() -> Self {
        return Self {
            real: N::LOG2_E(),
            imaginary: N::ZERO
        };
    }

    fn PI() -> Self {
        return Self {
            real: N::PI(),
            imaginary: N::ZERO
        };
    }

    fn SQRT_2() -> Self {
        return Self {
            real: N::SQRT_2(),
            imaginary: N::ZERO
        };
    }

    fn TAU() -> Self {
        return Self {
            real: N::TAU(),
            imaginary: N::ZERO
        };
    }

    fn LOG10_2() -> Self {
        return Self {
            real: N::LOG10_2(),
            imaginary: N::ZERO
        };
    }

    fn LOG2_10() -> Self {
        return Self {
            real: N::LOG2_10(),
            imaginary: N::ZERO
        };
    }
}

impl<N: Float> From<(N, N)> for Complex<N> {
    fn from((real, imaginary): (N, N)) -> Self {
        return Self::new(real, imaginary);
    }
}

impl<N: Float> From<N> for Complex<N> {
    fn from(real: N) -> Self {
        return Self::from_real(real);
    }
}

impl<N: Float> Into<(N, N)> for Complex<N> {
    fn into(self) -> (N, N) {
        return (self.real, self.imaginary);
    }
}

impl<N: Float+AsPrimitive<i64>> AsPrimitive<i64> for Complex<N> {
    fn as_(self) -> i64 { self.real.as_() }
}

impl<N: Float+AsPrimitive<u64>> AsPrimitive<u64> for Complex<N> {
    fn as_(self) -> u64 { self.real.as_() }
}

impl<N: Float+AsPrimitive<isize>> AsPrimitive<isize> for Complex<N> {
    fn as_(self) -> isize { self.real.as_() }
}

impl<N: Float+AsPrimitive<i8>> AsPrimitive<i8> for Complex<N> {
    fn as_(self) -> i8 { self.real.as_() }
}

impl<N: Float+AsPrimitive<i16>> AsPrimitive<i16> for Complex<N> {
    fn as_(self) -> i16 { self.real.as_() }
}

impl<N: Float+AsPrimitive<i32>> AsPrimitive<i32> for Complex<N> {
    fn as_(self) -> i32 { self.real.as_() }
}

impl<N: Float+AsPrimitive<i128>> AsPrimitive<i128> for Complex<N> {
    fn as_(self) -> i128 { self.real.as_() }
}

impl<N: Float+AsPrimitive<usize>> AsPrimitive<usize> for Complex<N> {
    fn as_(self) -> usize { self.real.as_() }
}

impl<N: Float+AsPrimitive<u8>> AsPrimitive<u8> for Complex<N> {
    fn as_(self) -> u8 { self.real.as_() }
}

impl<N: Float+AsPrimitive<u16>> AsPrimitive<u16> for Complex<N> {
    fn as_(self) -> u16 { self.real.as_() }
}

impl<N: Float+AsPrimitive<u32>> AsPrimitive<u32> for Complex<N> {
    fn as_(self) -> u32 { self.real.as_() }
}

impl<N: Float+AsPrimitive<u128>> AsPrimitive<u128> for Complex<N> {
    fn as_(self) -> u128 { self.real.as_() }
}

impl<N: Float+AsPrimitive<f32>> AsPrimitive<f32> for Complex<N> {
    fn as_(self) -> f32 { self.real.as_() }
}

impl<N: Float+AsPrimitive<f64>> AsPrimitive<f64> for Complex<N> {
    fn as_(self) -> f64 { self.real.as_() }
}

impl<N: Float+FromPrimitive> FromPrimitive for Complex<N> {
    fn from_i64(n: i64) -> Option<Self> {
        return Some(Self {
            real: N::from_i64(n)?,
            imaginary: N::zero()
        });
    }

    fn from_u64(n: u64) -> Option<Self> {
        return Some(Self {
            real: N::from_u64(n)?,
            imaginary: N::zero()
        });
    }

    fn from_isize(n: isize) -> Option<Self> {
        return Some(Self {
            real: N::from_isize(n)?,
            imaginary: N::zero()
        });
    }

    fn from_i8(n: i8) -> Option<Self> {
        return Some(Self {
            real: N::from_i8(n)?,
            imaginary: N::zero()
        });
    }

    fn from_i16(n: i16) -> Option<Self> {
        return Some(Self {
            real: N::from_i16(n)?,
            imaginary: N::zero()
        });
    }

    fn from_i32(n: i32) -> Option<Self> {
        return Some(Self {
            real: N::from_i32(n)?,
            imaginary: N::zero()
        });
    }

    fn from_i128(n: i128) -> Option<Self> {
        return Some(Self {
            real: N::from_i128(n)?,
            imaginary: N::zero()
        });
    }

    fn from_usize(n: usize) -> Option<Self> {
        return Some(Self {
            real: N::from_usize(n)?,
            imaginary: N::zero()
        });
    }

    fn from_u8(n: u8) -> Option<Self> {
        return Some(Self {
            real: N::from_u8(n)?,
            imaginary: N::zero()
        });
    }

    fn from_u16(n: u16) -> Option<Self> {
        return Some(Self {
            real: N::from_u16(n)?,
            imaginary: N::zero()
        });
    }

    fn from_u32(n: u32) -> Option<Self> {
        return Some(Self {
            real: N::from_u32(n)?,
            imaginary: N::zero()
        });
    }

    fn from_u128(n: u128) -> Option<Self> {
        return Some(Self {
            real: N::from_u128(n)?,
            imaginary: N::zero()
        });
    }

    fn from_f32(n: f32) -> Option<Self> {
        return Some(Self {
            real: N::from_f32(n)?,
            imaginary: N::zero()
        });
    }

    fn from_f64(n: f64) -> Option<Self> {
        return Some(Self {
            real: N::from_f64(n)?,
            imaginary: N::zero()
        });
    }
}

impl<N: Float+NumCast> NumCast for Complex<N> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        return Some(Self {
            real: N::from(n)?,
            imaginary: N::zero()
        });
    }
}

impl<N: Float+ToPrimitive> ToPrimitive for Complex<N> {
    fn to_i64(&self) -> Option<i64> { self.real.to_i64() }

    fn to_u64(&self) -> Option<u64> { self.real.to_u64() }

    fn to_isize(&self) -> Option<isize> { self.real.to_isize() }

    fn to_i8(&self) -> Option<i8> { self.real.to_i8() }

    fn to_i16(&self) -> Option<i16> { self.real.to_i16() }

    fn to_i32(&self) -> Option<i32> { self.real.to_i32() }

    fn to_i128(&self) -> Option<i128> { self.real.to_i128() }

    fn to_usize(&self) -> Option<usize> { self.real.to_usize() }

    fn to_u8(&self) -> Option<u8> { self.real.to_u8() }

    fn to_u16(&self) -> Option<u16> { self.real.to_u16() }

    fn to_u32(&self) -> Option<u32> { self.real.to_u32() }

    fn to_u128(&self) -> Option<u128> { self.real.to_u128() }

    fn to_f32(&self) -> Option<f32> { self.real.to_f32() }

    fn to_f64(&self) -> Option<f64> { self.real.to_f64() }
}

impl<N: Float+CheckedAdd> CheckedAdd for Complex<N> {
    fn checked_add(&self, other: &Self) -> Option<Self> {
        return Some(Self {
            real: self.real.checked_add(&other.real)?,
            imaginary: self.imaginary.checked_add(&other.imaginary)?
        });
    }
}

impl<N: Float+CheckedSub> CheckedSub for Complex<N> {
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        return Some(Self {
            real: self.real.checked_sub(&other.real)?,
            imaginary: self.imaginary.checked_sub(&other.imaginary)?
        });
    }
}

impl<N: Float+CheckedAdd+CheckedSub+CheckedMul> CheckedMul for Complex<N> {
    fn checked_mul(&self, other: &Self) -> Option<Self> {
        return Some(Self {
            real: self.real.checked_mul(&other.real)?.checked_sub(&self.imaginary.checked_mul(&other.imaginary)?)?,
            imaginary: self.real.checked_mul(&other.imaginary)?.checked_add(&self.imaginary.checked_mul(&other.real)?)?
        });
    }
}

impl<N: Float+CheckedAdd+CheckedSub+CheckedMul+CheckedDiv> CheckedDiv for Complex<N> {
    fn checked_div(&self, other: &Self) -> Option<Self> {
        let denominator = other.real.checked_mul(&other.real)?.checked_add(&other.imaginary.checked_mul(&other.imaginary)?)?;

        return Some(Self {
            real: self.real.checked_mul(&other.real)?.checked_add(&self.imaginary.checked_mul(&other.imaginary)?)?.checked_div(&denominator)?,
            imaginary: self.imaginary.checked_mul(&other.real)?.checked_sub(&self.real.checked_mul(&other.imaginary)?)?.checked_div(&denominator)?
        });
    }
}

impl<N: Float+CheckedNeg> CheckedNeg for Complex<N> {
    fn checked_neg(&self) -> Option<Self> {
        return Some(Self {
            real: self.real.checked_neg()?,
            imaginary: self.imaginary.checked_neg()?
        });
    }
}
