use crate::{SIDimension, SIUnit};

impl std::fmt::Display for SIUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.dim)
    }
}

impl SIUnit {
    pub const fn new(value: f64, l: i8, m: i8, t: i8, i: i8, o: i8, n: i8, j: i8) -> Self {
        Self {
            value,
            dim: SIDimension {
                l,
                m,
                t,
                i,
                o,
                n,
                j,
            },
        }
    }

    pub fn dim_eq(&self, other: &Self) -> bool {
        self.dim == other.dim
    }

    // SI base units
    pub const fn meter(value: f64) -> Self {
        Self::new(value, 1, 0, 0, 0, 0, 0, 0)
    }

    pub const fn kilogram(value: f64) -> Self {
        Self::new(value, 0, 1, 0, 0, 0, 0, 0)
    }

    pub const fn second(value: f64) -> Self {
        Self::new(value, 0, 0, 1, 0, 0, 0, 0)
    }

    pub const fn ampere(value: f64) -> Self {
        Self::new(value, 0, 0, 0, 1, 0, 0, 0)
    }

    pub const fn kelvin(value: f64) -> Self {
        Self::new(value, 0, 0, 0, 0, 1, 0, 0)
    }

    pub const fn mole(value: f64) -> Self {
        Self::new(value, 0, 0, 0, 0, 0, 1, 0)
    }

    pub const fn candela(value: f64) -> Self {
        Self::new(value, 0, 0, 0, 0, 0, 0, 1)
    }

    // Derived units
    pub const fn hertz(value: f64) -> Self {
        // Hz = 1 / s
        Self::new(value, 0, 0, -1, 0, 0, 0, 0)
    }

    pub const fn newton(value: f64) -> Self {
        // N = kg * m / s^2
        Self::new(value, 1, 1, -2, 0, 0, 0, 0)
    }

    pub const fn pascal(value: f64) -> Self {
        // Pa = kg / m / s^2
        Self::new(value, -1, 1, -2, 0, 0, 0, 0)
    }

    pub const fn degree_celcius(value: f64) -> Self {
        // °C = K + 273.15
        Self::new(value + 273.15, 0, 0, 0, 1, 0, 0, 0)
    }

    // TODO: add more (https://en.wikipedia.org/wiki/International_System_of_Units)
}

impl std::ops::Add for SIUnit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.dim_eq(&rhs));
        Self::Output {
            value: self.value + rhs.value,
            dim: self.dim,
        }
    }
}

impl std::ops::Sub for SIUnit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.dim_eq(&rhs));
        Self::Output {
            value: self.value - rhs.value,
            dim: self.dim,
        }
    }
}

impl std::ops::Mul for SIUnit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
            dim: self.dim + rhs.dim,
        }
    }
}

impl std::ops::Div for SIUnit {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
            dim: self.dim - rhs.dim,
        }
    }
}

// mul/div by a scalar

impl std::ops::Mul<f64> for SIUnit {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            value: self.value * rhs,
            dim: self.dim,
        }
    }
}

impl std::ops::Div<f64> for SIUnit {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            value: self.value / rhs,
            dim: self.dim,
        }
    }
}

impl std::ops::Mul<SIUnit> for f64 {
    type Output = SIUnit;
    fn mul(self, rhs: SIUnit) -> Self::Output {
        Self::Output {
            value: self * rhs.value,
            dim: rhs.dim,
        }
    }
}

impl std::ops::Div<SIUnit> for f64 {
    type Output = SIUnit;
    fn div(self, rhs: SIUnit) -> Self::Output {
        Self::Output {
            value: self / rhs.value,
            dim: rhs.dim,
        }
    }
}
