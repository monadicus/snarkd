use std::{fmt, ops::Deref};

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// `SmallVec` provides us with stack allocation in general cases but will fall back to heap for sizes > 64.
type InnerType = SmallVec<[u8; 64]>;

/// A generic storage for small-size binary blobs, generally digests.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Digest(pub InnerType);

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..]))
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl Serialize for Digest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;

        Ok(Self(InnerType::from(
            hex::decode(&name).map_err(serde::de::Error::custom)?,
        )))
    }
}

impl Deref for Digest {
    type Target = SmallVec<[u8; 64]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(clippy::from_over_into)]
impl Into<InnerType> for Digest {
    fn into(self) -> InnerType {
        self.0
    }
}

impl<'a> From<&'a [u8]> for Digest {
    fn from(other: &'a [u8]) -> Self {
        Self(other.into())
    }
}

impl From<InnerType> for Digest {
    fn from(other: InnerType) -> Self {
        Self(other)
    }
}

impl<const N: usize> From<[u8; N]> for Digest {
    fn from(other: [u8; N]) -> Self {
        Self(other[..].into())
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self[..]
    }
}

impl Digest {
    pub fn bytes<const N: usize>(&self) -> Option<[u8; N]> {
        if self.len() == N {
            let mut out = [0u8; N];
            out.copy_from_slice(&self[..]);
            Some(out)
        } else {
            None
        }
    }
}
