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

use failure::{Error};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;


/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;

///DataSegment represents a larger "chunk" of packet data, like a group of header values (such as the PrimaryHeader) and defines methods for reading and writing from bytes.
pub trait DataSegment {
    /// Parse packet from raw bytes
    fn from_cursor(reader: Cursor<Vec<u8>>) -> CommsResult<Self>  where Self: std::marker::Sized;
    /// Create a bytes representation of the packet
    fn to_bytes(&self) -> CommsResult<Vec<u8>>;
    ///the number of octets
    fn length(&self) -> u16;
}

#[derive(Eq, Debug, PartialEq)]
#[derive(Clone)]
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
    fn from_cursor(reader: Cursor<Vec<u8>>) -> CommsResult<Self> where Self: std::marker::Sized {

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
    fn length(&self) -> u16 {
        //3+1+1+11+2+14+16 bits for the primary header
        //totals to 6 octets
        6
    }
}

/// Structure used to implement SpacePacket
#[derive(Eq, Debug, PartialEq)]
pub struct SpacePacket<D> {
    primary_header: PrimaryHeader,
    //at least one of these two optionals must exist
    secondary_header: Option<D>,
    payload: Option<Vec<u8>>,
}

impl<D: DataSegment> SpacePacket<D> {

    fn from_bytes(raw: &[u8]) -> CommsResult<Self> where Self: std::marker::Sized {

        let mut reader = Cursor::new(raw.to_vec());


        let primary_header = PrimaryHeader::from_cursor(reader);
        let secondary_header;

        if primary_header.sec_header_flag == 1 {
            //parse secondary header information here
            secondary_header = D::from_cursor(reader);
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

        let primary_header = self.primary_header.to_bytes();
        bytes.append(&mut primary_header);

        if self.primary_header.sec_header_flag == 1 {
            let secondary_header = self.secondary_header.to_bytes();
            bytes.append(&mut secondary_header);//.clone()
        }

        bytes.append(&mut self.payload.clone());

        Ok(bytes)
    }

    fn length(&self) -> u16 {
        self.primary_header.length() + self.primary_header.data_length - 1
    }
}

#[derive(Clone, Default)]
struct SpacePacketBuilder<D> {
    primary_header: Option<PrimaryHeader>,
    //at least one of these two optionals must exist
    secondary_header: Option<D>,
    payload: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl<D: DataSegment> SpacePacketBuilder<D> {
    pub fn default() -> &'static mut Self {
        let mut new = SpacePacketBuilder{
            primary_header: PrimaryHeader{
                version: 0,
                packet_type: 0,
                sec_header_flag: 0,
                app_proc_id: 0,
                sequence_flags: 0,
                sequence_count: 0,
                data_length: 1
            },
            sec_header: None,
            payload: &[0]
        };
        &mut new
    }

    pub fn with_primary_header(&mut self, packet_type: u8, app_proc_id: u16, sequence_flags: u8, packet_name: u16, data_length: u16) -> &'static mut Self {
        self.primary_header.packet_type = packet_type;
        self.primary_header.app_proc_id = app_proc_id;
        self.primary_header.sequence_flags = sequence_flags;
        self.primary_header.sequence_count = packet_name;
        self.primary_header.data_length = data_length;

        self
    }


    pub fn with_secondary_header(&mut self, sec_header: D) -> &'static mut Self {
        self.primary_header.sec_header_flag = 1;
        self.secondary_header = sec_header;
        self.primary_header.data_length += self.secondary_header.length();

        self
    }

    pub fn with_payload(&mut self, payload: Vec<u8>) -> &'static mut Self {
        // we are replacing the default payload so subtract its length
        self.primary_header.data_length -= self.payload.len();
        self.payload = payload;
        // add the length of the new payload to the total
        self.primary_header.data_length += self.payload.len();

        self
    }

    fn build(&self) -> Result<SpacePacket<D>, String> {

        let sec_header_present = self.secondary_header != None && self.primary_header.sec_header_flag == 1;

        let payload_present = self.payload != None;

        let has_data = sec_header_present || payload_present;

        has_data.ok_or("a secondary header and/or payload is required");

        Ok(SpacePacket {
            primary_header: Clone::clone(self.primary_header
                .as_ref()
                .ok_or("primary header must be initialized")?),
            secondary_header: Clone::clone(self.secondary_header),
            payload: Clone::clone(self.payload)
        });
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

        let parsed = SpacePacket::from_bytes(raw);
        println!("parsed {:?}", parsed);

        assert_eq!(packet, parsed.unwrap());
    }

    #[test]
    fn parse_python_spacepacket() {
        let raw = b"\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery";
    
        let parsed = SpacePacket::from_bytes(raw);
        dbg!(parsed);
    }
}