mod identifier;

#[cfg(test)]
mod tests;

pub use identifier::Identifier;
pub use identifier::InvalidIdentifierError;

#[macro_export]
macro_rules! def_id {
    ($struct_name:ident, $prefix:literal $(| $alt_prefix:literal)* $(, { $generate_hint:tt })?) => {
        /// An id for the corresponding object type.
        ///
        /// This type _typically_ will not allocate and
        /// therefore is usually cheaply clonable.
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        pub struct $struct_name($crate::Identifier);

        impl $struct_name {
            /// The prefix of the id (e.g. `cus` for a `CustomerId`).
            #[inline(always)]
            pub fn prefix(&self) -> &str {
                self.0.prefix()
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn default_prefix() -> &'static str {
                $prefix
            }

            /// The prefix of the id (e.g. `cus` for a `CustomerId`).
            #[allow(dead_code)]
            #[inline(always)]
            pub fn inner(&self) -> &$crate::Identifier {
                &self.0
            }

            /// The valid prefixes of the id type (e.g. [`ch`, `py`\ for a `ChargeId`).
            #[inline(always)]
            pub fn prefixes() -> &'static [&'static str] {
                &[$prefix$(, $alt_prefix)*]
            }

            /// Extracts a string slice containing the entire id.
            #[inline(always)]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            /// Check is provided prefix would be a valid prefix for id's of this type
            pub fn is_valid_prefix(prefix: &str) -> bool {
                prefix == $prefix $( || prefix == $alt_prefix )*
            }

            pub fn is_valid_prefix_bytes(prefix: &[u8]) -> bool {
                prefix == $prefix.as_bytes() $( || prefix == $alt_prefix.as_bytes() )*
            }

            pub fn generate() -> Result<Self, $crate::identifier::InvalidIdentifierError> {
                Ok(Self($crate::Identifier::generate($prefix)?))
            }

            pub fn from_bytes(bytes: &[u8]) -> Result<Self, $crate::identifier::InvalidIdentifierError> {
                let prefix = bytes.split(|&b| b == b'_').next().ok_or($crate::identifier::InvalidIdentifierError)?;

                if !Self::is_valid_prefix_bytes(prefix) {
                    return Err($crate::identifier::InvalidIdentifierError);
                }
                Ok(Self($crate::Identifier::from_bytes(bytes)?))
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = $crate::ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {               
                if !s.starts_with(concat!($prefix, "_")) $(
                    && !s.starts_with(concat!($alt_prefix, "_"))
                )* {
                    Err($crate::ParseIdError {
                        typename: stringify!($struct_name),
                        expected: stringify!(id to start with $prefix $(or $alt_prefix)*),
                    })
                } else {
                    match s.parse() {
                        Ok(id) => Ok($struct_name(id)),
                        Err(_) => Err($crate::ParseIdError {
                            typename: stringify!($struct_name),
                            expected: stringify!(invalid identifier),
                        }),
                    }
                }
            }
        }

        def_id_serde_impls!($struct_name);
    };
    (enum $enum_name:ident { $( $(#[$test:meta])? $variant_name:ident($($variant_type:tt)*) ),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $enum_name {
            $( $(#[$test])* $variant_name($($variant_type)*), )*
        }

        impl $enum_name {
            pub fn as_str(&self) -> &str {
                match *self {
                    $( $enum_name::$variant_name(ref id) => id.as_str(), )*
                }
            }

            #[allow(dead_code)]
            pub fn as_bytes(&self) -> &[u8] {
                match *self {
                    $( $enum_name::$variant_name(ref id) => id.as_bytes(), )*
                }
            }

            #[allow(dead_code)]
            pub fn inner(&self) -> &Identifier {
                match *self {
                    $( $enum_name::$variant_name(ref id) => id.inner(), )*
                }
            }

            pub fn from_bytes(bytes: &[u8]) -> Result<Self, $crate::identifier::InvalidIdentifierError> {
                let prefix = bytes.split(|&b| b == b'_').next().ok_or($crate::identifier::InvalidIdentifierError)?;

                // Check each variant to see if this is a valid prefix for that variant, by using the variant type
                // to check the prefix.
                $(
                    if <$($variant_type)*>::is_valid_prefix_bytes(prefix) {
                        return Ok($enum_name::$variant_name(<$($variant_type)*>::from_bytes(bytes)?));
                    }
                )*

                Err($crate::identifier::InvalidIdentifierError)

            }
        }

        impl PartialEq<str> for $enum_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $enum_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $enum_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::ops::Deref for $enum_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( $enum_name::$variant_name(ref id) => std::fmt::Display::fmt(&id, f), )*
                }
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = $crate::ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let prefix = s.find('_')
                    .map(|i| &s[0..=i])
                    .ok_or_else(|| $crate::ParseIdError {
                        typename: stringify!($enum_name),
                        expected: "id to start with a prefix (as in 'prefix_')"
                    })?;

                match prefix {
                    $(_ if $($variant_type)*::is_valid_prefix(prefix) => {
                        Ok($enum_name::$variant_name(s.parse()?))
                    })*
                    _ => {
                        Err($crate::ParseIdError {
                            typename: stringify!($enum_name),
                            expected: "unknown id prefix",
                        })
                    }
                }
            }
        }

        $(
            impl From<$($variant_type)*> for $enum_name {
                fn from(id: $($variant_type)*) -> Self {
                    $enum_name::$variant_name(id)
                }
            }

            impl PartialEq<$($variant_type)*> for $enum_name {
                fn eq(&self, other: &$($variant_type)* ) -> bool {
                    self.as_str() == other.as_str()
                }
            }
        )*

        def_id_serde_impls!($enum_name);
    };
}

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! def_id_serde_impls {
    ($struct_name:ident) => {
        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                if serializer.is_human_readable() {
                    serializer.serialize_str(self.as_str())
                } else {
                    serializer.serialize_bytes(self.as_bytes())
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                if deserializer.is_human_readable() {
                    let s: String = serde::Deserialize::deserialize(deserializer)?;
                    s.parse::<Self>().map_err(::serde::de::Error::custom)
                } else {
                    let b: &[u8] = serde::Deserialize::deserialize(deserializer)?;
                    Self::from_bytes(b).map_err(::serde::de::Error::custom)
                }
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! def_id_serde_impls {
    ($struct_name:ident) => {};
}


#[derive(Clone, Debug)]
pub struct ParseIdError {
    typename: &'static str,
    expected: &'static str,
}

impl std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid `{}`, {}", self.typename, self.expected)
    }
}

impl std::error::Error for ParseIdError {
    fn description(&self) -> &str {
        "error parsing an id"
    }
}
