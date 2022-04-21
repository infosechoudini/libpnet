
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