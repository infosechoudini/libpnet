
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