use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::KType;
pub use k_onfig_derive::Konfig;

pub trait Konfig {
    fn konfig() -> KType;
}

macro_rules! konfig_number {
    ($type:ty) => {
        impl Konfig for $type {
            fn konfig() -> crate::KType {
                KType::Number
            }
        }
    };
}
konfig_number!(i8);
konfig_number!(i16);
konfig_number!(i32);
konfig_number!(i64);
konfig_number!(i128);
konfig_number!(u8);
konfig_number!(u16);
konfig_number!(u32);
konfig_number!(u64);
konfig_number!(u128);

macro_rules! konfig_string {
    ($type:ty) => {
        impl Konfig for $type {
            fn konfig() -> crate::KType {
                KType::String
            }
        }
    };
}
konfig_string!(String);
konfig_string!(str);

macro_rules! konfig_list {
    ($type:ident) => {
        impl<T: Konfig> Konfig for $type<T> {
            fn konfig() -> KType {
                KType::List(Box::new(T::konfig()))
            }
        }
    };
}
konfig_list!(Vec);
konfig_list!(HashSet);

macro_rules! konfig_map {
    ($type:ident) => {
        impl<V: Konfig, K> Konfig for $type<K, V> {
            fn konfig() -> KType {
                KType::Map(Box::new(V::konfig()))
            }
        }
    };
}
konfig_map!(HashMap);
konfig_map!(BTreeMap);

impl Konfig for bool {
    fn konfig() -> KType {
        KType::Bool
    }
}
