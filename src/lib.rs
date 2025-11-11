pub use std::fmt::{self, Debug, Display};
pub use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

pub use num_traits::{Bounded, AsPrimitive, FromPrimitive, NumCast, ToPrimitive, ConstOne, ConstZero, One, Zero, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedSub, Float, FloatConst, Inv, Pow};


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
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

    pub fn from_argument_amplitude(argument: N, amplitude: N) -> Self {
        return Self {
            real: argument.cos() * amplitude,
            imaginary: argument.sin() * amplitude
        };
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

    pub fn is_pure_real(&self) -> bool {
        return self.imaginary.is_zero();
    }

    pub fn is_pure_imaginary(&self) -> bool {
        return self.real.is_zero();
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

    pub fn conj(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: -self.imaginary
        };
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
        let m = r.powf(other.real) * (-other.imaginary * arg).exp();
        let theta = other.imaginary * r.ln() + other.real * arg;

        return Self::from_argument_amplitude(theta, m);
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

    // pub fn shl(&self, rhs: usize) -> Self {
    //     let size = std::mem::size_of::<Self>();
    //     let mut bytes = vec![0u8; size];
    //
    //     unsafe {
    //         let self_ptr = self as *const Self as *const u8;
    //         for i in 0..size {
    //             bytes[i] = *self_ptr.add(i);
    //         }
    //
    //         let byte_shift = rhs / 8;
    //         let bit_shift = rhs % 8;
    //
    //         if byte_shift > 0 {
    //             for i in 0..size {
    //                 bytes[i] = if i + byte_shift < size { bytes[i + byte_shift] } else { 0 };
    //             }
    //         }
    //
    //         if bit_shift > 0 {
    //             let mut carry = 0u8;
    //             for i in (0..size).rev() {
    //                 let new_carry = bytes[i] >> (8 - bit_shift);
    //                 bytes[i] = (bytes[i] << bit_shift) | carry;
    //                 carry = new_carry;
    //             }
    //         }
    //
    //         let mut result: Self = std::mem::zeroed();
    //         let r_ptr = &mut result as *mut Self as *mut u8;
    //         for i in 0..size {
    //             *r_ptr.add(i) = bytes[i];
    //         }
    //
    //         return result;
    //     }
    // }
    //
    // pub fn shl_assign(&mut self, other: usize) {
    //     *self = Self::shl(self, other);
    // }
    //
    // pub fn checked_shl(&self, rhs: usize) -> Option<Self> {
    //     if rhs < 2 * std::mem::size_of::<Self>() {
    //         return Some(Self::shl(self, rhs));
    //     } else {
    //         return None;
    //     }
    // }
    //
    // pub fn shr(&self, rhs: usize) -> Self {
    //     let size = std::mem::size_of::<Self>();
    //     let mut bytes = vec![0u8; size];
    //
    //     unsafe {
    //         let self_ptr = self as *const Self as *const u8;
    //         for i in 0..size {
    //             bytes[i] = *self_ptr.add(i);
    //         }
    //
    //         let byte_shift = rhs / 8;
    //         let bit_shift = rhs % 8;
    //
    //         if byte_shift > 0 {
    //             for i in (0..size).rev() {
    //                 bytes[i] = if i >= byte_shift { bytes[i - byte_shift] } else { 0 };
    //             }
    //         }
    //
    //         if bit_shift > 0 {
    //             let mut carry = 0u8;
    //             for i in 0..size {
    //                 let new_carry = bytes[i] << (8 - bit_shift);
    //                 bytes[i] = (bytes[i] >> bit_shift) | carry;
    //                 carry = new_carry;
    //             }
    //         }
    //
    //         let mut result: Self = std::mem::zeroed();
    //         let r_ptr = &mut result as *mut Self as *mut u8;
    //         for i in 0..size {
    //             *r_ptr.add(i) = bytes[i];
    //         }
    //
    //         return result;
    //     }
    // }
    //
    // pub fn shr_assign(&mut self, rhs: usize) {
    //     *self = Self::shr(self, rhs);
    // }
    //
    // pub fn checked_shr(&self, rhs: usize) -> Option<Self> {
    //     if rhs < 2 * std::mem::size_of::<Self>() {
    //         return Some(Self::shr(self, rhs));
    //     } else {
    //         return None;
    //     }
    // }

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
}

impl<N: Float> Default for Complex<N> {
    fn default() -> Self {
        return Self::zero();
    }
}

impl<N: Float> Add<N> for Complex<N> {
    type Output = Self;

    fn add(self, other: N) -> Self::Output {
        return Self::addf(&self, other);
    }
}

impl<N: Float> Add for Complex<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self::add(&self, &other);
    }
}

impl<N: Float> AddAssign<N> for Complex<N> {
    fn add_assign(&mut self, other: N) {
        Self::addf_assign(self, other);
    }
}

impl<N: Float> AddAssign for Complex<N> {
    fn add_assign(&mut self, other: Self) {
        Self::add_assign(self, &other);
    }
}

