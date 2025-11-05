use std::fmt;


#[derive(Clone)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64
}

impl Complex {
    pub const fn new(real: f64, imaginary: f64) -> Self {
        return Self {
            real: real,
            imaginary: imaginary
        };
    }

    pub const fn zero() -> Self {
        return Self {
            real: 0.0,
            imaginary: 0.0
        };
    }

    pub const fn one() -> Self {
        return Self {
            real: 1.0,
            imaginary: 0.0
        };
    }

    pub const fn i() -> Self {
        return Self {
            real: 0.0,
            imaginary: 1.0
        };
    }

    pub const fn from_real(real: f64) -> Self {
        return Self {
            real: real,
            imaginary: 0.0
        };
    }

    pub fn from_argument_amplitude(argument: f64, amplitude: f64) -> Self {
        return Self {
            real: argument.cos() * amplitude,
            imaginary: argument.sin() * amplitude
        };
    }

    pub const fn is_pure_real(&self) -> bool {
        return self.imaginary == 0.0;
    }

    pub const fn is_pure_imaginary(&self) -> bool {
        return self.real == 0.0;
    }

    pub const fn extract_real(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: 0.0
        };
    }

    pub const fn extract_imaginary(&self) -> Self {
        return Self {
            real: 0.0,
            imaginary: self.imaginary
        };
    }

    pub fn amplitude(&self) -> f64 {
        return (self.real * self.real + self.imaginary * self.imaginary).sqrt();
    }

    pub fn argument(&self) -> f64 {
        return self.imaginary.atan2(self.real);
    }

    pub const fn neg(&self) -> Self {
        return Self {
            real: -self.real,
            imaginary: -self.imaginary
        };
    }

    pub fn inv(&self) -> Self {
        let divisor = self.real.powi(2) + self.imaginary.powi(2);

        return Self {
            real: self.real / divisor,
            imaginary: -self.imaginary / divisor
        };
    }

    pub const fn conj(&self) -> Self {
        return Self {
            real: self.real,
            imaginary: -self.imaginary
        };
    }

    pub const fn add(&self, other: &Self) -> Self {
        return Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        };
    }

    pub const fn mult(&self, other: &Self) -> Self {
        return Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real
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

    pub fn log(&self, base: f64) -> Self {
        return self.ln().mult(&Self::from_real(base).ln().inverse());
    }

    pub fn powi(&self,  exponent: isize) -> Self {
        let mut n = Self::one();
        let (mut base, mut exponent) = if exponent > 0 {
            (self.clone(), exponent)
        } else {
            (self.inverse(), -exponent)
        };

        while exponent > 0 {
            if exponent % 2 == 1 {
                n = n.mult(&base);
            }
            base = base.mult(&base);

            exponent /= 2;
        }

        return n;
    }

    pub fn powf(&self, exponent: f64) -> Self {
        return (self.ln().mult(&Self::from_real(exponent))).exp();
    }
}

impl Default for Complex {
    fn default() -> Self {
        return Self::zero();
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let rsign = if self.real < 0.0 { '-' } else { '+' };
        let isign = if self.imaginary < 0.0 { '-' } else { '+' };
        write!(formatter, "( {rsign} {} {isign} {}i )", self.real.abs(), self.imaginary.abs())
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Complex {{ real: {}, imaginary: {}i )", self.real, self.imaginary)
    }
}

