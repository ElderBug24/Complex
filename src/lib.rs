use std::fmt::{self, Debug, Display};
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

use num_traits::{Num, Zero, One, Float, Signed, Inv, Pow};


pub trait ComplexInnerType: Num + Zero + One + Float + Signed + Clone + PartialEq + Debug + Display {}
impl<T> ComplexInnerType for T where T: Num + Zero + One + Float + Signed + Clone + PartialEq + Debug + Display {}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Complex<N: ComplexInnerType> {
    pub real: N,
    pub imaginary: N
}

impl<N: ComplexInnerType> Complex<N> {
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

    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        // assert!(bytes.len() == std::mem::size_of::<Self>());
        let ptr = bytes.as_ptr() as *const Self;

        return ptr.read_unaligned();
    }

    pub unsafe fn as_bytes(&self) -> &[u8] {
        return std::slice::from_raw_parts((self as *const Self) as *const u8, std::mem::size_of::<Self>());
    }

    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        return std::slice::from_raw_parts_mut((self as *mut Self) as *mut u8, std::mem::size_of::<Self>());
    }

    pub unsafe fn to_bytes<const LEN: usize>(&self) -> [u8; LEN] {
        // assert_eq!(LEN, std::mem::size_of::<Self>());
        let mut arr: [u8; LEN] = std::mem::MaybeUninit::uninit().assume_init();
        std::ptr::copy_nonoverlapping(self as *const Self as *const u8, arr.as_mut_ptr(), LEN);

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

    pub fn conj(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: -self.imaginary
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
        return self.ln() * Self::from_real(base).ln().inv();
    }

    pub fn powi(&self,  exponent: isize) -> Self {
        let mut n = Self::one();
        let (mut base, mut exponent) = if exponent > 0 {
            (*self, exponent)
        } else {
            (self.inv(), -exponent)
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
}

impl<N: ComplexInnerType> Add for Complex<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        };
    }
}

impl<N: ComplexInnerType> AddAssign for Complex<N> {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

impl<N: ComplexInnerType> BitAnd for Complex<N> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = &self as *const Self as *const u8;
            let other_bytes = &other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) & *other_bytes.add(i);
            }

            return result;
        }
    }
}

impl<N: ComplexInnerType> BitAndAssign for Complex<N> {
    fn bitand_assign(&mut self, other: Self) {
        *self = self.bitand(other);
    }
}

impl<N: ComplexInnerType> BitOr for Complex<N> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = &self as *const Self as *const u8;
            let other_bytes = &other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) | *other_bytes.add(i);
            }

            return result;
        }
    }
}

impl<N: ComplexInnerType> BitOrAssign for Complex<N> {
    fn bitor_assign(&mut self, other: Self) {
        *self = self.bitor(other);
    }
}

impl<N: ComplexInnerType> BitXor for Complex<N> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        unsafe {
            let mut result: Self = std::mem::zeroed();
            let self_bytes = &self as *const Self as *const u8;
            let other_bytes = &other as *const Self as *const u8;
            let result_bytes = &mut result as *mut Self as *mut u8;

            for i in 0..std::mem::size_of::<Self>() {
                *result_bytes.add(i) = *self_bytes.add(i) ^ *other_bytes.add(i);
            }

            return result;
        }
    }
}

impl<N: ComplexInnerType> BitXorAssign for Complex<N> {
    fn bitxor_assign(&mut self, other: Self) {
        *self = self.bitxor(other);
    }
}

impl<N: ComplexInnerType> Deref for Complex<N> {
    type Target = (N, N);

    fn deref(&self) -> &Self::Target {
        let ptr = &(&self.real, &self.imaginary) as *const (&N, &N) as *const &(N, N);
        let ptr: &(N, N) = unsafe { &*ptr };

        return ptr;
    }
}

impl<N: ComplexInnerType> DerefMut for Complex<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = &mut (&mut self.real, &mut self.imaginary) as *mut (&mut N, &mut N) as *mut &mut (N, N);
        let ptr: &mut (N, N) = unsafe { &mut *ptr };

        return ptr;
    }
}

impl<N: ComplexInnerType> Div for Complex<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.real * other.real + other.imaginary * other.imaginary;
        return Self {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denominator,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denominator
        };
    }
}

impl<N: ComplexInnerType> DivAssign for Complex<N> {
    fn div_assign(&mut self, other: Self) {
        *self = self.div(other);
    }
}

impl<N: ComplexInnerType> Index<bool> for Complex<N> {
    type Output = N;

    fn index(&self, index: bool) -> &Self::Output {
        return match index {
            false => &self.real,
            true => &self.imaginary
        };
    }
}

impl<N: ComplexInnerType> IndexMut<bool> for Complex<N> {
    fn index_mut(&mut self, index: bool) -> &mut Self::Output {
        return match index {
            false => &mut self.real,
            true => &mut self.imaginary
        };
    }
}

impl<N: ComplexInnerType> Mul for Complex<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real
        };
    }
}

impl<N: ComplexInnerType> MulAssign for Complex<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other);
    }
}

