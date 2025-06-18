use serde::{de::DeserializeOwned, Serialize};

pub trait Message: DeserializeOwned + Sized + Serialize  {
    fn into_writer<W: std::io::Write>(&self, writer: W) -> Result<(), ciborium::ser::Error<std::io::Error>> {
        ciborium::into_writer(self, writer)
    }
    fn into_vec_u8(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buf: Vec<u8> = Vec::new();
        self.into_writer(&mut buf)?;
        Ok(buf)
    }
    fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(reader)
    }
}