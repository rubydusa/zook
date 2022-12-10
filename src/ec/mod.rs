use std::ops::{Add, Sub};

use crate::ff::FieldElement;

#[derive(Copy, Clone)]
enum CurvePoint<const A: u32, const B: u32, const P: u32> {
    Zero,
    Point {
        x: FieldElement<P>,
        y: FieldElement<P>,
    },
}

impl<const A: u32, const B: u32, const P: u32> Add for CurvePoint<A, B, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            CurvePoint::Zero => rhs,
            CurvePoint::Point { x: x1, y: y1 } => match rhs {
                CurvePoint::Zero => self,
                CurvePoint::Point { x: x2, y: y2 } => {
                    if x1 == x2 {
                        CurvePoint::Zero
                    } else {
                        let s = (y1 - y2) / (x1 - x2);
                        let x = s * s - x1 - x2;
                        let y = y1 + s * (x2 - x1);

                        CurvePoint::Point { x, y }
                    }
                }
            },
        }
    }
}

impl<const A: u32, const B: u32, const P: u32> Sub for CurvePoint<A, B, P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match rhs {
            CurvePoint::Zero => self,
            CurvePoint::Point { x, y } => {
                let rhs = CurvePoint::Point { x, y: -y };
                self + rhs
            }
        }
    }
}
