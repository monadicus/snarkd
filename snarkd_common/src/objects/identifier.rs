use super::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub field: Field,
    pub length: u8,
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for Identifier {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Blob(blob) => Ok({
                let mut digest = Field::from(blob);
                if digest.is_empty() {
                    return Err(rusqlite::types::FromSqlError::InvalidBlobSize {
                        expected_size: 1,
                        blob_size: digest.len(),
                    });
                }
                let length = digest[digest.len() - 1];
                let new_length = digest.len() - 1;
                digest.truncate(new_length);
                Identifier {
                    field: digest,
                    length,
                }
            }),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::ToSql for Identifier {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let mut out = self.field.clone();
        out.push(self.length);
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Blob(out.0.into_iter().collect()),
        ))
    }
}
