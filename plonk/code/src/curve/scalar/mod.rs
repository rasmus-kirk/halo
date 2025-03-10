mod op_i32;
mod op_i64;
mod op_scalar;
mod op_u32;
mod op_u64;
mod op_usize;

use ark_ff::{AdditiveGroup, FftField, Field};
use halo_accumulation::group::PallasScalar;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// a ∈ 𝔽ₚ
/// Scalar a, an element of the field 𝔽ₚ
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Scalar {
    pub(crate) scalar: PallasScalar,
}

impl Scalar {
    pub fn inverse(&self) -> Scalar {
        Scalar {
            scalar: self.scalar.inverse().unwrap(),
        }
    }

    pub fn pow(&self, exp: u64) -> Scalar {
        Scalar {
            scalar: self.scalar.pow([exp]),
        }
    }

    pub fn get_root_of_unity(n: u64) -> Option<Scalar> {
        let scalar = PallasScalar::get_root_of_unity(n)?;
        Some(Scalar { scalar })
    }
}

impl Scalar {
    pub const ONE: Self = Scalar {
        scalar: PallasScalar::ONE,
    };

    pub const ZERO: Self = Scalar {
        scalar: PallasScalar::ZERO,
    };
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.scalar;
        let v1 = val.to_string();
        let v2 = (-val).to_string();
        let str = if v1.len() <= v2.len() {
            v1
        } else {
            format!("-{}", v2)
        };
        write!(f, "{}", str)
    }
}

impl Distribution<Scalar> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Scalar {
        Scalar { scalar: rng.gen() }
    }
}

// Negate -----------------------------------------------------

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        Scalar::from(-self.scalar)
    }
}

// Add -----------------------------------------------------

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar + other.scalar,
        }
    }
}

impl Add<&Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar + other.scalar,
        }
    }
}

impl Add<Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar + other.scalar,
        }
    }
}

impl Add<&Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar + other.scalar,
        }
    }
}

// Sub -----------------------------------------------------

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar - other.scalar,
        }
    }
}

impl Sub<&Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar - other.scalar,
        }
    }
}

impl Sub<Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar - other.scalar,
        }
    }
}

impl Sub<&Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar - other.scalar,
        }
    }
}

// Mul -----------------------------------------------------

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar * other.scalar,
        }
    }
}

impl Mul<&Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar * other.scalar,
        }
    }
}

impl Mul<Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar * other.scalar,
        }
    }
}

impl Mul<&Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar * other.scalar,
        }
    }
}

// Div -----------------------------------------------------

impl Div for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar / other.scalar,
        }
    }
}

impl Div<&Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar / other.scalar,
        }
    }
}

impl Div<Scalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar / other.scalar,
        }
    }
}

impl Div<&Scalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: &Scalar) -> Scalar {
        Scalar {
            scalar: self.scalar / other.scalar,
        }
    }
}
