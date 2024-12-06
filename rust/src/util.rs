use derive_more::{Display};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{_0}; {_1}]")]
pub struct Index2D(pub usize, pub usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("({_0}; {_1})")]
pub struct Diff(pub i32, pub i32);
