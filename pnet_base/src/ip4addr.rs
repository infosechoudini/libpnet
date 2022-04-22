// Replaced std with core to support no_std implementation
use core::{
    str::FromStr,
    fmt,
};

// Import of Error from std crate to support Trait for ParseMacAddrErr
#[cfg(feature = "std")]
use std::error::Error;


/// The number of bytes in an ethernet (Ipv4) address.
pub const IPV4_ADDR_LEN: usize = 4;

/// Structure of a Ipv4 Address.
type IPAddrv4 = [u8; IPV4_ADDR_LEN];


/// A MAC address.
#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd)]
pub struct Ipv4Addr(pub u8, pub u8, pub u8, pub u8);


impl Ipv4Addr {
    /// Construct a new `Ipv4Addr` instance.
    #[inline]
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr(a, b, c, d)
    }

    /// Construct an all-zero `Ipv4Addr` instance.
    #[inline]
    pub fn zero() -> Ipv4Addr {
        Default::default()
    }

    /// Returns the 4 eight-bit integers that make up this address
    pub fn octets(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn to_strint(&self) -> String {
        let octets = self.octets();
        format!("{:?}.{:?}.{:?}.{:?}", octets[0], octets[1], octets[2], octets[3]).to_string()
    }
}

impl From<IPAddrv4> for Ipv4Addr {
    #[inline(always)]
    fn from(addr: IPAddrv4) -> Ipv4Addr {
        Ipv4Addr(addr[0], addr[1], addr[2], addr[3])
    }
}

impl From<Ipv4Addr> for IPAddrv4 {
    #[inline(always)]
    fn from(addr: Ipv4Addr) -> Self {
        [addr.0, addr.1, addr.2, addr.3]
    }
}

impl PartialEq<IPAddrv4> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &IPAddrv4) -> bool {
        *self == Ipv4Addr::from(*other)
    }
}

impl core::fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            fmt,
            "{:?}.{:?}.{:?}.{:?}",
            self.0, self.1, self.2, self.3
        )
    }
}

impl core::fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self, fmt)
    }
}


/// Represents an error which occurred whilst parsing a MAC address.
#[derive( Copy,  PartialEq,  Eq,  Clone, fmt::Debug)]
pub enum ParseIpv4AddrErr {
    /// The IP address has too many components, eg. 192.168.1.1.1
    TooManyComponents,
    /// The IP address has too few components, eg. 192.168
    TooFewComponents,
    /// One of the components contains an invalid value, eg. 192.168.2x.22
    InvalidComponent,
}

// Error trait is only support via std 
#[cfg(feature = "std")]
impl Error for ParseIpv4AddrErr {}


impl ParseIpv4AddrErr {
    fn description(&self) -> &str {
        match *self {
            ParseIpv4AddrErr::TooManyComponents => "Too many components in a MAC address string",
            ParseIpv4AddrErr::TooFewComponents => "Too few components in a MAC address string",
            ParseIpv4AddrErr::InvalidComponent => "Invalid component in a MAC address string",
        }
    }
}

impl fmt::Display for ParseIpv4AddrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


impl FromStr for Ipv4Addr {
    type Err = ParseIpv4AddrErr;
    #[inline]
    fn from_str(s: &str) -> Result<Ipv4Addr, ParseIpv4AddrErr> {
        let mut parts = [0u8; 4];
        let splits = s.split('.');
        let mut i = 0;
        for split in splits {
            if i == 6 {
                return Err(ParseIpv4AddrErr::TooManyComponents);
            }
            match u8::from_str_radix(split, 16) {
                Ok(b) if split.len() != 0 => parts[i] = b,
                _ => return Err(ParseIpv4AddrErr::InvalidComponent),
            }
            i += 1;
        }

        if i == 4 {
            Ok(Ipv4Addr(
                parts[0], parts[1], parts[2], parts[3]
            ))
        } else {
            Err(ParseIpv4AddrErr::TooFewComponents)
        }
    }
}