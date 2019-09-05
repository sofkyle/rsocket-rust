extern crate bytes;

use super::{Body, Frame, Writeable};
use crate::result::RSocketResult;
use bytes::{BigEndian, BufMut, ByteOrder, Bytes, BytesMut};

#[derive(Debug)]
pub struct Error {
  code: u32,
  data: Option<Bytes>,
}

pub struct ErrorBuilder {
  stream_id: u32,
  flag: u16,
  value: Error,
}

impl ErrorBuilder {
  fn new(stream_id: u32, flag: u16) -> ErrorBuilder {
    ErrorBuilder {
      stream_id,
      flag,
      value: Error {
        code: 0,
        data: None,
      },
    }
  }

  pub fn set_code(mut self, code: u32) -> Self {
    self.value.code = code;
    self
  }

  pub fn set_data(mut self, data: Bytes) -> Self {
    self.value.data = Some(data);
    self
  }

  pub fn build(self) -> Frame {
    Frame::new(self.stream_id, Body::Error(self.value), self.flag)
  }
}

impl Error {
  pub fn decode(flag: u16, bf: &mut BytesMut) -> RSocketResult<Error> {
    let code = BigEndian::read_u32(bf);
    bf.advance(4);
    let d: Option<Bytes> = if bf.is_empty() {
      None
    } else {
      Some(Bytes::from(bf.to_vec()))
    };
    Ok(Error { code, data: d })
  }

  pub fn builder(stream_id: u32, flag: u16) -> ErrorBuilder {
    ErrorBuilder::new(stream_id, flag)
  }

  pub fn get_data(&self) -> &Option<Bytes> {
    &self.data
  }

  pub fn get_code(&self) -> u32 {
    self.code
  }
}

impl Writeable for Error {
  fn write_to(&self, bf: &mut BytesMut) {
    bf.put_u32_be(self.code);
    match &self.data {
      Some(v) => bf.put(v),
      None => (),
    }
  }

  fn len(&self) -> u32 {
    4 + match &self.data {
      Some(v) => v.len() as u32,
      None => 0,
    }
  }
}
