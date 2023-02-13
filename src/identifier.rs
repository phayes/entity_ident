use lazy_static::lazy_static;
use regex::bytes::Regex;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([a-zA-Z0-9]{1,8})_([a-zA-Z0-9]{1,22})$").unwrap();
    static ref IDENT_REGEX: Regex = Regex::new(r"^([a-zA-Z0-9]{1,8})$").unwrap();
}

pub struct InvalidIdentifierError;

// Representation:
// First Byte: Length of the identifier

// It is UNSAFE to put anything other than valid ASCII in the identifier
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Identifier {
    bytes: [u8; 32],
}

impl Identifier {
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes[0] as usize
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        // SAFETY: Identifier cannot be constructed from invalid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.as_bytes()) }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[1..=self.len()]
    }

    #[inline]
    pub fn from_str(s: &str) -> Result<Self, InvalidIdentifierError> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidIdentifierError> {
        let len = bytes.len();
        if len > 31 {
            return Err(InvalidIdentifierError);
        }

        // Use Regex to check the format
        if !IDENTIFIER_REGEX.is_match(bytes) {
            return Err(InvalidIdentifierError);
        }

        let mut id_bytes = [0; 32];
        id_bytes[0] = len as u8;
        id_bytes[1..=len].copy_from_slice(&bytes);
        Ok(Self { bytes: id_bytes })
    }

    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let len = bytes.len();
        debug_assert!(len <= 31);
        debug_assert!(IDENTIFIER_REGEX.is_match(&bytes));

        let mut id_bytes = [0; 32];
        id_bytes[0] = len as u8;
        id_bytes[1..=len].copy_from_slice(&bytes);
        Self { bytes: id_bytes }
    }

    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        Self::from_bytes_unchecked(s.as_bytes())
    }

    pub fn ident(&self) -> &str {
        let bytes = self.as_bytes();

        let caps = IDENTIFIER_REGEX.captures(bytes).unwrap();
        let ident_bytes = caps.get(1).unwrap().as_bytes();
        unsafe { std::str::from_utf8_unchecked(ident_bytes) }
    }

    pub fn generate(ident: &str) -> Result<Self, InvalidIdentifierError> {
        if !IDENT_REGEX.is_match(ident.as_bytes()) {
            return Err(InvalidIdentifierError);
        }

        let mut rand_bytes: [u8; 16] = [0; 16];
        getrandom::getrandom(&mut rand_bytes).unwrap();
    
        let num: u128 = u128::from_be_bytes(rand_bytes);

        // TODO: Encode to bytes and avoid this allocation
        let encoded_num = base62::encode(num);

        let mut bytes = [0; 32];

        // Zero-index records the size
        let len = (ident.len() + 1 + encoded_num.len()) as u8;
        debug_assert!(len <= 31);
        bytes[0] = len;

        bytes[1..=ident.len()].copy_from_slice(ident.as_bytes());
        bytes[ident.len() + 1] = b'_';
        bytes[ident.len() + 2..=len as usize].copy_from_slice(encoded_num.as_bytes());

        debug_assert!(IDENTIFIER_REGEX.is_match(&bytes[1..(bytes[0]+1) as usize]));

        Ok(Self { bytes })
    }

}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq<str> for Identifier {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<String> for Identifier {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialOrd for Identifier {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for Identifier {
    type Error = InvalidIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Identifier {
    type Error = InvalidIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl TryFrom<&[u8]> for Identifier {
    type Error = InvalidIdentifierError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl Into<Vec<u8>> for Identifier {
    fn into(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl std::str::FromStr for Identifier {
    type Err = InvalidIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}