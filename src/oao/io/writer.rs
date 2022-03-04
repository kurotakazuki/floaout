use crate::oao::OaoMetadata;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub struct OaoWriter<W: Write> {
    pub inner: W,
    pub metadata: OaoMetadata,
}

impl<W: Write> OaoWriter<W> {
    pub fn new(mut inner: W, metadata: OaoMetadata) -> Result<Self> {
        metadata.write(&mut inner)?;

        Ok(Self { inner, metadata })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl OaoWriter<BufWriter<File>> {
    pub fn create<P: AsRef<Path>>(filename: P, metadata: OaoMetadata) -> Result<Self> {
        let file = File::create(filename)?;
        let buf_writer = BufWriter::new(file);
        Self::new(buf_writer, metadata)
    }
}
