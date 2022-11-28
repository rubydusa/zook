use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct FieldElement<const P: u32> {
    val: u32,
}

#[derive(Debug)]
enum ModularArithmeticError {
    ZeroModulo,
    ZeroMultiplicativeInverse,
}

impl<const P: u32> FieldElement<P> {
    pub fn new(val: u32) -> FieldElement<P> {
        if P == 0 {
            panic!("FieldElement can't have 0 as a modulo")
        }
        FieldElement { val }
    }

    pub fn val(&self) -> u32 {
        self.val
    }
}

impl<const P: u32> Add for FieldElement<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_add(self.val, rhs.val, P).expect("something went terribly wrong"),
        }
    }
}

impl<const P: u32> Sub for FieldElement<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_sub(self.val, rhs.val, P).expect("something went terribly wrong"),
        }
    }
}

impl<const P: u32> Mul for FieldElement<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_mul(self.val, rhs.val, P).expect("something went terribly wrong"),
        }
    }
}

impl<const P: u32> Div for FieldElement<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            val: match modulus_div(self.val, rhs.val, P) {
                Err(ModularArithmeticError::ZeroModulo) => {
                    panic!("something went terribly wrong")
                }
                Err(ModularArithmeticError::ZeroMultiplicativeInverse) => {
                    panic!("zero multiplicative inverse")
                }
                Ok(new_val) => new_val,
            },
        }
    }
}

fn modulus_add(a: u32, b: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        Err(ModularArithmeticError::ZeroModulo)
    } else {
        Ok(
            u32::try_from((u64::from(a) + u64::from(b)).rem_euclid(u64::from(n)))
                .expect("unexpected overflow in modulus addition"),
        )
    }
}

fn modulus_sub(a: u32, b: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        Err(ModularArithmeticError::ZeroModulo)
    } else if a > b {
        Ok((a - b).rem_euclid(n))
    } else {
        let b_inverse = additive_inverse(b, n);
        modulus_add(a, b_inverse?, n)
    }
}

fn modulus_mul(a: u32, b: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        Err(ModularArithmeticError::ZeroModulo)
    } else {
        Ok(
            u32::try_from((u64::from(a) * u64::from(b)).rem_euclid(u64::from(n)))
                .expect("unexpected overflow in modulus multiplication"),
        )
    }
}

fn modulus_div(a: u32, b: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        Err(ModularArithmeticError::ZeroModulo)
    } else {
        modulus_mul(a, multiplicative_inverse(b, n)?, n)
    }
}

fn additive_inverse(a: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        Err(ModularArithmeticError::ZeroModulo)
    } else {
        Ok(n - a.rem_euclid(n))
    }
}

// requires gcd(a, n) == 1
fn multiplicative_inverse(a: u32, n: u32) -> Result<u32, ModularArithmeticError> {
    if n == 0 {
        return Err(ModularArithmeticError::ZeroModulo);
    }
    if a == 0 {
        return Err(ModularArithmeticError::ZeroMultiplicativeInverse);
    }
    // no need to check if p is positive because if p is not positive and n is greater than equal
    // than p, n is also not positive and panic would be triggered

    let mut cur_n = i64::from(n);
    let mut cur_a = i64::from(a.rem_euclid(n));

    let mut val = 1;
    let mut pre = 0;
    let mut rem = cur_n % cur_a;

    while rem != 0 {
        let val_cof = -(cur_n / cur_a);
        (val, pre) = (pre + val * val_cof, val);

        cur_n = cur_a;
        cur_a = rem;

        rem = cur_n % cur_a;

        if rem == 0 && cur_a != 1 {
            panic!("gcd(a,n) != 1");
        }
    }

    Ok(u32::try_from(val.rem_euclid(i64::from(n)))
        .expect("multiplicative inverse modulos returned negative value"))
}
