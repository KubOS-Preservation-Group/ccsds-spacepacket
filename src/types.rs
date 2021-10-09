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

/// Macro for generating implementations of [`From<T>`] for [`PacketType`]
/// and [`From<PacketType>`] for `T` for some type `T`.
macro_rules! impl_packet_type_conv {
    ( $($T:ty)+ ) => {
        $(
            impl From<$T> for $crate::types::PacketType {
                fn from(byte: $T) -> $crate::types::PacketType {
                    match byte {
                        0 => $crate::types::PacketType::Data,
                        1 => $crate::types::PacketType::Command,
                        _ => $crate::types::PacketType::Unknown,
                    }
                }
            }

            impl From<$crate::types::PacketType> for $T {
                fn from(packet_type: $crate::types::PacketType) -> $T {
                    match packet_type {
                        $crate::types::PacketType::Data | $crate::types::PacketType::Unknown => 0,
                        $crate::types::PacketType::Command => 1,
                    }
                }
            }
        )+
    };
}

// To add implementations for more types, just add them to the list of types being
// passed to the invocation of impl_packet_type_conv here.
impl_packet_type_conv! { u8 u16 }

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

/// Macro for generating implementations of [`From<T>`] for [`SecondaryHeaderFlag`]
/// and [`From<SecondaryHeaderFlag>`] for `T` for some type `T`.
macro_rules! impl_sec_header_flag_conv {
    ( $($T:ty)+ ) => {
        $(
            impl From<$T> for $crate::types::SecondaryHeaderFlag {
                fn from(byte: $T) -> $crate::types::SecondaryHeaderFlag {
                    match byte {
                        0 => $crate::types::SecondaryHeaderFlag::NotPresent,
                        1 => $crate::types::SecondaryHeaderFlag::Present,
                        _ => $crate::types::SecondaryHeaderFlag::Unknown,
                    }
                }
            }

            impl From<$crate::types::SecondaryHeaderFlag> for $T {
                fn from(packet_type: $crate::types::SecondaryHeaderFlag) -> $T {
                    match packet_type {
                        $crate::types::SecondaryHeaderFlag::NotPresent | $crate::types::SecondaryHeaderFlag::Unknown => 0,
                        $crate::types::SecondaryHeaderFlag::Present => 1,
                    }
                }
            }
        )+
    };
}

// To add implementations for more types, just add them to the list of types being
// passed to the invocation of impl_sec_header_flag_conv here.
impl_sec_header_flag_conv! { u8 u16 }

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
