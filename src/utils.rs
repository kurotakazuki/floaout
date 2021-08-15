use std::fmt::Debug;
use std::io::{Error, ErrorKind, Result};

pub(crate) fn expected_and_found_error<T: Debug>(
    error_kind: ErrorKind,
    expected: T,
    found: T,
) -> Error {
    Error::new(
        error_kind,
        format!("expected `{:?}`, found `{:?}`", expected, found),
    )
}

pub(crate) fn is_equal<T: Debug + Eq>(error_kind: ErrorKind, expected: T, found: T) -> Result<()> {
    if found != expected {
        return Err(expected_and_found_error(error_kind, expected, found));
    }
    Ok(())
}

pub(crate) fn return_invalid_data_if_not_equal<T: Debug + Eq>(val: T, expect: T) -> Result<()> {
    is_equal(ErrorKind::InvalidData, expect, val)
}
