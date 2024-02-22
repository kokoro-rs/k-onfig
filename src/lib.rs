pub mod konfig;
pub use konfig::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KType {
    Bool,
    Map,
    Object(BTreeMap<&'static str, KType>),
    Number,
    String,
    List(Box<KType>),
    None,
}
