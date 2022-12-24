mod runtime_error;
#[macro_export]
macro_rules! runtime_error {
    ($($t:tt)*) => {
        {
            use util::make_runtime_error;
            make_runtime_error(format!($($t)*))
        }
    };
}
pub use runtime_error::make_runtime_error;
pub use runtime_error::Error;
pub use runtime_error::Result;
pub use runtime_error::RuntimeError;

mod disjoint_sets;
pub use disjoint_sets::DisjointSets;

mod int_disjoint_sets;
pub use int_disjoint_sets::IntDisjointSets;

mod knot_hash;
pub use knot_hash::knot_hash;
pub use knot_hash::KnotHashState;
