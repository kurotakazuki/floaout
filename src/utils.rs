use std::io::{Error, ErrorKind, Result};

pub fn return_invalid_data_if_not_equal<T: std::fmt::Debug + Eq>(val: T, expect: T) -> Result<()> {
    if val != expect {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("expected `{:?}`, found `{:?}`", expect, val),
        ));
    }
    Ok(())
}
