use derive_more::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{_0}; {_1}]")]
pub struct Index2D(pub usize, pub usize);

impl Index2D {
    pub fn get_distance_to(&self, other: Index2D) -> Diff {
        self.as_diff().sub(other.as_diff())
    }

    pub fn add(&self, diff: Diff) -> Option<Index2D> {
        Self::from_diff(self.as_diff().add(diff))
    }

    pub fn sub(&self, diff: Diff) -> Option<Index2D> {
        Self::from_diff(self.as_diff().sub(diff))
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("({_0}; {_1})")]
pub struct Diff(pub i32, pub i32);

impl Diff {
    pub fn add(&self, other: Diff) -> Diff {
        Diff(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: Diff) -> Diff {
        Diff(self.0 - other.0, self.1 - other.1)
    }
}