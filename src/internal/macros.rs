//! A set of macros for easily working with internals.

#[cfg(any(feature = "model", feature = "utils"))]
macro_rules! cdn {
    ($e:expr) => {
        concat!("https://cdn.discordapp.com", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(cdn!($e), $($rest)*)
    };
}

/// The macro forwards the generation to the `bitflags::bitflags!` macro and implements the default
/// (de)serialization for Discord's bitmask values.
///
/// The flags are created with `T::from_bits_truncate` for the deserialized integer value.
///
/// Use the `bitflags::bitflags! macro directly if a different serde implementation is required.
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$outer])*
        #[repr(Rust, packed)]
        $vis struct $BitFlags($T);

        bitflags::bitflags! {
            impl $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )*
            }
        }

        bitflags!(__impl_serde $BitFlags: $T);
    };
    (__impl_serde $BitFlags:ident: $T:tt) => {
        impl<'de> serde::de::Deserialize<'de> for $BitFlags {
            fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                Ok(Self::from_bits_truncate(u64::deserialize(deserializer)? as $T))
            }
        }

        impl serde::ser::Serialize for $BitFlags {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                self.bits().serialize(serializer)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::assert_json;

    #[test]
    fn enum_number() {
        enum_number! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
            pub enum T {
                /// AAA
                A = 1,
                /// BBB
                B = 2,
                /// CCC
                C = 3,
                _ => Unknown(u8),
            }
        }

        assert_json(&T::A, json!(1));
        assert_json(&T::B, json!(2));
        assert_json(&T::C, json!(3));
        assert_json(&T::Unknown(123), json!(123));

        assert_eq!(T::A.as_str(), "A");
        assert_eq!(T::B.as_str(), "B");
        assert_eq!(T::C.as_str(), "C");
        assert_eq!(T::Unknown(123).as_str(), "Unknown");
    }
}
