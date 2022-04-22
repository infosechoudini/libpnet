
use crate::ip4addr::Ipv4Addr;
use crate::ip6addr::Ipv6Addr;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


impl IpAddr {
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }

    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, IpAddr::V6(_))
    }
}


impl core::fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            fmt,
            "{:?}",
            self
        )
    }
}