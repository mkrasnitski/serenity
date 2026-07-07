use std::fmt;
use std::str::FromStr;

use nonmax::NonMaxU64;
use serde::de::{Deserializer, Error};
use to_arraystring::ToArrayString;

/// The inner storage of an ID.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(Rust, packed)]
#[must_use]
pub struct Snowflake(NonMaxU64);

impl Snowflake {
    pub const fn new(id: u64) -> Self {
        let Some(inner) = NonMaxU64::new(id) else {
            panic!("Attempted to call Snowflake::new with invalid (u64::MAX) value")
        };

        Self(inner)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        { self.0 }.get()
    }
}

impl fmt::Debug for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        { self.0 }.fmt(f)
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        { self.0 }.fmt(f)
    }
}

impl FromStr for Snowflake {
    type Err = nonmax::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl PartialEq<u64> for Snowflake {
    fn eq(&self, u: &u64) -> bool {
        self.get() == *u
    }
}

impl ToArrayString for Snowflake {
    type ArrayString = <u64 as ToArrayString>::ArrayString;
    const MAX_LENGTH: usize = <u64 as ToArrayString>::MAX_LENGTH;

    fn to_arraystring(self) -> Self::ArrayString {
        self.get().to_arraystring()
    }
}

struct SnowflakeVisitor;

impl serde::de::Visitor<'_> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or integer snowflake that is not u64::MAX")
    }

    // Called by formats like TOML.
    fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
        self.visit_u64(u64::try_from(value).map_err(Error::custom)?)
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
        NonMaxU64::new(value)
            .map(Snowflake)
            .ok_or_else(|| Error::custom("invalid value, expected non-max"))
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map(Snowflake).map_err(Error::custom)
    }
}

impl<'de> serde::Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Snowflake, D::Error> {
        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

impl serde::Serialize for Snowflake {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&{ self.0 })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    #[test]
    fn test_snowflake_serde() {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct S {
            id: Snowflake,
        }

        let s = S {
            id: Snowflake::new(17_5928_8472_9911_7063),
        };
        let json = json!({"id": "175928847299117063"});
        assert_eq!(serde_json::to_value(&s).unwrap(), json);
        assert_eq!(serde_json::from_value::<S>(json).unwrap(), s);
    }
}
