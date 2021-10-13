// This file contains type definitions for various fields used within the PrimaryHeader and other parts of the space packet
// It is derived from Apache 2.0-licensed work done by Noah Ryan in https://github.com/nsmryan/ccsds_primary_header/blob/master/src/primary_header.rs


/// The PacketType indicates whether the packet is a command (Command) or a 
/// telemetry (Data) packet.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PacketType {
  /// The packet contains telemetry data.
  Data,
  /// The packet contains a command.
  Command,
  /// The packet type is unknown. This should not occur, but it is included
  /// for encoding an integer into a packet type.
  Unknown
} 

impl Default for PacketType {
    fn default() -> PacketType {
        PacketType::Data
    }
}

impl From<u8> for PacketType {
    fn from(byte: u8) -> PacketType {
        match byte {
            0 => PacketType::Data,
            1 => PacketType::Command,
            _ => PacketType::Unknown
        }
    }
}

impl From<u16> for PacketType {
    fn from(byte: u16) -> PacketType {
        PacketType::from(byte as u8)
    }
}

impl From<PacketType> for u8 {
    fn from(packet_type: PacketType) -> u8 {
        match packet_type { 
            PacketType::Data    => 0,
            PacketType::Command => 1,
            PacketType::Unknown => 0,
        }
    }
}

impl From<PacketType> for u16 {
    fn from(packet_type: PacketType) -> u16 {
        u8::from(packet_type) as u16
    }
}

/// The secondary header flag indicates whether there is another header
/// following the primary header (Present) or not (NotPresent).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SecondaryHeaderFlag {
  /// The secondary header is not present. The bytes following the primary header
  /// is the packet's data section.
  NotPresent,
  /// A secondary header is present in the packet. The secondary header follows the
  /// primary header.
  Present,
  /// The secondary header flag in not valid. This should not occur, but it is included
  /// for turning an integer into a SecondaryHeaderFlag.
  Unknown
} 

impl Default for SecondaryHeaderFlag {
    fn default() -> SecondaryHeaderFlag {
        SecondaryHeaderFlag::NotPresent
    }
}

impl From<u8> for SecondaryHeaderFlag {
    fn from(byte: u8) -> SecondaryHeaderFlag {
        match byte {
            0 => SecondaryHeaderFlag::NotPresent,
            1 => SecondaryHeaderFlag::Present,
            _ => SecondaryHeaderFlag::Unknown
        }
    }
}

impl From<u16> for SecondaryHeaderFlag {
    fn from(byte: u16) -> SecondaryHeaderFlag {
        SecondaryHeaderFlag::from(byte as u8)
    }
}

impl From<SecondaryHeaderFlag> for u8 {
    fn from(flag: SecondaryHeaderFlag) -> u8 {
        match flag {
            SecondaryHeaderFlag::NotPresent => 0,
            SecondaryHeaderFlag::Present    => 1,
            SecondaryHeaderFlag::Unknown    => 0
        }
    }
}

impl From<SecondaryHeaderFlag> for u16 {
    fn from(flag: SecondaryHeaderFlag) -> u16 {
       u8::from(flag) as u16
    }
}

/// The sequence flag indicates the interpretation of the sequence count.
/// Continuation- the sequence count indicates the block in a series of packets
///               containing segmented data
/// FirstSegement- the packet is the first in a series of segemented packets.
/// LastSegement- the packet is the last in a series of segemented packets.
/// Unsegmented- the sequence count is an incrementing counter used to distinguish
///              packets.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SeqFlag {
  /// The packets is a continuation in a series of packets.
  Continuation,
  /// The packets is the first is a series of packets.
  FirstSegment,
  /// The packets is the last is a series of packets.
  LastSegment,
  /// The packets is a standalone packet. Most packets are unsegmented.
  Unsegmented,
  /// The sequence flag is unknown. This should not occur, but it is included
  /// for encoding integers into this type.
  Unknown
}

impl Default for SeqFlag {
    fn default() -> SeqFlag {
        SeqFlag::Unsegmented
    }
}

impl From<u8> for SeqFlag {
    fn from(byte: u8) -> SeqFlag {
        match byte {
            0 => SeqFlag::Continuation,
            1 => SeqFlag::FirstSegment,
            2 => SeqFlag::LastSegment,
            3 => SeqFlag::Unsegmented,
            _ => SeqFlag::Unknown
        }
    }
}

impl From<u16> for SeqFlag {
    fn from(byte: u16) -> SeqFlag {
        SeqFlag::from(byte as u8)
    }
}

impl From<SeqFlag> for u16 {
    fn from(byte: SeqFlag) -> u16 {
        match byte {
            SeqFlag::Continuation => 0,
            SeqFlag::FirstSegment => 1,
            SeqFlag::LastSegment  => 2,
            SeqFlag::Unsegmented  => 3,
            SeqFlag::Unknown      => 0
        }
    }
}