use std::ops::Deref;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(
Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Snowflake(#[serde(with = "snowflake_string")] pub u64);

impl Into<u64> for Snowflake {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<String> for Snowflake {
    fn into(self) -> String { self.0.to_string() }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl AsRef<u64> for Snowflake {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl Deref for Snowflake {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl Snowflake {
    pub fn into_option(self) -> Option<Snowflake> {
        if self.0.eq(&0) {
            Option::None
        } else {
            Option::Some(self)
        }
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Snowflake {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|res| Snowflake(res)).map_err(|_| ())
    }
}

pub(crate) mod snowflake_string {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::fmt::{self, Display};

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: From<u64>,
            D: Deserializer<'de>,
    {
        struct SnowflakeVisitor;

        impl<'de> Visitor<'de> for SnowflakeVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a snowflake (either as a string containing a u64, or a u64)"
                )
            }

            fn visit_u64<E>(self, v: u64) -> Result<u64, E>
                where
                    E: DeError,
            {
                Ok(v)
            }

            fn visit_str<E>(self, s: &str) -> Result<u64, E>
                where
                    E: DeError,
            {
                s.parse().map_err(DeError::custom)
            }
        }

        Ok(T::from(deserializer.deserialize_any(SnowflakeVisitor {})?))
    }
}