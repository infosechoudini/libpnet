use pnet_base::MacAddr;
use std::convert::TryInto;
use std::error::Error;
use crate::native::parser::*;

#[derive(Copy, Clone, Debug, PartialEq , Eq, PartialOrd, Ord, Hash)]
pub struct Ethernet<'a> {
    pub destination: &'a [u8],
    pub source: &'a [u8],
    pub ether_type: &'a [u8],
    pub payload: &'a [u8]
}

impl <'a> Default for Ethernet<'a>{
    fn default() -> Self {
        Ethernet {
            destination: &[0; 6],
            source: &[0; 6],
            ether_type: &[0; 2],
            payload: &[0; 0]
        }
    }
}

impl<'a> Ethernet <'a> {
    #[inline(always)]
    pub fn new(i: &'a [u8]) -> Result<Ethernet<'a>, &dyn Error> {
        let destination: &'a [u8] = &i[0..6];
        let source: &'a [u8] = &i[6..12];
        let ether_type: &'a [u8] = &i[12..14];
        let payload: &'a [u8] = &i[14..];
        
        Ok(Ethernet {
            destination: &destination,
            source: &source,
            ether_type: &ether_type,
            payload: &payload,
        })
    }

    pub fn get_destination(&self) -> MacAddr {
        MacAddr::new(self.destination[0], self.destination[1], self.destination[2], self.destination[3], self.destination[4], self.destination[5])
    }

    pub fn get_source(&self) -> MacAddr {
        MacAddr::new(self.source[0], self.source[1], self.source[2], self.source[3], self.source[4], self.source[5])
    }

    pub fn get_ether_type(&self) -> u16 {
        u16::from_be_bytes(self.ether_type.try_into().unwrap())
    }

    pub fn get_payload(&self) -> &[u8] {
        self.payload
    }
}