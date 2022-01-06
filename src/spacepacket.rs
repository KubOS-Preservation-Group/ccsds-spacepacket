// Implementation of the CCSDS spacepacket with an optional secondary header
use crate::PrimaryHeader;
use crate::types;
use deku::prelude::*;
use std::marker::PhantomData;

#[derive(Eq, Debug, PartialEq, Clone)]
pub struct SpacePacket<'a,S: deku::DekuContainerRead<'a> + deku::DekuWrite> {
    pub primary_header: PrimaryHeader,
    pub secondary_header: Option<S>,
    pub payload: Vec<u8>,
    phantom: PhantomData<&'a S>,
}
impl <'a, S: deku::DekuContainerRead<'a> + deku::DekuWrite> SpacePacket<'a,S> {
    pub fn parse(raw: &'a Vec<u8>) -> Self {
        let (rest,primary_header) = PrimaryHeader::from_bytes((raw,0)).expect("failed to parse primary header");
        
        match primary_header.sec_header_flag {
            types::SecondaryHeaderFlag::Present => {
                let (rest,secondary_header) = S::from_bytes((rest.0,0)).expect("failed to parse secondary header");
                Self {
                    primary_header,
                    secondary_header: Some(secondary_header),
                    payload: rest.0.to_vec(),
                    phantom: PhantomData::<&S>,
                }
            }
            _ => Self {
                primary_header,
                secondary_header: None,
                payload: rest.0.to_vec(),
                phantom: PhantomData::<&S>,
            }
        }       
    }
}