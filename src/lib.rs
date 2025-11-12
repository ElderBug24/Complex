mod traits_ops;
mod traits_num_traits;

pub use std::fmt::{self, Debug, Display};
pub use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign};

pub use num_traits::{Bounded, AsPrimitive, FromPrimitive, NumCast, ToPrimitive, ConstOne, ConstZero, One, Zero, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedSub, Float, FloatConst, Inv, Pow};


#[derive(Clone, Copy, PartialEq)]
pub struct Complex<N: Float> {
    pub real: N,
    pub imaginary: N
}

impl<N: Float> Complex<N> {
    pub const fn new(real: N, imaginary: N) -> Self {
        return Self {
            real: real,
            imaginary: imaginary
        };
    }

    pub fn from_real(real: N) -> Self {
        return Self {
            real: real,
            imaginary: N::zero()
        };
    }

    pub fn zero() -> Self {
        return Self {
            real: N::zero(),
            imaginary: N::zero()
        };
    }

    pub fn one() -> Self {
        return Self {
            real: N::one(),
            imaginary: N::zero()
        };
    }

    pub fn i() -> Self {
        return Self {
            real: N::zero(),
            imaginary: N::one()
        };
    }

    pub fn from_argument_amplitude(argument: N, amplitude: N) -> Self {
        return Self {
            real: argument.cos() * amplitude,
            imaginary: argument.sin() * amplitude
        };
    }

    pub fn min_value() -> Self {
        return Self {
            real: N::min_value(),
            imaginary: N::min_value()
        };
    }

    pub fn max_value() -> Self {
        return Self {
            real: N::max_value(),
            imaginary: N::max_value()
        };
    }

    pub fn is_zero(&self) -> bool {
        return self.real.is_zero() && self.imaginary.is_zero();
    }

    pub fn is_one(&self) -> bool {
        return self.real.is_one() && self.imaginary.is_zero();
    }

    pub fn is_i(&self) -> bool {
        return self.real.is_zero() && self.imaginary.is_one();
    }

    pub fn set_zero(&mut self) {
        *self = Self::zero();
    }

    pub fn set_one(&mut self) {
        *self = Self::one();
    }

    pub fn set_i(&mut self) {
        *self = Self::i();
    }

    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        // assert!(bytes.len() == std::mem::size_of::<Self>());
        let ptr = bytes.as_ptr() as *const Self;

