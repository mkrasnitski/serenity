#[macro_export]
macro_rules! api {
    ($e:expr) => {
        concat!("https://discord.com/api/v10", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

/// The `enum_number!` macro generates `From` implementations to convert between values and the
/// enum which can then be utilized by `serde` with `#[serde(from = "u8", into = "u8")]`.
///
/// When defining the enum like this:
/// ```ignore
/// enum_number! {
///     /// The `Foo` enum
///     #[derive(Clone, Copy, Deserialize, Serialize)]
///     #[serde(from = "u8", into = "u8")]
///     pub enum Foo {
///         /// First
///         Aah = 1,
///         /// Second
///         Bar = 2,
///         _ => Unknown(u8),
///     }
/// }
/// ```
///
/// Code like this will be generated:
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// #
/// /// The `Foo` enum
/// #[derive(Clone, Copy, Deserialize, Serialize)]
/// #[serde(from = "u8", into = "u8")]
/// pub enum Foo {
///     /// First
///     Aah,
///     /// Second,
///     Bar,
///     /// Variant value is unknown.
///     Unknown(u8),
/// }
///
/// impl From<u8> for Foo {
///     fn from(value: u8) -> Self {
///         match value {
///             1 => Self::Aah,
///             2 => Self::Bar,
///             unknown => Self::Unknown(unknown),
///         }
///     }
/// }
///
/// impl From<Foo> for u8 {
///     fn from(value: Foo) -> Self {
///         match value {
///             Foo::Aah => 1,
///             Foo::Bar => 2,
///             Foo::Unknown(unknown) => unknown,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! enum_number {
    (
        $(#[$outer:meta])*
        $(#[<default> = $default:literal])?
        $vis:vis enum $Enum:ident {
            $(
                $(#[doc = $doc:literal])*
                $(#[cfg $($cfg:tt)*])?
                $Variant:ident = $value:literal,
            )*
            _ => Unknown($T:ty),
        }
    ) => {
        $(#[$outer])*
        $vis struct $Enum (pub $T);

        $(
            impl Default for $Enum {
                fn default() -> Self {
                    Self($default)
                }
            }
        )?

        #[allow(non_snake_case, non_upper_case_globals)]
        #[allow(clippy::allow_attributes, reason = "Does not always trigger due to macro")]
        impl $Enum {
            $(
                $(#[doc = $doc])*
                $(#[cfg $($cfg)*])?
                $vis const $Variant: Self = Self($value);
            )*

            /// Variant value is unknown.
            #[must_use]
            $vis const fn Unknown(val: $T) -> Self {
                Self(val)
            }

            /// Get a string representation
            #[must_use]
            $vis const fn as_str(&self) -> &'static str {
                match &self.0 {
                    $(
                        $value => stringify!($Variant),
                    )*
                    _ => "Unknown",
                }
            }
        }
    };
}
