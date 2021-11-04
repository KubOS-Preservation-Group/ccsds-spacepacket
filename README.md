# CCSDS Space Packet Protocol

![Crates.io](https://img.shields.io/crates/v/ccsds-spacepacket) ![Crates.io (recent)](https://img.shields.io/crates/dr/ccsds-spacepacket)

This is a rust implementation of the [CCSDS Space Packet Protocol](https://public.ccsds.org/Pubs/133x0b2e1.pdf).

## About the Protocol 
Space Packet Protocol is a standardized packet protocol for use in aerospace applications that was developed by a consortium of major space agencies including NASA, JAXA, ESA, and others.

Space Packet Protocol defines three major segments, a Primary Header, a Secondary Header, and a Payload.

The Primary Header is set by the standard and includes things fields for versioning, identifiers, sequence counters, flags, and the data length of the packet.

The Secondary Header is left as a space where an organisation can specify their own custom header values before passing the final spec on to contractors or projects for implementation. 

The Payload is also left unspecified and can contain pretty much any data, such as files or other packets.

The Secondary Header and Payload make up the "User Data Field" and at least one of these two components must be present in all Space Packets. 


## What this library does
This library attempts to implement a general-purpose parser for Space Packets that can interperet both the generic aspects of the space packet protocol (i.e. the Primary Header) in addition to any custom fields supplied within the Secondary Headers.

This Secondary Header parsing is accomplished by allowing users of the library to pass in a parser that can interperet the Secondary Header as specified by their project or organisation.


## Current status
Currently this library just implements Primary Header parsing, but expanding it to be able to deal with a complete, generic spacepacket (think of generics in programming) with user defined custom secondary headers is one of the projects main goals.


## Usage

```rust
	// say you have some bytes you want to turn into a PrimaryHeader
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

	// do the parsing and save the parsed header and any remaining bytes
	let (rest, parsed) = PrimaryHeader::from_bytes((raw, 0)).expect("failed to parse header");

	assert_eq!(parsed, expected);
	assert_eq!(rest.0, [255,255])
```