        return unsafe { ptr.read_unaligned() };
    }

    pub fn as_bytes(&self) -> &[u8] {
        return unsafe { std::slice::from_raw_parts((self as *const Self) as *const u8, std::mem::size_of::<Self>()) };
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        return unsafe { std::slice::from_raw_parts_mut((self as *mut Self) as *mut u8, std::mem::size_of::<Self>()) };
    }

    pub unsafe fn to_bytes<const LEN: usize>(&self) -> [u8; LEN] {
        // assert_eq!(LEN, std::mem::size_of::<Self>());
        let mut arr: [u8; LEN] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe { std::ptr::copy_nonoverlapping(self as *const Self as *const u8, arr.as_mut_ptr(), LEN) };

        return arr;
    }

    pub fn extract_real(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: N::zero()
        };
    }

    pub fn extract_imaginary(&self) -> Self {
        return Self {
            real: N::zero(),
            imaginary: self.imaginary
        };
    }

    pub fn amplitude(&self) -> N {
        return (self.real * self.real + self.imaginary * self.imaginary).sqrt();
    }

    pub fn argument(&self) -> N {
        return self.imaginary.atan2(self.real);
    }

    pub fn conj(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: -self.imaginary
        };
    }

    pub fn add(&self, other: &Self) -> Self {
        return Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        };
    }

    pub fn add_assign(&mut self, other: &Self) {
        *self = Self::add(self, other);
    }

    pub fn addf(&self, other: N) -> Self {
        return Self {
            real: self.real + other,
            imaginary: self.imaginary
        };
    }

    pub fn addf_assign(&mut self, other: N) {
        *self = Self::addf(self, other);
    }

    pub fn sub(&self, other: &Self) -> Self {
        return Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        };
    }

    pub fn sub_assign(&mut self, other: &Self) {
        *self = Self::sub(self, other);
    }

    pub fn subf(&self, other: N) -> Self {
        return Self {
            real: self.real - other,
            imaginary: self.imaginary
        };
    }

    pub fn subf_assign(&mut self, other: N) {
        *self = Self::subf(self, other);
    }

    pub fn mul(&self, other: &Self) -> Self {
        return Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real
        };
    }

    pub fn mul_assign(&mut self, other: &Self) {
        *self = Self::mul(self, other);
    }

    pub fn mulf(&self, other: N) -> Self {
        return Self {
            real: self.real * other,
            imaginary: self.imaginary * other
        };
    }

    pub fn mulf_assign(&mut self, other: N) {
        *self = Self::mulf(self, other);
    }

    pub fn div(&self, other: &Self) -> Self {
        let denominator = other.real * other.real + other.imaginary * other.imaginary;
        return Self {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denominator,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denominator
        };
    }

    pub fn div_assign(&mut self, other: &Self) {
        *self = Self::div(self, other);
    }

    pub fn divf(&self, other: N) -> Self {
        return Self {
            real: self.real / other,
            imaginary: self.imaginary / other
        };
    }

    pub fn divf_assign(&mut self, other: N) {
        *self = Self::divf(self, other);
    }

    pub fn neg(&self) -> Self {
        return Self {
            real: -self.real,
            imaginary: -self.imaginary
        };
    }

    pub fn recip(&self) -> Self {
        let divisor = self.real * self.real + self.imaginary * self.imaginary;

        return Self {
            real: self.real / divisor,
            imaginary: -self.imaginary / divisor
        };
    }

    pub fn inv(&self) -> Self {
        return Self::recip(self);
    }

    pub fn abs(&self) -> Self {
        return Self {
            real: self.real.abs(),
            imaginary: self.imaginary.abs()
        };
    }

    pub fn naive_round(&self) -> Self {
        return Self {
            real: self.real.round(),
            imaginary: self.imaginary.round()
        };
    }

    pub fn naive_floor(&self) -> Self {
        return Self {
            real: self.real.floor(),
            imaginary: self.imaginary.floor()
        };
    }

    pub fn naive_ceil(&self) -> Self {
        return Self {
            real: self.real.ceil(),
            imaginary: self.imaginary.ceil()
        };
    }

    pub fn naive_trunc(&self) -> Self {
        return Self {
            real: self.real.trunc(),
            imaginary: self.imaginary.trunc()
        };
    }

    pub fn signum(&self) -> Self {
        return Self {
            real: self.real.signum(),
            imaginary: self.imaginary.signum()
        };
    }

    pub fn ln(&self) -> Self {
        return Self {
            real: self.amplitude().ln(),
            imaginary: self.argument()
        };
    }

    pub fn exp(&self) -> Self {
        let exp_real = self.real.exp();
        return Self {
            real: exp_real * self.imaginary.cos(),
            imaginary: exp_real * self.imaginary.sin()
        };
    }

    pub fn log(&self, base: N) -> Self {
        return Self::div(&self.ln(), &Self::from_real(base).ln());
    }

    pub fn copysign(&self, sign: &Self) -> Self {
        return Self {
            real: self.real.copysign(sign.real),
            imaginary: self.imaginary.copysign(sign.imaginary)
        };
    }

    pub fn powi(&self,  exponent: isize) -> Self {
        let mut n = Self::one();
        let (mut base, mut exponent) = if exponent > 0 {
            (*self, exponent)
        } else {
            (self.recip(), -exponent)
        };

        while exponent > 0 {
            if exponent % 2 == 1 {
                n = n * base;
            }
            base *= base;

            exponent /= 2;
        }

        return n;
    }

    pub fn powf(&self, exponent: N) -> Self {
        return (self.ln() * Self::from_real(exponent)).exp();
    }

    pub fn pow(&self, other: &Self) -> Self {
        let r = self.amplitude();
        let arg = self.argument();

        return Complex::new(-other.imaginary * arg, other.imaginary * r.ln() + other.real * arg).exp().mulf(r.powf(other.real));
    }

    pub fn bitand(&self, other: &Self) -> Self {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = self as *const Self as *const u8;
            let other_bytes = other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) & *other_bytes.add(i);
            }

            return result;
        }
    }

    pub fn bitand_assign(&mut self, other: &Self) {
        *self = Self::bitand(self, other);
    }

    pub fn bitor(&self, other: &Self) -> Self {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = self as *const Self as *const u8;
            let other_bytes = other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) | *other_bytes.add(i);
            }

            return result;
        }
    }

    pub fn bitor_assign(&mut self, other: &Self) {
        *self = Self::bitand(self, other);
    }

    pub fn bitxor(&self, other: &Self) -> Self {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = self as *const Self as *const u8;
            let other_bytes = other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) ^ *other_bytes.add(i);
            }

            return result;
        }
    }

    pub fn bitxor_assign(&mut self, other: &Self) {
        *self = Self::bitxor(self, &other);
    }

    pub fn index(&self, index: bool) -> &N {
        return match index {
            false => &self.real,
            true => &self.imaginary
        };
    }

    pub fn index_mut(&mut self, index: bool) -> &mut N {
        return match index {
            false => &mut self.real,
            true => &mut self.imaginary
        };
    }

    pub fn is_pure_real(&self) -> bool {
        return self.imaginary.is_zero();
    }

    pub fn is_pure_imaginary(&self) -> bool {
        return self.real.is_zero();
    }

    pub fn is_finite(&self) -> (bool, bool) {
        return (self.real.is_finite(), self.imaginary.is_finite());
    }

    pub fn is_normal(&self) -> (bool, bool) {
        return (self.real.is_normal(), self.imaginary.is_normal());
    }

    pub fn is_subnormal(&self) -> (bool, bool) {
        return (self.real.is_subnormal(), self.imaginary.is_subnormal());
    }

    pub fn is_sign_positive(&self) -> (bool, bool) {
        return (self.real.is_sign_positive(), self.imaginary.is_sign_positive());
    }

    pub fn is_sign_negative(&self) -> (bool, bool) {
        return (self.real.is_sign_negative(), self.imaginary.is_sign_negative());
    }
}

impl<N: Float> Default for Complex<N> {
    fn default() -> Self {
        return Self::zero();
    }
}

impl<N: Float+Debug> Debug for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "Complex {{ real: {:?}, imaginary: {:?}i }}", self.real, self.imaginary);
    }
}

impl<N: Float+Display> Display for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let rsign = if self.real.is_sign_negative() { '-' } else { '+' };
        let isign = if self.imaginary.is_sign_negative() { '-' } else { '+' };

        return write!(formatter, "( {rsign} {} {isign} {}i )", self.real.abs(), self.imaginary.abs());
    }
}

