use failure::Fail;

use crate::crypto::FromBase58CheckError;
use crate::crypto::NotMatchingPrefixError;

/// Possible errors for base58checked
#[derive(Debug, PartialEq, Fail)]
pub enum KeyFromBase58CheckError {
    /// Base58 error.
    #[fail(display = "invalid base58")]
    InvalidBase58,
    /// The input had invalid checksum.
    #[fail(display = "invalid checksum")]
    InvalidChecksum,
    /// The input is missing checksum.
    #[fail(display = "missing checksum")]
    MissingChecksum,
    /// Provided prefix doesn't match one in base58 string
    #[fail(display = "not matching prefix")]
    NotMatchingPrefix,
    /// Provided prefix doesn't match one in base58 string
    #[fail(display = "invalid key size")]
    InvalidKeySize,
}

impl From<FromBase58CheckError> for KeyFromBase58CheckError {
    fn from(err: FromBase58CheckError) -> Self {
        match err {
            FromBase58CheckError::InvalidBase58 => {
                Self::InvalidBase58
            }
            FromBase58CheckError::InvalidChecksum => {
                Self::InvalidChecksum
            }
            FromBase58CheckError::MissingChecksum => {
                Self::MissingChecksum
            }
        }
    }
}

impl From<NotMatchingPrefixError> for KeyFromBase58CheckError {
    fn from(_: NotMatchingPrefixError) -> Self {
        Self::NotMatchingPrefix
    }
}
