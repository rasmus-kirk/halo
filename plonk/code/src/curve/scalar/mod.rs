mod op_i32;
mod op_i64;
mod op_scalar;
mod op_u32;
mod op_u64;
mod op_usize;

use ark_ff::{AdditiveGroup, BigInteger, FftField, Field, PrimeField};
use halo_accumulation::group::PallasScalar;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{
    fmt,
    ops::{Add, BitXor, Div, Mul, Neg, Sub},
};

/// a ∈ 𝔽ₚ
/// Scalar a, an element of the field 𝔽ₚ
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Scalar {
    pub(crate) scalar: PallasScalar,
}

impl Scalar {
    pub fn new(scalar: PallasScalar) -> Self {
        Scalar { scalar }
    }

    pub const ONE: Self = Scalar {
        scalar: PallasScalar::ONE,
    };

    pub const ZERO: Self = Scalar {
        scalar: PallasScalar::ZERO,
    };

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

    pub fn to_bits(&self) -> Vec<bool> {
        self.scalar.into_bigint().to_bits_be()
    }
}

impl From<Vec<bool>> for Scalar {
    fn from(bits: Vec<bool>) -> Self {
        let scalar = PallasScalar::from_bigint(BigInteger::from_bits_be(&bits)).unwrap();
        Scalar { scalar }
    }
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

// BitXor -----------------------------------------------------

impl BitXor for Scalar {
    type Output = Scalar;

    fn bitxor(self, other: Scalar) -> Scalar {
        let xs = self.to_bits();
        let ys = other.to_bits();
        let mut zs = vec![false; xs.len().max(ys.len())];
        for (i, z) in zs.iter_mut().enumerate() {
            *z = xs.get(i).unwrap_or(&false) ^ ys.get(i).unwrap_or(&false);
        }
        Scalar::from(zs)
    }
}

impl BitXor<&Scalar> for Scalar {
    type Output = Scalar;

    fn bitxor(self, other: &Scalar) -> Scalar {
        self ^ *other
    }
}

impl BitXor<Scalar> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, other: Scalar) -> Scalar {
        *self ^ other
    }
}

impl BitXor<&Scalar> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, other: &Scalar) -> Scalar {
        *self ^ *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar() {
        let a = Scalar::from(3);
        let b = Scalar::from(5);
        let c = a + b;
        assert_eq!(c.to_string(), "8");
        let d = a - b;
        assert_eq!(d.to_string(), "-2");
        let e = a * b;
        assert_eq!(e.to_string(), "15");
        let f = a / b;
        let f2 = b.inverse() * a;
        assert_eq!(f.to_string(), f2.to_string());
        let g = a ^ b;
        assert_eq!(g.to_string(), "6");
    }
}
