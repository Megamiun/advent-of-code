use crate::util::direction::Direction;
use derive_more::Display;
use forward_ref_generic::forward_ref_binop;
use num_traits::ToPrimitive;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug, Ord, PartialOrd)]
#[display("[{_0}; {_1}]")]
pub struct Index2D(pub usize, pub usize);

impl Index2D {
    pub fn as_diff(&self) -> Diff {
        Diff(self.0 as i32, self.1 as i32)
    }

    pub fn from_diff(Diff(x_diff, y_diff): &Diff) -> Option<Index2D> {
        let new_x = x_diff.to_usize()?;
        let new_y = y_diff.to_usize()?;

        Some(Index2D(new_x, new_y))
    }
}

impl Add<Direction> for Index2D {
    type Output = Option<Index2D>;
    
    fn add(self, other: Direction) -> Self::Output {
        self + other.get_dir()
    }
}

impl Add<Index2D> for Index2D {
    type Output = Index2D;
    
    fn add(self, other: Index2D) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Index2D> for Index2D {
    type Output = Diff;
    
    fn sub(self, other: Index2D) -> Self::Output {
        Diff(self.0 as i32 - other.0 as i32, self.1 as i32 - other.1 as i32)
    }
}

impl Mul<usize> for Index2D {
    type Output = Index2D;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<usize> for Index2D {
    type Output = Index2D;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Rem<Index2D> for Index2D {
    type Output = Index2D;

    fn rem(self, rhs: Index2D) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl Add<Diff> for Index2D {
    type Output = Option<Index2D>;

    fn add(self, other: Diff) -> Self::Output {
        Self::from_diff(&(self.as_diff() + other))
    }
}

impl Sub<Diff> for Index2D {
    type Output = Option<Index2D>;

    fn sub(self, other: Diff) -> Self::Output {
        Self::from_diff(&(self.as_diff() - other))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug, Ord, PartialOrd)]
#[display("({_0}; {_1})")]
pub struct Diff(pub i32, pub i32);

impl Add<Direction> for Diff {
    type Output = Diff;

    fn add(self, other: Direction) -> Self::Output {
        self + other.get_dir()
    }
}

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
        Diff(self.0 - other.0 as i32, self.1 - other.1 as i32)
    }
}

impl Mul<usize> for Diff {
    type Output = Diff;

    fn mul(self, rhs: usize) -> Self::Output {
        Diff(self.0 * rhs.to_i32().unwrap(), self.1 * rhs.to_i32().unwrap())
    }
}

impl Mul<i32> for Diff {
    type Output = Diff;

    fn mul(self, rhs: i32) -> Self::Output {
        Diff(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<usize> for Diff {
    type Output = Diff;

    fn div(self, rhs: usize) -> Self::Output {
        Diff(self.0 / rhs.to_i32().unwrap(), self.1 / rhs.to_i32().unwrap())
    }
}

impl Div<i32> for Diff {
    type Output = Diff;

    fn div(self, rhs: i32) -> Self::Output {
        Diff(self.0 / rhs, self.1 / rhs)
    }
}

forward_ref_binop! { impl Sub for Diff }
forward_ref_binop! { impl Add for Diff }
forward_ref_binop! { impl Add for Diff, Index2D }
forward_ref_binop! { impl Add for Diff, Direction }
forward_ref_binop! { impl Mul for Diff, usize }
forward_ref_binop! { impl Mul for Diff, i32 }
forward_ref_binop! { impl Div for Diff, usize }
forward_ref_binop! { impl Div for Diff, i32 }

forward_ref_binop! { impl Sub for Index2D }
forward_ref_binop! { impl Sub for Index2D, Diff }
forward_ref_binop! { impl Add for Index2D }
forward_ref_binop! { impl Add for Index2D, Diff }
forward_ref_binop! { impl Add for Index2D, Direction }
forward_ref_binop! { impl Mul for Index2D, usize }
forward_ref_binop! { impl Div for Index2D, usize }
forward_ref_binop! { impl Rem, rem for Index2D }