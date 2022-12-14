use std::num::NonZeroU32;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FieldElement<const P: u32> {
    val: u32,
}

enum ModularArithmeticError {
    NoMultiplicativeInverse,
}

impl<const P: u32> FieldElement<P> {
    pub fn new(val: u32) -> FieldElement<P> {
        if P == 0 {
            panic!("FieldElement can't have 0 as a modulo")
        }
        FieldElement { val: val % P }
    }

    pub fn val(&self) -> u32 {
        self.val
    }

    pub fn pow(self, rhs: Self) -> Self {
        Self {
            val: modulus_exp(self.val, rhs.val, NonZeroU32::new(P).unwrap()),
        }
    }
}

impl<const P: u32> Add for FieldElement<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_add(self.val, rhs.val, NonZeroU32::new(P).unwrap()),
        }
    }
}

impl<const P: u32> Sub for FieldElement<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_sub(self.val, rhs.val, NonZeroU32::new(P).unwrap()),
        }
    }
}

impl<const P: u32> Mul for FieldElement<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: modulus_mul(self.val, rhs.val, NonZeroU32::new(P).unwrap()),
        }
    }
}

impl<const P: u32> Div for FieldElement<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            val: match modulus_div(
                self.val,
                NonZeroU32::new(rhs.val).expect("division by 0"),
                NonZeroU32::new(P).unwrap(),
            ) {
                Ok(new_val) => new_val,
                Err(ModularArithmeticError::NoMultiplicativeInverse) => panic!("gcd(a,n) != 1"),
            },
        }
    }
}

impl<const P: u32> Neg for FieldElement<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            val: modulus_sub(0, self.val, NonZeroU32::new(P).unwrap()),
        }
    }
}

fn modulus_add(a: u32, b: u32, n: NonZeroU32) -> u32 {
    u32::try_from((u64::from(a) + u64::from(b)).rem_euclid(u64::from(n.get())))
        .expect("unexpected overflow in modulus addition")
}

fn modulus_sub(a: u32, b: u32, n: NonZeroU32) -> u32 {
    if a > b {
        (a - b).rem_euclid(n.get())
    } else {
        let b_inverse = additive_inverse(b, n);
        modulus_add(a, b_inverse, n)
    }
}

fn modulus_mul(a: u32, b: u32, n: NonZeroU32) -> u32 {
    u32::try_from((u64::from(a) * u64::from(b)).rem_euclid(u64::from(n.get())))
        .expect("unexpected overflow in modulus multiplication")
}

fn modulus_div(a: u32, b: NonZeroU32, n: NonZeroU32) -> Result<u32, ModularArithmeticError> {
    Ok(modulus_mul(a, multiplicative_inverse(b, n)?.get(), n))
}

fn modulus_exp(a: u32, b: u32, n: NonZeroU32) -> u32 {
    if n.get() == 1 {
        0
    } else if b == 0 {
        1
    } else {
        let mut acm = 0;
        let mut cur = a;
        let bits = u32::BITS - b.leading_zeros();

        for i in 0..bits {
            cur = modulus_add(cur, cur, n);
            if (b >> i) & 1 == 1 {
                acm = modulus_add(acm, cur, n);
            }
        }

        acm
    }
}

fn additive_inverse(a: u32, n: NonZeroU32) -> u32 {
    n.get() - a.rem_euclid(n.get())
}

// requires gcd(a, n) == 1
fn multiplicative_inverse(
    a: NonZeroU32,
    n: NonZeroU32,
) -> Result<NonZeroU32, ModularArithmeticError> {
    let a = a.get();
    let n = n.get();
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
            return Err(ModularArithmeticError::NoMultiplicativeInverse);
        }
    }

    Ok(NonZeroU32::new(
        u32::try_from(val.rem_euclid(i64::from(n)))
            .expect("multiplicative inverse modulos returned negative value"),
    )
    .expect("multiplicative_inverse returned 0"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "FieldElement can't have 0 as a modulo")]
    fn it_panics_on_modulo_zero() {
        let _a = FieldElement::<0>::new(1);
    }

    #[test]
    fn it_does_not_wrap_on_bigger_modulo() {
        let a = FieldElement::<5>::new(3);
        assert_eq!(a.val(), 3);
    }

    #[test]
    fn it_becomes_zero_on_equal_modulo() {
        let a = FieldElement::<5>::new(5);
        assert_eq!(a.val(), 0);
    }

    #[test]
    fn it_wraps_on_smaller_modulo() {
        let a = FieldElement::<5>::new(7);
        assert_eq!(a.val(), 2);
    }

    mod addition {
        use super::super::*;

        #[test]
        fn non_overflowing_addition() {
            let a = FieldElement::<5>::new(1);
            let b = FieldElement::<5>::new(3);

            assert_eq!((a + b).val(), 4);
        }

        #[test]
        fn overflowing_addition() {
            let a = FieldElement::<5>::new(3);
            let b = FieldElement::<5>::new(4);

            assert_eq!((a + b).val(), 2);
        }

        #[test]
        fn it_is_commutative() {
            let a = FieldElement::<5>::new(3);
            let b = FieldElement::<5>::new(4);

            assert_eq!((a + b).val(), (b + a).val());
        }

        #[test]
        fn it_is_associative() {
            let a = FieldElement::<5>::new(2);
            let b = FieldElement::<5>::new(3);
            let c = FieldElement::<5>::new(4);

            assert_eq!(((a + b) + c).val(), (a + (b + c)).val());
        }

        #[test]
        fn zero_is_identity_element() {
            let a = FieldElement::<5>::new(3);
            let zero = FieldElement::<5>::new(0);

            assert_eq!((a + zero).val(), a.val());
        }
    }

    #[test]
    #[should_panic(expected = "division by 0")]
    fn it_panics_on_division_by_zero() {
        let a = FieldElement::<5>::new(3);
        let b = FieldElement::<5>::new(0);
        let _c = a / b;
    }
}
