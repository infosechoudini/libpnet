// Copyright (c) 2014-2016 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
extern crate serde;

mod macaddr;
mod ip4addr;
mod ip6addr;
pub use crate::macaddr::*;
pub use crate::ip4addr::*;
pub use crate::ip6addr::*;