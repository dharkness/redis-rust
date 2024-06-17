pub use pattern::Pattern;
pub use sets::{diff, intersect, SetOp, union};
pub use store::{IfKindResult, Store};
pub use value::{Kind, Value};

mod pattern;
mod sets;
mod store;
mod value;
