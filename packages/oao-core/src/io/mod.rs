pub use self::read::ReadBytes;
pub use self::read::ReadExt;

mod read;

pub fn cb_ok(_: &mut [u8]) -> std::io::Result<()> {
    Ok(())
}
