pub use self::read::{ReadBytes, ReadExt};

mod read;

pub fn cb_ok(_: &mut [u8]) -> std::io::Result<()> {
    Ok(())
}
