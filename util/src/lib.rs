use std::error;

mod runtime_error;
pub fn make_runtime_error(message: String) -> Box<dyn error::Error> {
    Box::new(RuntimeError::new(message))
}
#[macro_export]
macro_rules! runtime_error {
    ($($t:tt)*) => {
        {
            use util::make_runtime_error;
            make_runtime_error(format!($($t)*))
        }
    };
}
pub use runtime_error::RuntimeError;

mod disjoint_sets;
pub use disjoint_sets::DisjointSets;

mod int_disjoint_sets;
pub use int_disjoint_sets::IntDisjointSets;

mod knot_hash;
pub use knot_hash::knot_hash;
pub use knot_hash::KnotHashState;
