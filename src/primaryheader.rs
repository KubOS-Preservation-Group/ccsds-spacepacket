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
use crate::types;
use deku::prelude::*;

#[derive(Eq, Debug, PartialEq, Clone, DekuRead, DekuWrite)]
pub struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    #[deku(bits = "3")]
    pub version: u8,

    /// Packet Type - 1 bit
    pub packet_type: types::PacketType,

    /// Secondary Header Flag - 1 bit
    pub sec_header_flag: types::SecondaryHeaderFlag,

    /// Application Process ID - 11 bits
    #[deku(bits = "11", endian = "big")]
    pub app_proc_id: u16,

    /// Sequence Flags - 2 bits
    pub sequence_flags: types::SeqFlag,

    /// Packet Sequence Count or Packet Name - 14 bits
    #[deku(bits = "14", endian = "big")]
    pub sequence_count: u16,

    /// Packet Data Length - 2 bytes
    #[deku(endian = "big")]
    pub data_length: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_python_spacepacket_primary_header() {
        //this is the equivalent of an all-zero primary header except for a data length of 64 followed by two bytes set to all 1 as a "payload"
        let raw = b"\x00\x00\xc0\x00\x00\x40\xff\xff";
        let expected = PrimaryHeader {
            version: 0,
            packet_type: types::PacketType::Data,
            sec_header_flag: types::SecondaryHeaderFlag::NotPresent,
            app_proc_id: 0,
            sequence_flags: types::SeqFlag::Unsegmented,
            sequence_count: 0,
            data_length: 64,
        };
        let (rest, parsed) = PrimaryHeader::from_bytes((raw, 0)).expect("failed to parse header");

        assert_eq!(parsed, expected);
        assert_eq!(rest.0, [255, 255])
    }
}
