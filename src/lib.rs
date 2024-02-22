pub mod konfig;
pub use konfig::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum KType {
    Bool,
    Map(Box<KType>),
    Object(BTreeMap<&'static str, KType>),
    Number,
    String,
    List(Box<KType>),
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum _KType {
    Bool,
    Map(Box<_KType>),
    Object(HashMap<String, _KType>),
    Number,
    String,
    List(Box<_KType>),
    None,
}
impl From<KType> for _KType {
    fn from(value: KType) -> Self {
        match value {
            KType::Bool => _KType::Bool,
            KType::Map(b) => _KType::Map(Box::new((*b).into())),
            KType::Object(m) => _KType::Object(
                m.iter()
                    .map(|(s, t)| (s.to_string(), t.clone().into()))
                    .collect(),
            ),
            KType::Number => _KType::Number,
            KType::String => _KType::String,
            KType::List(l) => _KType::List(Box::new((*l).into())),
            KType::None => _KType::None,
        }
    }
}
