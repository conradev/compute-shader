// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code,
         non_snake_case,
         non_camel_case_types,
         non_upper_case_globals)]

use cocoa::base::{class, id};

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MTLLanguageVersion {
    v1_0 = (1 << 16),
    v1_1 = (1 << 16) + 1,
    v1_2 = (1 << 16) + 2,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MTLCPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MTLStorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
    Memoryless = 3,
}

bitflags! {
    pub flags MTLResourceOptions: usize {
        const MTLResourceCPUCacheModeDefaultCache = MTLCPUCacheMode::DefaultCache as usize,
        const MTLResourceCPUCacheModeWriteCombined = MTLCPUCacheMode::WriteCombined as usize,

        const MTLResourceStorageModeShared = (MTLStorageMode::Shared as usize) << 4,
        const MTLResourceStorageModeManaged = (MTLStorageMode::Managed as usize)  << 4,
        const MTLResourceStorageModePrivate = (MTLStorageMode::Private as usize) << 4,
        const MTLResourceStorageModeMemoryless = (MTLStorageMode::Memoryless as usize) << 4,
    }
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    pub fn MTLCreateSystemDefaultDevice() -> id;
}

pub trait MTLDevice {
    unsafe fn newLibraryWithSource(self, source: id, options: id, error: *mut id) -> id;
    unsafe fn newComputePipelineStateWithFunction(self, function: id, error: *mut id) -> id;

    unsafe fn newCommandQueue(self) -> id;

    unsafe fn newBufferWithBytes(self, bytes: *const u8, length: usize, options: MTLResourceOptions) -> id;
    unsafe fn newBufferWithLength(self, length: usize, options: MTLResourceOptions) -> id;
}

impl MTLDevice for id {
    unsafe fn newLibraryWithSource(self, source: id, options: id, error: *mut id) -> id {
        msg_send![self, newLibraryWithSource:source options:options error:error]
    }

    unsafe fn newComputePipelineStateWithFunction(self, function: id, error: *mut id) -> id {
        msg_send![self, newComputePipelineStateWithFunction:function error:error]
    }

    unsafe fn newCommandQueue(self) -> id {
        msg_send![self, newCommandQueue]
    }

    unsafe fn newBufferWithBytes(self, bytes: *const u8, length: usize, options: MTLResourceOptions) -> id {
        msg_send![self, newBufferWithBytes:bytes length:length options:options]
    }

    unsafe fn newBufferWithLength(self, length: usize, options: MTLResourceOptions) -> id {
        msg_send![self, newBufferWithLength:length options:options]
    }
}

pub trait MTLCommandQueue {
    unsafe fn device(self) -> id;

    unsafe fn commandBuffer(self) -> id;
}

impl MTLCommandQueue for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn commandBuffer(self) -> id {
        msg_send![self, commandBuffer]
    }
}

pub trait MTLCommandBuffer {
    unsafe fn computeCommandEncoder(self) -> id;

    unsafe fn commit(self);
}

impl MTLCommandBuffer for id {
    unsafe fn computeCommandEncoder(self) -> id {
        msg_send![self, computeCommandEncoder]
    }

    unsafe fn commit(self) {
        msg_send![self, commit];
    }
}

pub trait MTLComputeCommandEncoder {
    unsafe fn setComputePipelineState(self, state: id);
    unsafe fn setBuffer(self, buffer: id, offset: usize, index: usize);
    unsafe fn endEncoding(self);
}

impl MTLComputeCommandEncoder for id {
    unsafe fn setComputePipelineState(self, state: id) {
        msg_send![self, setComputePipelineState:state];
    }

    unsafe fn setBuffer(self, buffer: id, offset: usize, index: usize) {
        msg_send![self, setBuffer:buffer offset:offset atIndex:index];
    }

    unsafe fn endEncoding(self) {
        msg_send![self, endEncoding];
    }
}

pub trait MTLLibrary {
    unsafe fn functionNames(self) -> id;
    unsafe fn newFunctionWithName(self, name: id) -> id;
}

impl MTLLibrary for id {
    unsafe fn functionNames(self) -> id {
        msg_send![self, functionNames]
    }

    unsafe fn newFunctionWithName(self, name: id) -> id {
        msg_send![self, newFunctionWithName:name]
    }
}

pub trait MTLFunction {
    unsafe fn functionType(self) -> MTLFunctionType;
}

impl MTLFunction for id {
    unsafe fn functionType(self) -> MTLFunctionType {
        msg_send![self, functionType]
    }
}

pub trait MTLCompileOptions {
    unsafe fn new(_: Self) -> id {
        msg_send![class("MTLCompileOptions"), new]
    }

    unsafe fn setLanguageVersion(self, other: MTLLanguageVersion);
}

impl MTLCompileOptions for id {
    unsafe fn setLanguageVersion(self, other: MTLLanguageVersion) {
        msg_send![self, setLanguageVersion:other];
    }
}
