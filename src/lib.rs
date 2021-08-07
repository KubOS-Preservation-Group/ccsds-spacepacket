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

#[derive(Eq, Debug, PartialEq)]
struct SecondaryHeader {
    /// Command ID from MT - 64 bits
    command_id: u64,
    /// Destination service port - 16 bits
    destination_port: u16,
}

/// Structure used to implement SpacePacket version of LinkPacket
#[derive(Eq, Debug, PartialEq)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    secondary_header: SecondaryHeader,
    payload: Vec<u8>,
}


pub fn hello() {
    println!("hello")
}