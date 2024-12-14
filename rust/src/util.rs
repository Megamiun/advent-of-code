use derive_more::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};
use forward_ref_generic::forward_ref_binop;
use num_traits::ToPrimitive;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{_0}; {_1}]")]
pub struct Index2D(pub usize, pub usize);

impl Index2D {
    pub fn get_distance_to(&self, other: Index2D) -> Diff {
        self.as_diff().sub(other.as_diff())
    }

    pub fn as_diff(&self) -> Diff {
        Diff(self.0 as i32, self.1 as i32)
    }

    pub fn from_diff(Diff(x_diff, y_diff): Diff) -> Option<Index2D> {
        let new_x = usize::try_from(x_diff);
        let new_y = usize::try_from(y_diff);

        match (new_x, new_y) {
            (Ok(new_x_val), Ok(new_y_val)) => Some(Index2D(new_x_val, new_y_val)),
            _ => None,
        }
    }
}

impl Add<Index2D> for Index2D {
    type Output = Index2D;
    fn add(self, other: Index2D) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Index2D> for Index2D {
    type Output = Option<Index2D>;
    fn sub(self, other: Index2D) -> Self::Output {
        Some(Self(self.0.checked_sub(other.0)?, self.1.checked_sub(other.1)?))
    }
}

impl Mul<usize> for Index2D {
    type Output = Index2D;

    fn mul(self, rhs: usize) -> Self::Output {
        Index2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<usize> for Index2D {
    type Output = Index2D;

    fn div(self, rhs: usize) -> Self::Output {
        Index2D(self.0 / rhs, self.1 / rhs)
    }
}

impl Rem<Index2D> for Index2D {
    type Output = Index2D;

    fn rem(self, rhs: Index2D) -> Self::Output {
        Index2D(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl Add<Diff> for Index2D {
    type Output = Option<Index2D>;
    fn add(self, other: Diff) -> Self::Output {
        Self::from_diff(self.as_diff() + other)
    }
}

impl Sub<Diff> for Index2D {
    type Output = Option<Index2D>;
    fn sub(self, other: Diff) -> Self::Output {
        Self::from_diff(self.as_diff() - other)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("({_0}; {_1})")]
pub struct Diff(pub i32, pub i32);

impl Add<Diff> for Diff {
    type Output = Diff;
    fn add(self, other: Diff) -> Self::Output {
        Diff(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Diff> for Diff {
    type Output = Diff;
    fn sub(self, other: Diff) -> Self::Output {
        Diff(self.0 - other.0, self.1 - other.1)
    }
}

impl Add<Index2D> for Diff {
    type Output = Diff;
    fn add(self, other: Index2D) -> Self::Output {
        self + other.as_diff()
    }
}

impl Mul<usize> for Diff {
    type Output = Option<Diff>;

    fn mul(self, rhs: usize) -> Self::Output {
        Some(Diff(self.0 * rhs.to_i32()?, self.1 * rhs.to_i32()?))
    }
}

impl Div<usize> for Diff {
    type Output = Option<Diff>;

    fn div(self, rhs: usize) -> Self::Output {
        Some(Diff(self.0 / rhs.to_i32()?, self.1 / rhs.to_i32()?))
    }
}

forward_ref_binop! { impl Sub for Diff }
forward_ref_binop! { impl Add for Diff }
forward_ref_binop! { impl Add for Diff, Index2D }
forward_ref_binop! { impl Mul for Diff, usize }
forward_ref_binop! { impl Div for Diff, usize }

forward_ref_binop! { impl Sub for Index2D }
forward_ref_binop! { impl Sub for Index2D, Diff }
forward_ref_binop! { impl Add for Index2D }
forward_ref_binop! { impl Add for Index2D, Diff }
forward_ref_binop! { impl Mul for Index2D, usize }
forward_ref_binop! { impl Div for Index2D, usize }
forward_ref_binop! { impl Rem, rem for Index2D }