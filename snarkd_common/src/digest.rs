use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use prost::{
    bytes::{Buf, BufMut},
    encoding::{skip_field, DecodeContext, WireType},
    DecodeError, Message,
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// `SmallVec` provides us with stack allocation in general cases but will fall back to heap for sizes > 64.
type InnerType = SmallVec<[u8; 64]>;

/// A generic storage for small-size binary blobs, generally digests.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Digest(pub InnerType);

pub type Digest16 = Digest;
pub type Digest32 = Digest;
pub type Digest48 = Digest;
pub type Digest64 = Digest;

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

impl DerefMut for Digest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl Message for Digest {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            prost::encoding::encode_key(1, WireType::LengthDelimited, buf);
            prost::encoding::encode_varint(self.len() as u64, buf);
            buf.put(self.as_slice());
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            prost::encoding::check_wire_type(WireType::LengthDelimited, wire_type)?;
            let len = prost::encoding::decode_varint(buf)?;
            if len > buf.remaining() as u64 {
                return Err(DecodeError::new("buffer underflow"));
            }
            let len = len as usize;

            self.truncate(0);
            self.reserve_exact(len);
            buf.copy_to_slice(unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), len) });

            Ok(())
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }

    fn encoded_len(&self) -> usize {
        prost::encoding::key_len(1)
            + prost::encoding::encoded_len_varint(self.len() as u64)
            + self.len()
    }

    fn clear(&mut self) {
        self.truncate(0);
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for Digest {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Blob(blob) => Ok(Self::from(blob)),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::ToSql for Digest {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Borrowed(
            rusqlite::types::ValueRef::Blob(&self[..]),
        ))
    }
}
