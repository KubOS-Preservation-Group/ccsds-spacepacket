# CCSDS Space Packet Protocol

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