pub fn main() {

}

use nom::{
  IResult,
  sequence::tuple,
  number::streaming::{be_u32, be_u64}
};

#[derive(Debug,PartialEq,Eq)]
pub struct LogFileHdr {
  magic: u32,
  version: u32,
  db_id: u64
}


pub fn header(input:&[u8]) -> IResult<&[u8], LogFileHdr> {
  let (input, (magic, version, db_id)) = tuple((be_u32, be_u32, be_u64))(input)?;
  Ok((input, LogFileHdr {
    magic: magic,
    version: version,
    db_id: db_id
  }))
}


#[test]
fn parse_header() {
  use byteorder::{BigEndian, ByteOrder};
  assert_eq!(header(b"ZKLG\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], LogFileHdr {
    magic: BigEndian::read_u32(b"ZKLG"),
    version: 2,
    db_id: 0,
  })));
}
