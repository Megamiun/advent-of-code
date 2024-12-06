use derive_more::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{_0}; {_1}]")]
pub struct Index2D(pub usize, pub usize);

impl Index2D {
    pub fn add(&self, Diff(x_diff, y_diff): Diff) -> Option<Index2D> {
        let Index2D(x, y) = self;
        let new_x = usize::try_from(*x as i32 + x_diff);
        let new_y = usize::try_from(*y as i32 + y_diff);

        match (new_x, new_y) {
            (Ok(new_x_val), Ok(new_y_val)) => Some(Index2D(new_x_val, new_y_val)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("({_0}; {_1})")]
pub struct Diff(pub i32, pub i32);
