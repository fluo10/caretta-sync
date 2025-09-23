use rusqlite::{types::FromSql, Error, ToSql};

use crate::{Double, Single, Triple};

impl FromSql for Single {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = u16::column_result(value)?;
        Self::try_from(int).or_else(|e| {
            Err(rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}

impl ToSql for Single {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(u16::from(*self).into())
    }
}

impl FromSql for Double {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = u32::column_result(value)?;
        Self::try_from(int).or_else(|e| {
            Err(rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}

impl ToSql for Double {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(u32::from(*self).into())
    }
}

impl FromSql for Triple {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = u64::column_result(value)?;
        Self::try_from(int).or_else(|e| {
            Err(rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}

impl ToSql for Triple {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            i64::try_from(u64::from(*self)).map_err(
                |err| Error::ToSqlConversionFailure(err.into())
            )?
        )))
    }
}