impl<N: Float> BitAnd for Complex<N> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        return Self::bitand(&self, &other);
    }
}

impl<N: Float> BitAndAssign for Complex<N> {
    fn bitand_assign(&mut self, other: Self) {
        Self::bitand_assign(self, &other);
    }
}

impl<N: Float> BitOr for Complex<N> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        return Self::bitor(&self, &other);
    }
}

impl<N: Float> BitOrAssign for Complex<N> {
    fn bitor_assign(&mut self, other: Self) {
        Self::bitor_assign(self, &other);
    }
}

impl<N: Float> BitXor for Complex<N> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        return Self::bitxor(&self, &other);
    }
}

impl<N: Float> BitXorAssign for Complex<N> {
    fn bitxor_assign(&mut self, other: Self) {
        Self::bitxor_assign(self, &other);
    }
}

impl<N: Float> Deref for Complex<N> {
    type Target = (N, N);

    fn deref(&self) -> &Self::Target {
        let ptr = &(&self.real, &self.imaginary) as *const (&N, &N) as *const &(N, N);
        let ptr: &(N, N) = unsafe { &*ptr };

        return ptr;
    }
}

impl<N: Float> DerefMut for Complex<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = &mut (&mut self.real, &mut self.imaginary) as *mut (&mut N, &mut N) as *mut &mut (N, N);
        let ptr: &mut (N, N) = unsafe { &mut *ptr };

        return ptr;
    }
}

impl<N: Float> Div<N> for Complex<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        return Self::divf(&self, other);
    }
}

impl<N: Float> Div for Complex<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        return Self::div(&self, &other);
    }
}

impl<N: Float> DivAssign<N> for Complex<N> {
    fn div_assign(&mut self, other: N) {
        Self::divf_assign(self, other);
    }
}

impl<N: Float> DivAssign for Complex<N> {
    fn div_assign(&mut self, other: Self) {
        Self::div_assign(self, &other);
    }
}

impl<N: Float> Index<bool> for Complex<N> {
    type Output = N;

    fn index(&self, index: bool) -> &Self::Output {
        return Self::index(self, index);
    }
}

impl<N: Float> IndexMut<bool> for Complex<N> {
    fn index_mut(&mut self, index: bool) -> &mut Self::Output {
        return Self::index_mut(self, index);
    }
}

impl<N: Float> Mul<N> for Complex<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        return Self::mulf(&self, other);
    }
}

impl<N: Float> Mul for Complex<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Self::mul(&self, &other);
    }
}

impl<N: Float> MulAssign<N> for Complex<N> {
    fn mul_assign(&mut self, other: N) {
        Self::mulf_assign(self, other);
    }
}

impl<N: Float> MulAssign for Complex<N> {
    fn mul_assign(&mut self, other: Self) {
        Self::mul_assign(self, &other);
    }
}

impl<N: Float> Neg for Complex<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self::neg(&self);
    }
}

impl<N: Float> Not for Complex<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self::conj(&self);
    }
}

// impl<N: Float> Shl<usize> for Complex<N> {
//     type Output = Self;
//
//     fn shl(self, rhs: usize) -> Self::Output {
//         return Self::shl(&self, rhs);
//     }
// }
//
// impl<N: Float> ShlAssign<usize> for Complex<N> {
//     fn shl_assign(&mut self, rhs: usize) {
//         Self::shl_assign(self, rhs);
//     }
// }
//
// impl<N: Float> Shr<usize> for Complex<N> {
//     type Output = Self;
//
//     fn shr(self, rhs: usize) -> Self::Output {
//         return Self::shr(&self, rhs);
//     }
// }
//
// impl<N: Float> ShrAssign<usize> for Complex<N> {
//     fn shr_assign(&mut self, rhs: usize) {
//         Self::shr_assign(self, rhs);
//     }
// }

impl<N: Float> Sub<N> for Complex<N> {
    type Output = Self;

    fn sub(self, other: N) -> Self::Output {
        return Self::subf(&self, other);
    }
}

impl<N: Float> Sub for Complex<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self::sub(&self, &other);
    }
}

impl<N: Float> SubAssign<N> for Complex<N> {
    fn sub_assign(&mut self, other: N) {
        Self::subf_assign(self, other);
    }
}

impl<N: Float> SubAssign for Complex<N> {
    fn sub_assign(&mut self, other: Self) {
        Self::sub_assign(self, &other);
    }
}

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

impl<N: Float+Display> Display for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let rsign = if self.real.is_sign_negative() { '-' } else { '+' };
        let isign = if self.imaginary.is_sign_negative() { '-' } else { '+' };

        return write!(formatter, "( {rsign} {} {isign} {}i )", self.real.abs(), self.imaginary.abs());
    }
}

impl<N: Float+Debug> Debug for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "Complex {{ real: {:?}, imaginary: {:?}i }}", self.real, self.imaginary);
    }
}

