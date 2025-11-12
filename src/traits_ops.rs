use super::Complex;

use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use num_traits::Float;


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

