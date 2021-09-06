use nom::{
    bits::{bits, streaming::take},
    combinator::map,
    error::Error as NomError,
    IResult,
    sequence::tuple,
    number::streaming::be_u8
};


#[derive(Clone,Debug,PartialEq,Eq)]
pub struct SpacePacket<'a, T> {
    pub primary_header: PrimaryHeader,
    pub secondary_header: Option<T>,
    pub payload: &'a [u8]
}


#[derive(Clone,Debug,PartialEq,Eq)]
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


fn primary_header_parser(input: &[u8] ) -> IResult<&[u8], (u8, u8, u8, u16, u8, u16, u16)> {
    let version = take(3u8);
    let packet_type = take(1u8);
    let sec_header_flag = take(1u8);
    let app_proc_id = take(11u8);
    let sequence_flags = take(2u8);
    let sequence_count = take(14u8);
    let data_length = take(16u8);

    bits::<_, _, NomError<(&[u8], usize)>, _, _>( tuple((version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length)))(input)
}

pub fn primary_header(input: &[u8] ) -> IResult<&[u8], PrimaryHeader> {
    map(primary_header_parser, |(version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length)| { 
        PrimaryHeader {
            version,
            packet_type,
            sec_header_flag,
            app_proc_id,
            sequence_flags,
            sequence_count,
            data_length
        }
            
    })(input)
}

//make sec_header_parser optional if you just want to parse generic, no-sec-headers spacepackets
fn parse_spacepacket<T>(bytes: &[u8], sec_header_parser: fn(&[u8]) -> IResult<&[u8], T>) -> IResult<&[u8], SpacePacket<T>> {

    let (remaining, pri_header) = primary_header(bytes).expect("failed to parse primary header");

    let sec_header = None;
    dbg!(remaining);
    if (pri_header.sec_header_flag == 1) {
        let (remaining, sec_header) = sec_header_parser(remaining).expect("failed to parse secondary header");

        let sec_header = Some(sec_header);
        dbg!(remaining);
        //its a scope issue preventing the value of the sec header form leaving this scope
    }

    Ok((remaining, SpacePacket::<T> {
        primary_header: pri_header,
        secondary_header: sec_header,
        payload: remaining
    }))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub struct SecondaryHeader {
        /// Packet Version Number - 3 bits
        pub meme: u8,
        pub meme2: u8,
    }

    //this parser style is inspired by the README of https://github.com/rust-bakery/nom-derive 
    fn sec_header_parser(input: &[u8] ) -> IResult<&[u8], SecondaryHeader> {
        let (i, meme) = be_u8(input)?;
        let (i, meme2) = be_u8(i)?;
        Ok((i, SecondaryHeader{ meme, meme2 }))
    }

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
            sequence_count:0,
            data_length: 64
        };
        let (remaining, parsed) = primary_header(raw).expect("failed to parse header");

        assert_eq!(parsed, expected);
        assert_eq!(remaining, &[255, 255])
    }

    #[test]
    fn parse_python_spacepacket_secondary_header() {
      
        let raw = b"\x08\x00\x00\x00\x00\x40\xff\xff\xff";
        
        let expected_p = PrimaryHeader {
            version: 0,
            packet_type: 0,
            sec_header_flag: 1,
            app_proc_id: 0,
            sequence_flags: 0,
            sequence_count:0,
            data_length: 64
        };

         let expected_s = SecondaryHeader {
            meme: 255,
            meme2: 255
        };
        

        let (remaining, parsed) = primary_header(raw).expect("failed to parse header");

        assert_eq!(parsed, expected_p);
        let (remaining, parsed) = sec_header_parser(remaining).expect("failed to parse header");
        assert_eq!(parsed, expected_s);

        assert_eq!(remaining, &[255])
    }

    #[test]
    fn parse_python_spacepacket() {
        //this is the equivalent of an all-zero primary header except for a data length of 64 followed by two bytes set to all 1 as a "payload" 
        let raw = b"\x08\x00\x00\x00\x00\x03\xff\xff\xff";
        let expected = SpacePacket {
            primary_header: PrimaryHeader {
                version: 0,
                packet_type: 0,
                sec_header_flag: 1,
                app_proc_id: 0,
                sequence_flags: 0,
                sequence_count:0,
                data_length: 3
            },
            secondary_header: Some(SecondaryHeader {
                meme: 255,
                meme2: 255
            }),
            payload: &[255]
        };

        let (remaining, parsed) = parse_spacepacket::<SecondaryHeader>(raw, sec_header_parser).expect("failed to parse space packet");

        assert_eq!(parsed, expected);
    }    
}

// \x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery
