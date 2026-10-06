//! Stable wire names for closed enums.
//!
//! Every enum stored in SQLite, written to the ledger or sent over the CLI
//! envelope uses one lowercase `snake_case` name per variant. `as_str`,
//! `FromStr`, `Display` and serde all derive from the same table so the four
//! can never disagree (tested in each module).

use thiserror::Error;

/// At most this many bytes of rejected input appear in an error message.
pub(crate) const SHOWN_BYTES: usize = 64;

/// Rejected input as an error shows it: escaped, and cut at
/// [`SHOWN_BYTES`] with the full length noted, so hostile input cannot
/// inflate error messages, logs or responses.
pub(crate) fn bounded(value: &str) -> String {
    let mut out = String::new();
    for c in value.escape_debug() {
        if out.len() + c.len_utf8() > SHOWN_BYTES {
            out.push_str(&format!("… ({} bytes)", value.len()));
            return out;
        }
        out.push(c);
    }
    out
}

/// A wire string that names no variant of the expected enum.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unknown {type_name} value \"{value}\"")]
pub struct UnknownVariant {
    pub type_name: &'static str,
    /// The rejected input, bounded and escaped.
    pub value: String,
}

impl UnknownVariant {
    pub fn new(type_name: &'static str, value: &str) -> UnknownVariant {
        UnknownVariant {
            type_name,
            value: bounded(value),
        }
    }
}

/// Defines a closed enum with stable wire names: `as_str`, `FromStr`,
/// `Display`, serde and an `ALL` list, all from one table. Exported so every
/// kernel crate defines its wire enums the same way (the using crate needs
/// `serde` as a dependency).
#[macro_export]
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident => $wire:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $( $(#[$vmeta])* $variant ),+
        }

        impl $name {
            /// Every variant, in declaration order.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The stable wire name.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $wire),+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::UnknownVariant;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($wire => Ok($name::$variant),)+
                    _ => Err($crate::UnknownVariant::new(stringify!($name), s)),
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = <::std::borrow::Cow<'de, str> as ::serde::Deserialize>::deserialize(d)?;
                s.parse().map_err(::serde::de::Error::custom)
            }
        }
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use std::fmt::Display;
    use std::str::FromStr;

    /// Asserts that `as_str`, `Display`, `FromStr` and serde agree for every variant.
    pub(crate) fn assert_wire_roundtrip<T>(all: &[T], as_str: fn(T) -> &'static str)
    where
        T: Copy
            + PartialEq
            + std::fmt::Debug
            + Display
            + FromStr
            + serde::Serialize
            + serde::de::DeserializeOwned,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut seen = std::collections::HashSet::new();
        for &v in all {
            let wire = as_str(v);
            assert!(seen.insert(wire), "duplicate wire name {wire}");
            assert!(
                wire.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "wire name {wire} is not lowercase snake_case"
            );
            assert_eq!(v.to_string(), wire);
            assert_eq!(wire.parse::<T>().unwrap(), v);
            let json = serde_json::to_string(&v).unwrap();
            assert_eq!(json, format!("\"{wire}\""));
            assert_eq!(serde_json::from_str::<T>(&json).unwrap(), v);
        }
    }
}
