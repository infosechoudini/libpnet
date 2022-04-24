// Copyright (c) 2014, 2015, 2017 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Packet helpers for `pnet_macros`.

// * External Imports
// **  Using core vs std allows for conversion to no_std for the future
use core::ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo};

// * Internal Imports
use pnet_base;

/// Represents a generic network packet.
pub trait Packet {
    /// Retrieve the underlying buffer for the packet.
    fn packet(&self) -> &[u8];

    /// Retrieve the payload for the packet.
    fn payload(&self) -> &[u8];

}

/// Represents a generic, mutable, network packet.
pub trait MutablePacket: Packet {
    /// Retrieve the underlying, mutable, buffer for the packet.
    fn packet_mut(&mut self) -> &mut [u8];

    /// Retrieve the mutable payload for the packet.
    fn payload_mut(&mut self) -> &mut [u8];

    /// Initialize this packet by cloning another.
    fn clone_from<T: Packet>(&mut self, other: &T) {
        use core::ptr;

        assert!(self.packet().len() >= other.packet().len());
        unsafe {
            ptr::copy_nonoverlapping(
                other.packet().as_ptr(),
                self.packet_mut().as_mut_ptr(),
                other.packet().len(),
            );
        }
    }
}

/// Used to convert on-the-wire packets to their #\[packet\] equivalent.
pub trait FromPacket: Packet {
    /// The type of the packet to convert from.
    type T;

    /// Converts a wire-format packet to #\[packet\] struct format.
    fn from_packet(&self) -> Self::T;
}

/// Used to find the calculated size of the packet. This is used for occasions where the underlying
/// buffer is not the same length as the packet itself.
pub trait PacketSize: Packet {
    /// Get the calculated size of the packet.
    fn packet_size(&self) -> usize;
}


// Allows indexing of a packet.
macro_rules! impl_index {
    ($t:ident, $index_t:ty, $output_t:ty) => {
        impl<'p> Index<$index_t> for $t<'p> {
            type Output = $output_t;
            #[inline]
            fn index(&self, index: $index_t) -> &$output_t {
                &self.as_slice().index(index)
            }
        }
    };
}

// Allows indexing of a mutable packet.
macro_rules! impl_index_mut {
    ($t:ident, $index_t:ty, $output_t:ty) => {
        impl<'p> IndexMut<$index_t> for $t<'p> {
            #[inline]
            fn index_mut(&mut self, index: $index_t) -> &mut $output_t {
                self.as_mut_slice().index_mut(index)
            }
        }
    };
}

/// Packet data.
#[derive(PartialEq)]
pub enum PacketData<'p> {
    /// A packet owns its contents.
    Owned(Vec<u8>),
    /// A packet borrows its contents.
    Borrowed(&'p [u8]),
}

/// Implementations of `PacketData` for `Packet`
impl<'p> PacketData<'p> {

    /// Get a slice of the packet data.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            &PacketData::Owned(ref data) => data.deref(),
            &PacketData::Borrowed(ref data) => data,
        }
    }

    /// No-op - returns `self`.
    #[inline]
    pub fn to_immutable(self) -> PacketData<'p> {
        self
    }

    /// A length of the packet data.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

}

impl_index!(PacketData, usize, u8);
impl_index!(PacketData, Range<usize>, [u8]);
impl_index!(PacketData, RangeTo<usize>, [u8]);
impl_index!(PacketData, RangeFrom<usize>, [u8]);
impl_index!(PacketData, RangeFull, [u8]);

/// Mutable packet data.
#[derive(PartialEq)]
pub enum MutPacketData<'p> {
    /// Owned mutable packet data.
    Owned(Vec<u8>),
    /// Borrowed mutable packet data.
    Borrowed(&'p mut [u8]),
}

/// Implementations of `MutPacketData` for `MutablePacket`
impl<'p> MutPacketData<'p> {
    /// Get packet data as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            &MutPacketData::Owned(ref data) => data.deref(),
            &MutPacketData::Borrowed(ref data) => data,
        }
    }

    /// Get packet data as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            &mut MutPacketData::Owned(ref mut data) => data.deref_mut(),
            &mut MutPacketData::Borrowed(ref mut data) => data,
        }
    }

    /// Get an immutable version of packet data.
    #[inline]
    pub fn to_immutable(self) -> PacketData<'p> {
        match self {
            MutPacketData::Owned(data) => PacketData::Owned(data),
            MutPacketData::Borrowed(data) => PacketData::Borrowed(data),
        }
    }

    /// Get a length of data in the packet.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl_index!(MutPacketData, usize, u8);
impl_index!(MutPacketData, Range<usize>, [u8]);
impl_index!(MutPacketData, RangeTo<usize>, [u8]);
impl_index!(MutPacketData, RangeFrom<usize>, [u8]);
impl_index!(MutPacketData, RangeFull, [u8]);

impl_index_mut!(MutPacketData, usize, u8);
impl_index_mut!(MutPacketData, Range<usize>, [u8]);
impl_index_mut!(MutPacketData, RangeTo<usize>, [u8]);
impl_index_mut!(MutPacketData, RangeFrom<usize>, [u8]);
impl_index_mut!(MutPacketData, RangeFull, [u8]);

/// Used to convert a type to primitive values representing it.
pub trait PrimitiveValues {
    /// A tuple of types, to represent the current value.
    type T;

    /// Convert a value to primitive types representing it.
    fn to_primitive_values(&self) -> Self::T;
}

// Used to convert MacAddr to a tuple of u8s
impl PrimitiveValues for pnet_base::MacAddr {
    type T = (u8, u8, u8, u8, u8, u8);
    #[inline]
    fn to_primitive_values(&self) -> (u8, u8, u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

// Used to convert IpAddrV4 to a tuple of u8s
impl PrimitiveValues for ::std::net::Ipv4Addr {
    type T = (u8, u8, u8, u8);
    #[inline]
    fn to_primitive_values(&self) -> (u8, u8, u8, u8) {

        // octets() is a method of Ipv4Addr that 
        // returns a slice of u8s representing the address.
        let octets = self.octets();

        (octets[0], octets[1], octets[2], octets[3])
    }
}

// Used to convert IpAddrV6 to a tuple of u8s
impl PrimitiveValues for ::std::net::Ipv6Addr {
    type T = (u16, u16, u16, u16, u16, u16, u16, u16);
    #[inline]
    fn to_primitive_values(&self) -> (u16, u16, u16, u16, u16, u16, u16, u16) {

        // segments() is a method of std::net::Ipv6Addr 
        // returns a slice of u16s representing the address.
        let segments = self.segments();

        (
            segments[0],
            segments[1],
            segments[2],
            segments[3],
            segments[4],
            segments[5],
            segments[6],
            segments[7],
        )
    }
}
