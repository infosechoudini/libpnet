// Replaced std with core to support no_std implementation
use core::{
    str::FromStr,
    fmt,
};

// Import of Error from std crate to support Trait for ParseMacAddrErr
#[cfg(feature = "std")]
use std::error::Error;



/// The number of bytes in an ethernet (Ipv4) address.
pub const IPV6_ADDR_LEN: usize = 8;

/// Structure of a Ipv4 Address.
type IPAddrv6 = [u16; IPV6_ADDR_LEN];


/// A MAC address.
#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd)]
pub struct Ipv6Addr(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);


impl Ipv6Addr {
    /// Construct a new `Ipv6Addr` instance.
    #[inline]
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        Ipv6Addr(a, b, c, d, e, f, g, h)
    }

    /// Construct an all-zero `Ipv6Addr` instance.
    #[inline]
    pub fn zero() -> Ipv6Addr {
        Default::default()
    }

    /// Returns the 8 sixteen-bit integers that make up this address
    pub fn segments(&self) -> [u16; 8] {
        [self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7]
    }
}

impl From<IPAddrv6> for Ipv6Addr {
    #[inline(always)]
    fn from(addr: IPAddrv6) -> Ipv6Addr {
        Ipv6Addr(addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6], addr[7])
    }
}

impl From<Ipv6Addr> for IPAddrv6 {
    #[inline(always)]
    fn from(addr: Ipv6Addr) -> Self {
        [addr.0, addr.1, addr.2, addr.3, addr.4, addr.5, addr.6, addr.7]
    }
}

impl PartialEq<IPAddrv6> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &IPAddrv6) -> bool {
        *self == Ipv6Addr::from(*other)
    }
}

impl core::fmt::Display for Ipv6Addr {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            fmt,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7
        )
    }
}

impl core::fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self, fmt)
    }
}


/// Represents an error which occurred whilst parsing a MAC address.
#[derive( Copy,  PartialEq,  Eq,  Clone, fmt::Debug)]
pub enum ParseIpv6AddrErr {
    /// The IP address has too many components, eg. 192.168.1.1.1
    TooManyComponents,
    /// The IP address has too few components, eg. 192.168
    TooFewComponents,
    /// One of the components contains an invalid value, eg. 192.168.2x.22
    InvalidComponent,
}

// Error trait is only support via std 
#[cfg(feature = "std")]
impl Error for ParseIpv6AddrErr {}


impl ParseIpv6AddrErr {
    fn description(&self) -> &str {
        match *self {
            ParseIpv6AddrErr::TooManyComponents => "Too many components in a MAC address string",
            ParseIpv6AddrErr::TooFewComponents => "Too few components in a MAC address string",
            ParseIpv6AddrErr::InvalidComponent => "Invalid component in a MAC address string",
        }
    }
}

impl fmt::Display for ParseIpv6AddrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


impl FromStr for Ipv6Addr {
    type Err = ParseIpv6AddrErr;
    #[inline]
    fn from_str(s: &str) -> Result<Ipv6Addr, ParseIpv6AddrErr> {
        let mut parts = [0u16; 8];
        let splits = s.split('.');
        let mut i = 0;
        for split in splits {
            if i == 8 {
                return Err(ParseIpv6AddrErr::TooManyComponents);
            }
            match u16::from_str_radix(split, 64) {
                Ok(b) if split.len() != 0 => parts[i] = b,
                _ => return Err(ParseIpv6AddrErr::InvalidComponent),
            }
            i += 1;
        }

        if i == 4 {
            Ok(Ipv6Addr(
                parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6], parts[7]
            ))
        } else {
            Err(ParseIpv6AddrErr::TooFewComponents)
        }
    }
}