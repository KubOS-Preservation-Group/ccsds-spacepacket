// This file contains type definitions for various fields used within the PrimaryHeader and other parts of the space packet
// It is derived from Apache 2.0-licensed work done by Noah Ryan in https://github.com/nsmryan/ccsds_primary_header/blob/master/src/primary_header.rs

use deku::prelude::*;

/// The PacketType indicates whether the packet is a command (Command) or a
/// telemetry (Data) packet.
#[derive(Debug, PartialEq, Eq, Copy, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum PacketType {
    /// The packet contains telemetry data.
    Data = 0,
    /// The packet contains a command.
    Command = 1,
}

impl Default for PacketType {
    fn default() -> PacketType {
        PacketType::Data
    }
}

/// The secondary header flag indicates whether there is another header
/// following the primary header (Present) or not (NotPresent).
#[derive(Debug, PartialEq, Eq, Copy, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum SecondaryHeaderFlag {
    /// The secondary header is not present. The bytes following the primary header
    /// is the packet's data section.
    NotPresent = 0,
    /// A secondary header is present in the packet. The secondary header follows the
    /// primary header.
    Present = 1,
}

/// The sequence flag indicates the interpretation of the sequence count.
/// Continuation- the sequence count indicates the block in a series of packets
///               containing segmented data
/// FirstSegement- the packet is the first in a series of segemented packets.
/// LastSegement- the packet is the last in a series of segemented packets.
/// Unsegmented- the sequence count is an incrementing counter used to distinguish
///              packets.
#[derive(Debug, PartialEq, Eq, Copy, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum SeqFlag {
    /// The packets is a continuation in a series of packets.
    Continuation = 0,
    /// The packets is the first is a series of packets.
    FirstSegment = 1,
    /// The packets is the last is a series of packets.
    LastSegment = 2,
    /// The packets is a standalone packet. Most packets are unsegmented.
    Unsegmented = 3,
}

impl Default for SeqFlag {
    fn default() -> SeqFlag {
        SeqFlag::Unsegmented
    }
}
