use std::fmt;
use std::marker::PhantomData;
use serde::de::{self, Visitor};
use serde::Deserializer;

pub fn bool_or_bit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    struct BoolOrBit(PhantomData<fn() -> bool>);

    impl<'de> Visitor<'de> for BoolOrBit {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("true, false, 0, or 1")
        }

        fn visit_bool<E: de::Error>(self, value: bool) -> Result<bool, E> {
            Ok(value)
        }

        fn visit_u8<E: de::Error>(self, value: u8) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_u32<E: de::Error>(self, value: u32) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_i8<E: de::Error>(self, value: i8) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_i16<E: de::Error>(self, value: i16) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_i32<E: de::Error>(self, value: i32) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<bool, E> {
            Ok(value != 0)
        }
    }

    deserializer.deserialize_any(BoolOrBit(PhantomData))
}
