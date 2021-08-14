//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Modifications by:
//  - Adrian Edwards

//! Packet Definition for SpacePacket

use failure::{Error, Fail};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;


#[macro_use]
extern crate derive_builder;

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;


/// Generic Packet trait which defines the internal packet requirements
/// of the communications service.
///DataSegment represents a larger "chunk" of packet data, like a group of header values (such as the PrimaryHeader)
pub trait DataSegment {
    /// Parse packet from raw bytes
    fn from_cursor(reader: Cursor<Vec<u8>>) -> CommsResult<Self>;
    /// Create a bytes representation of the packet
    fn to_bytes(&self) -> CommsResult<Vec<u8>>;
}

#[derive(Eq, Debug, PartialEq)]
struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    version: u8,
    /// Packet Type - 1 bit
    packet_type: u8,
    /// Secondary Header Flag - 1 bit
    sec_header_flag: u8,
    /// Application Process ID - 11 bits
    app_proc_id: u16,
    /// Sequence Flags - 2 bits
    sequence_flags: u8,
    /// Packet Sequence Count or Packet Name - 14 bits
    sequence_count: u16,
    /// Packet Data Length - 2 bytes
    data_length: u16,
}

impl DataSegment for PrimaryHeader {
    fn from_cursor(reader: Cursor<Vec<u8>>) -> CommsResult<Self> {

        let header_0 = reader.read_u16::<BigEndian>()?;
        let version = ((header_0 & 0xE000) >> 13) as u8;
        let packet_type = ((header_0 & 0x1000) >> 12) as u8;
        let sec_header_flag = ((header_0 & 0x800) >> 11) as u8;
        let app_proc_id = (header_0 & 0x7FF) as u16;

        let header_1 = reader.read_u16::<BigEndian>()?;
        let sequence_flags = ((header_1 & 0xC000) >> 14) as u8;
        let sequence_count = (header_1 & 0x3FFF) as u16;

        let data_length = reader.read_u16::<BigEndian>()?;

        Ok(PrimaryHeader {
            version,
            packet_type,
            sec_header_flag,
            app_proc_id,
            sequence_flags,
            sequence_count,
            data_length,
        })
    }

    fn to_bytes(&self) -> CommsResult<Vec<u8>> {
        let mut bytes = vec![];

        let header_0: u16 = (self.app_proc_id) as u16
            | u16::from(self.sec_header_flag) << 11
            | u16::from(self.packet_type) << 12
            | u16::from(self.version) << 13;

        let header_1 = (self.sequence_count as u16)
            | u16::from(self.sequence_flags) << 14;

        let header_2 = self.data_length;

        bytes.write_u16::<BigEndian>(header_0)?;
        bytes.write_u16::<BigEndian>(header_1)?;
        bytes.write_u16::<BigEndian>(header_2)?;
        Ok(bytes)
    }
}

//TODO make this subclassable or something so that projects can define their own custom thing per their own projects spec
#[derive(Eq, Debug, PartialEq)]
pub struct SecondaryHeader {
}

/// Structure used to implement SpacePacket
#[derive(Eq, Debug, PartialEq)]
#[derive(Builder)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    //at least one of these two optionals must exist
    secondary_header: Optional<SecondaryHeader>,
    payload: Optional<Vec<u8>>,
}

impl DataSegment for SpacePacket {

    fn from_cursor(reader: Cursor<Vec<u8>>) -> CommsResult<Self> {

        let primary_header = PrimaryHeader.from_cursor(reader)
        let secondary_header;

        if primary_header.sec_header_flag == 1 {
            //parse secondary header information here
            secondary_header = SecondaryHeader.from_cursor(reader)
        }
    
        let pos = reader.position() as usize;
        let payload = raw[pos..].to_vec();
        Ok(SpacePacket {
            primary_header,
            secondary_header,
            payload,
        })//Ok(Box::new(
    }

    fn to_bytes(&self) -> CommsResult<Vec<u8>> {
        let mut bytes = vec![];

        let primary_header = self.primary_header.to_bytes()
        bytes.append(&mut primary_header.clone());

        if self.primary_header.sec_header_flag == 1 {
            let secondary_header = self.secondary_header.to_bytes()
            bytes.append(&mut secondary_header);//.clone()
        }

        //write secondary header here
        // bytes.write_u64::<BigEndian>(self.secondary_header.command_id)?;
        // bytes.write_u16::<BigEndian>(self.secondary_header.destination_port)?;
        bytes.append(&mut self.payload.clone());

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn do_build_parse() {
        let packet =
            SpacePacket::build(1294, 0, 15001, &[5, 4, 3, 2, 1]).unwrap();
        println!("packet {:?}", packet);

        let raw = packet.to_bytes();
        println!("bytes {:?}", raw);

        let mut reader = Cursor::new(raw.to_vec());

        let parsed = SpacePacket::from_cursor(reader);
        println!("parsed {:?}", parsed);

        assert_eq!(packet, parsed.unwrap());
    }

    #[test]
    fn parse_python_spacepacket() {
        let raw = b"\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery";
        let mut reader = Cursor::new(raw.to_vec());

        let parsed = SpacePacket::from_cursor(reader);
        dbg!(parsed);
    }
}