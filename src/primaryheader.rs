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

//! Packet Definition for SpacePacket

// use crate::packet::{LinkPacket, PayloadType};
// use crate::CommsResult;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use crate::ParseResult;


#[derive(Eq, Debug, PartialEq, Clone)]
pub struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    pub version: u8,
    /// Packet Type - 1 bit
    pub packet_type: u8,
    /// Secondary Header Flag - 1 bit
    pub sec_header_flag: u8,
    /// Application Process ID - 11 bits
    pub app_proc_id: u16,
    /// Sequence Flags - 2 bits
    pub sequence_flags: u8,
    /// Packet Sequence Count or Packet Name - 14 bits
    pub sequence_count: u16,
    /// Packet Data Length - 2 bytes
    pub data_length: u16,
}


impl PrimaryHeader {

    fn parse(raw: &[u8]) -> ParseResult<PrimaryHeader> {
        let mut reader = Cursor::new(raw.to_vec());

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

    fn to_bytes(&self) -> ParseResult<Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::primaryheader::PrimaryHeader;

    #[test]
    fn parse_python_spacepacket_primary_header() {
        //this is the equivalent of an all-zero primary header except for a data length of 64 followed by two bytes set to all 1 as a "payload"
        let raw = b"\x00\x00\x00\x00\x00\x40\xff\xff";
        let expected = PrimaryHeader {
            version: 0,
            packet_type: 0,
            sec_header_flag: 0,
            app_proc_id: 0,
            sequence_flags: 0,
            sequence_count: 0,
            data_length: 64,
        };
        let parsed = PrimaryHeader::parse(raw).expect("failed to parse header");

        assert_eq!(parsed, expected);
    }
}
