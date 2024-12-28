use num_traits::Num;
use paste::paste;

macro_rules! define_parse {
    ($num_type:ty) => {
        paste! {
            pub fn [<parse_ $num_type>](num: &str) -> $num_type {
                $num_type::from_str_radix(num, 10).unwrap()
            }
        }
    };
}

define_parse!(isize);
define_parse!(i128);
define_parse!(i64);
define_parse!(i32);
define_parse!(usize);
define_parse!(u128);
define_parse!(u64);
define_parse!(u32);
define_parse!(f64);
define_parse!(f32);
