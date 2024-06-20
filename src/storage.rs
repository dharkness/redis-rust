pub use pattern::Pattern;
pub use ranges::{clamp, clamp_range};
pub use sets::{diff, intersect, pop_random_members, Random, random_members, SetOp, union};
pub use store::{IfKindResult, Store};
pub use value::{Kind, Value};

mod pattern;
mod ranges;
mod sets;
mod store;
mod value;
