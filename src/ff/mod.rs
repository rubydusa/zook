use std::num::Wrapping;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
struct FieldElement<const P: u32> {
    val: u32,
}

impl<const P: u32> FieldElement<P> {
    pub fn new(val: u32) -> FieldElement<P> {
        FieldElement { val }
    }

    pub fn val(&self) -> u32 {
        self.val
    }
}

fn modulus_add(a: u32, b: u32, n: u32) -> u32 {
    u32::try_from((u64::from(a) + u64::from(b)).rem_euclid(u64::from(n)))
        .expect("unexpected overflow in modulus addition")
}

fn modulus_sub(a: u32, b: u32, n: u32) -> u32 {
    if a > b {
        (a - b).rem_euclid(n)
    } else {
        let b_inverse = additive_inverse(b, n);
        modulus_add(a, b_inverse, n)
    }
}

fn modulus_mul(a: u32, b: u32, n: u32) -> u32 {
    u32::try_from((u64::from(a) * u64::from(b)).rem_euclid(u64::from(n)))
        .expect("unexpected overflow in modulus multiplication")
}

fn additive_inverse(a: u32, n: u32) -> u32 {
    u32::try_from((-i64::from(a)).rem_euclid(i64::from(n)))
        .expect("unexpected overflow in modulus additive invesrse")
}

// assumes gcd(a, n) == 1
fn multiplicative_inverse(a: u32, n: u32) -> u32 {
    if !a < n {
        panic!("p should be greater than n");
    }
    if !a > 0 {
        panic!("n and p should be positive");
    }
    // no need to check if p is positive because if p is not positive and n is greater than equal
    // than p, n is also not positive and panic would be triggered

    let mut cur_n = i64::from(n);
    let mut cur_a = i64::from(a);

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

    u32::try_from(val.rem_euclid(i64::from(n)))
        .expect("multiplicative inverse modulos returned negative value")
}
