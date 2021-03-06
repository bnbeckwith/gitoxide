use git_object::{owned, HashKind};
use std::io;

pub trait Write {
    type Error: std::error::Error + From<io::Error>;

    /// Write `object` using the given kind of `hash` into the database, returning id to reference it in subsequent reads.
    fn write(&self, object: &owned::Object, hash: HashKind) -> Result<owned::Id, Self::Error> {
        let mut buf = Vec::with_capacity(2048);
        object.write_to(&mut buf)?;
        self.write_stream(object.kind(), buf.len() as u64, buf.as_slice(), hash)
    }
    /// As `write`, but takes an `object` kind along with its encoded bytes.
    fn write_buf(&self, object: git_object::Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
        self.write_stream(object, from.len() as u64, from, hash)
    }
    /// As `write`, but takes an input stream. This is commonly used for writing blobs directly without reading them to memory first.
    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        from: impl io::Read,
        hash: HashKind,
    ) -> Result<owned::Id, Self::Error>;
}
