use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    combinator::{map, not, peek},
    multi::{length_data, many0, many0_count},
    number::complete::{be_u32, be_u64},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Target: {:?}", args);
    let mut f = File::open(&args[1])?;
    let mut buffer = [0; 2000];
    f.read(&mut buffer)?;
    match log_file(&buffer) {
        Ok((v, l)) => println!("{:?} {}", l, v.len()),
        Err(e) => println!("{:?}", &e),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct LogFileHdr {
    magic: u32,
    version: u32,
    db_id: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Txn<'a> {
    crc: u64,
    buf: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct LogFile<'a> {
    hdr: LogFileHdr,
    txns: Vec<Txn<'a>>,
}

fn log_file<'a>(input: &'a [u8]) -> IResult<&'a [u8], LogFile> {
    terminated(
        map(pair(header, many0(txn)), |(hdr, txns)| LogFile {
            hdr: hdr,
            txns: txns,
        }),
        many0_count(tag([0])),
    )(input)
}

fn txn<'a>(input: &'a [u8]) -> IResult<&[u8], Txn<'a>> {
    map(
        preceded(
            peek(not(tag(b"0000"))),
            terminated(pair(be_u64, length_data(be_u32)), tag(b"B")),
        ),
        |(crc, result)| Txn {
            crc: crc,
            buf: result,
        },
    )(input)
}

fn header(input: &[u8]) -> IResult<&[u8], LogFileHdr> {
    map(
        tuple((be_u32, be_u32, be_u64)),
        |(magic, version, db_id)| LogFileHdr {
            magic: magic,
            version: version,
            db_id: db_id,
        },
    )(input)
}

#[test]
fn parse_header() {
    use byteorder::{BigEndian, ByteOrder};
    assert_eq!(
        header(b"ZKLG\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00"),
        Ok((
            &b""[..],
            LogFileHdr {
                magic: BigEndian::read_u32(b"ZKLG"),
                version: 2,
                db_id: 0,
            }
        ))
    );
}
