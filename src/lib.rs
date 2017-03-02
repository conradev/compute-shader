// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A cross-platform interface to a subset of GPU compute functionality.

extern crate euclid;
extern crate gl;
extern crate libc;

#[macro_use]
#[cfg(target_os = "macos")]
extern crate bitflags;
#[cfg(target_os = "macos")]
extern crate byteorder;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate core_foundation;
#[cfg(target_os = "macos")]
extern crate io_surface;
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod api {
    #[cfg(target_os = "macos")]
    pub mod cl;
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub mod gl;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub mod metal;
}

pub mod buffer;
pub mod device;
pub mod error;
pub mod image;
pub mod instance;
pub mod profile_event;
pub mod program;
pub mod queue;
pub mod sync_event;