impl<N: ComplexInnerType> Neg for Complex<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self {
            real: -self.real,
            imaginary: -self.imaginary
        };
    }
}

impl<N: ComplexInnerType> Not for Complex<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self {
            real: self.real,
            imaginary: -self.imaginary
        };
    }
}

impl<N: ComplexInnerType> Shl<usize> for Complex<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let size = std::mem::size_of::<Self>();
        let mut bytes = vec![0u8; size];

        unsafe {
            let self_ptr = &self as *const Self as *const u8;
            for i in 0..size {
                bytes[i] = *self_ptr.add(i);
            }

            let byte_shift = rhs / 8;
            let bit_shift = rhs % 8;

            if byte_shift > 0 {
                for i in 0..size {
                    bytes[i] = if i + byte_shift < size { bytes[i + byte_shift] } else { 0 };
                }
            }

            if bit_shift > 0 {
                let mut carry = 0u8;
                for i in (0..size).rev() {
                    let new_carry = bytes[i] >> (8 - bit_shift);
                    bytes[i] = (bytes[i] << bit_shift) | carry;
                    carry = new_carry;
                }
            }

            let mut result: Self = std::mem::zeroed();
            let r_ptr = &mut result as *mut Self as *mut u8;
            for i in 0..size {
                *r_ptr.add(i) = bytes[i];
            }

            return result;
        }
    }
}

impl<N: ComplexInnerType> ShlAssign<usize> for Complex<N> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.shl(rhs);
    }
}

impl<N: ComplexInnerType> Shr<usize> for Complex<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let size = std::mem::size_of::<Self>();
        let mut bytes = vec![0u8; size];

        unsafe {
            let self_ptr = &self as *const Self as *const u8;
            for i in 0..size {
                bytes[i] = *self_ptr.add(i);
            }

            let byte_shift = rhs / 8;
            let bit_shift = rhs % 8;

            if byte_shift > 0 {
                for i in (0..size).rev() {
                    bytes[i] = if i >= byte_shift { bytes[i - byte_shift] } else { 0 };
                }
            }

            if bit_shift > 0 {
                let mut carry = 0u8;
                for i in 0..size {
                    let new_carry = bytes[i] << (8 - bit_shift);
                    bytes[i] = (bytes[i] >> bit_shift) | carry;
                    carry = new_carry;
                }
            }

            let mut result: Self = std::mem::zeroed();
            let r_ptr = &mut result as *mut Self as *mut u8;
            for i in 0..size {
                *r_ptr.add(i) = bytes[i];
            }

            return result;
        }
    }
}

impl<N: ComplexInnerType> ShrAssign<usize> for Complex<N> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.shr(rhs);
    }
}

impl<N: ComplexInnerType> Sub for Complex<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        };
    }
}

impl<N: ComplexInnerType> SubAssign for Complex<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other);
    }
}

impl<N: ComplexInnerType> Inv for Complex<N> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let divisor = self.real * self.real + self.imaginary * self.imaginary;

        return Self {
            real: self.real / divisor,
            imaginary: -self.imaginary / divisor
        };
    }
}

impl<N: ComplexInnerType> Pow<N> for Complex<N> {
    type Output = Self;

    fn pow(self, other: N) -> Self::Output {
        return (self.ln() * Self::from_real(other)).exp();
    }
}

impl<N: ComplexInnerType> Pow<Self> for Complex<N> {
    type Output = Self;

    fn pow(self, other: Self) -> Self::Output {
        let r = self.amplitude();
        let arg = self.argument();
        let m = r.powf(other.real) * (-other.imaginary * arg).exp();
        let theta = other.imaginary * r.ln() + other.real * arg;

        return Self::from_argument_amplitude(theta, m);
    }
}

impl<N: ComplexInnerType> Zero for Complex<N> {
    fn zero() -> Self {
        return Self {
            real: N::zero(),
            imaginary: N::zero()
        };
    }

    fn is_zero(&self) -> bool {
        return self.real.is_zero() && self.imaginary.is_zero();
    }

    fn set_zero(&mut self) {
        self.real = N::zero();
        self.imaginary = N::zero();
    }
}

impl<N: ComplexInnerType> One for Complex<N> {
    fn one() -> Self {
        return Self {
            real: N::one(),
            imaginary: N::zero()
        };
    }

    fn is_one(&self) -> bool {
        return self.real.is_one() && self.imaginary.is_zero();
    }

    fn set_one(&mut self) {
        self.real = N::one();
        self.imaginary = N::zero();
    }
}

impl<N: ComplexInnerType> Default for Complex<N> {
    fn default() -> Self {
        return Self::zero();
    }
}

impl<N: ComplexInnerType> Display for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let rsign = if self.real.is_negative() { '-' } else { '+' };
        let isign = if self.imaginary.is_negative() { '-' } else { '+' };
        write!(formatter, "( {rsign} {} {isign} {}i )", self.real.abs(), self.imaginary.abs())
    }
}

impl<N: ComplexInnerType> Debug for Complex<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Complex {{ real: {}, imaginary: {}i )", self.real, self.imaginary)
    }
}

